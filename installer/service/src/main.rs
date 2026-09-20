#![allow(unused_must_use)]

use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tiny_http::{Header, Method, Response, Server};
use ureq::Agent;
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
};

const SERVICE_NAME: &str = "GnxAppMonitor";
const STATUS_CONTENT_TYPE: &str = "application/json; charset=utf-8";

type SharedStatus = Arc<Mutex<Status>>;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ConfigFile {
    monitor: MonitorConfig,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MonitorConfig {
    app_url: String,
    interval_seconds: u64,
    request_timeout_seconds: u64,
    listen_port: u16,
}

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct Status {
    service_running: bool,
    url_available: bool,
    manifest_available: bool,
    service_worker_available: bool,
    app_url: String,
    checked_at: u64,
}

fn main() {
    if env::args().any(|arg| arg == "--console") {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_for_ctrlc = Arc::clone(&stop);
        ctrlc::set_handler(move || stop_for_ctrlc.store(true, Ordering::SeqCst))
            .expect("could not register Ctrl+C handler");
        run(config(), stop);
        return;
    }

    if let Err(error) = service_dispatcher::start(SERVICE_NAME, ffi_service_main) {
        eprintln!("service dispatcher failed: {error}");
        std::process::exit(1);
    }
}

define_windows_service!(ffi_service_main, service_main);

fn service_main(_arguments: Vec<std::ffi::OsString>) -> Result<(), windows_service::Error> {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_for_handler = Arc::clone(&stop);
    let handler = move |event| match event {
        ServiceControl::Stop | ServiceControl::Shutdown => {
            stop_for_handler.store(true, Ordering::SeqCst);
            ServiceControlHandlerResult::NoError
        }
        ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
        _ => ServiceControlHandlerResult::NotImplemented,
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, handler)?;
    status_handle.set_service_status(running_status())?;
    run(config(), stop);
    status_handle.set_service_status(stopped_status())?;
    Ok(())
}

fn running_status() -> ServiceStatus {
    ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    }
}

fn stopped_status() -> ServiceStatus {
    ServiceStatus {
        current_state: ServiceState::Stopped,
        ..running_status()
    }
}

fn config() -> MonitorConfig {
    let path = env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("GnxAppMonitor.exe"))
        .with_file_name("appsettings.json");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    serde_json::from_str::<ConfigFile>(text.trim_start_matches('\u{feff}'))
        .unwrap_or_else(|error| panic!("cannot parse {}: {error}", path.display()))
        .monitor
}

fn run(settings: MonitorConfig, stop: Arc<AtomicBool>) {
    let state = Arc::new(Mutex::new(Status {
        service_running: true,
        app_url: settings.app_url.clone(),
        ..Status::default()
    }));
    let server_state = Arc::clone(&state);
    let server_stop = Arc::clone(&stop);
    let port = settings.listen_port;
    let server_thread = thread::spawn(move || run_status_server(port, server_state, server_stop));

    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(settings.request_timeout_seconds.max(1)))
        .timeout_read(Duration::from_secs(settings.request_timeout_seconds.max(1)))
        .timeout_write(Duration::from_secs(settings.request_timeout_seconds.max(1)))
        .build();

    while !stop.load(Ordering::SeqCst) {
        let base = settings.app_url.trim_end_matches('/');
        let app_url_available = check(&agent, base);
        let manifest = check(&agent, &format!("{base}/manifest.webmanifest"));
        let worker = check(&agent, &format!("{base}/sw.js"));
        if let Ok(mut current) = state.lock() {
            current.url_available = app_url_available;
            current.manifest_available = manifest;
            current.service_worker_available = worker;
            current.checked_at = now();
        }
        println!("url={app_url_available} manifest={manifest} service_worker={worker}");

        for _ in 0..settings.interval_seconds.max(10) {
            if stop.load(Ordering::SeqCst) {
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    let _ = server_thread.join();
}

fn check(agent: &Agent, url: &str) -> bool {
    agent
        .get(url)
        .call()
        .map(|response| response.status() < 400)
        .unwrap_or(false)
}

fn run_status_server(port: u16, state: SharedStatus, stop: Arc<AtomicBool>) {
    let server = Server::http(format!("127.0.0.1:{port}"))
        .unwrap_or_else(|error| panic!("cannot bind local status endpoint: {error}"));
    while !stop.load(Ordering::SeqCst) {
        let request = match server.recv_timeout(Duration::from_secs(1)) {
            Ok(Some(request)) => request,
            Ok(None) => continue,
            Err(_) => break,
        };
        let origin = request
            .headers()
            .iter()
            .find(|header| header.field.equiv("Origin"))
            .map(|header| header.value.as_str().to_string());
        let allowed_origin = origin.filter(|value| {
            value.starts_with("http://localhost:") || value.starts_with("http://127.0.0.1:")
        });
        let mut response = if request.method() == &Method::Get
            && request.url().split('?').next() == Some("/status")
        {
            let body = state
                .lock()
                .map(|value| serde_json::to_string(&*value).unwrap())
                .unwrap_or_else(|_| "{}".into());
            Response::from_string(body).with_status_code(200)
        } else {
            Response::from_string("").with_status_code(404)
        };
        if let Ok(header) = Header::from_bytes(b"Content-Type", STATUS_CONTENT_TYPE.as_bytes()) {
            response = response.with_header(header);
        }
        if let Some(origin) = allowed_origin {
            if let Ok(header) =
                Header::from_bytes(b"Access-Control-Allow-Origin", origin.as_bytes())
            {
                response = response.with_header(header);
            }
        }
        let _ = request.respond(response);
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
