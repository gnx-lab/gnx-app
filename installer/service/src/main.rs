#![allow(unused_must_use)]

use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    process::Command,
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

const SERVICE_NAME: &str = "GnxMeshMonitor";
const STATUS_CONTENT_TYPE: &str = "application/json; charset=utf-8";

type SharedStatus = Arc<Mutex<Status>>;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ConfigFile {
    monitor: MonitorConfig,
    #[serde(default)]
    provisioning: ProvisioningConfig,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MonitorConfig {
    app_url: String,
    interval_seconds: u64,
    request_timeout_seconds: u64,
    listen_port: u16,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ProvisioningConfig {
    enabled: bool,
    interval_seconds: u64,
    auto_reboot: bool,
    state_path: String,
}

impl Default for ProvisioningConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_seconds: 120,
            auto_reboot: true,
            state_path: r"C:\ProgramData\GnX Mesh\provisioning.json".into(),
        }
    }
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProvisioningState {
    phase: String,
    code: String,
    message: String,
    reboot_pending: bool,
    service_user: String,
    distribution: String,
    linux_service: String,
    last_attempt: Option<String>,
    last_error: Option<String>,
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
    provisioning_phase: String,
    provisioning_code: String,
    provisioning_message: String,
    provisioning_reboot_pending: bool,
    dedicated_user: String,
    wsl_distribution: String,
    linux_service: String,
    provisioning_checked_at: u64,
    provisioning_last_attempt: Option<String>,
    provisioning_last_error: Option<String>,
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

fn config() -> ConfigFile {
    let path = env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("gnx-mesh-monitor.exe"))
        .with_file_name("appsettings.json");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    serde_json::from_str::<ConfigFile>(text.trim_start_matches('\u{feff}'))
        .unwrap_or_else(|error| panic!("cannot parse {}: {error}", path.display()))
}

fn run(settings: ConfigFile, stop: Arc<AtomicBool>) {
    let monitor = settings.monitor;
    let provisioning = settings.provisioning;
    let state = Arc::new(Mutex::new(Status {
        service_running: true,
        app_url: monitor.app_url.clone(),
        provisioning_phase: "NOT_STARTED".into(),
        provisioning_code: "NOT_STARTED".into(),
        provisioning_message: "The WSL/Quadlet reconciler has not run yet.".into(),
        dedicated_user: "gnxmeshsvc".into(),
        wsl_distribution: "gnx-mesh".into(),
        linux_service: "unknown".into(),
        ..Status::default()
    }));
    let server_state = Arc::clone(&state);
    let server_stop = Arc::clone(&stop);
    let port = monitor.listen_port;
    let server_thread = thread::spawn(move || run_status_server(port, server_state, server_stop));

    let provisioning_state = Arc::clone(&state);
    let provisioning_stop = Arc::clone(&stop);
    thread::spawn(move || run_provisioner(provisioning, provisioning_state, provisioning_stop));

    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(monitor.request_timeout_seconds.max(1)))
        .timeout_read(Duration::from_secs(monitor.request_timeout_seconds.max(1)))
        .timeout_write(Duration::from_secs(monitor.request_timeout_seconds.max(1)))
        .build();

    while !stop.load(Ordering::SeqCst) {
        let base = monitor.app_url.trim_end_matches('/');
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

        for _ in 0..monitor.interval_seconds.max(10) {
            if stop.load(Ordering::SeqCst) {
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    let _ = server_thread.join();
}

fn run_provisioner(settings: ProvisioningConfig, state: SharedStatus, stop: Arc<AtomicBool>) {
    if !settings.enabled {
        update_provisioning_status(
            &state,
            ProvisioningState {
                phase: "DISABLED".into(),
                code: "DISABLED".into(),
                message: "WSL/Quadlet provisioning is disabled by configuration.".into(),
                ..ProvisioningState::default()
            },
        );
        return;
    }

    while !stop.load(Ordering::SeqCst) {
        reconcile_wsl(&settings, &state);
        for _ in 0..settings.interval_seconds.max(30) {
            if stop.load(Ordering::SeqCst) {
                return;
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn reconcile_wsl(settings: &ProvisioningConfig, state: &SharedStatus) {
    let script = env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("gnx-mesh-monitor.exe"))
        .with_file_name("provision-wsl.ps1");
    if !script.exists() {
        update_provisioning_status(
            state,
            ProvisioningState {
                phase: "BLOCKED".into(),
                code: "PROVISIONER_MISSING".into(),
                message: format!("Missing provisioning script: {}", script.display()),
                ..ProvisioningState::default()
            },
        );
        return;
    }

    let mut command = Command::new("powershell.exe");
    command
        .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-File"])
        .arg(&script)
        .args(["-Mode", "Reconcile", "-StatePath"])
        .arg(&settings.state_path);
    if settings.auto_reboot {
        command.arg("-AutoReboot");
    }
    let result = command.output();
    let mut provisioned = fs::read_to_string(&settings.state_path)
        .ok()
        .and_then(|text| serde_json::from_str::<ProvisioningState>(&text).ok())
        .unwrap_or_default();

    if provisioned.phase.is_empty() {
        provisioned.phase = "BLOCKED".into();
        provisioned.code = "PROVISIONER_NO_STATE".into();
        provisioned.message = match result {
            Ok(output) => format!("Provisioner exited {} without valid state: {}", output.status, String::from_utf8_lossy(&output.stderr).trim()),
            Err(error) => format!("Could not start the provisioning command: {error}"),
        };
    }
    update_provisioning_status(state, provisioned);
}

fn update_provisioning_status(state: &SharedStatus, provisioned: ProvisioningState) {
    if let Ok(mut current) = state.lock() {
        current.provisioning_phase = provisioned.phase;
        current.provisioning_code = provisioned.code;
        current.provisioning_message = provisioned.message;
        current.provisioning_reboot_pending = provisioned.reboot_pending;
        if !provisioned.service_user.is_empty() {
            current.dedicated_user = provisioned.service_user;
        }
        if !provisioned.distribution.is_empty() {
            current.wsl_distribution = provisioned.distribution;
        }
        current.linux_service = provisioned.linux_service;
        current.provisioning_last_attempt = provisioned.last_attempt;
        current.provisioning_last_error = provisioned.last_error;
        current.provisioning_checked_at = now();
    }
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
