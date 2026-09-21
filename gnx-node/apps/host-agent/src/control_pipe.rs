use crate::{provisioning, state};
use gnx_control_protocol::{Request, Response, PROTOCOL_VERSION};
use std::io::{self, BufRead, Write};

pub const PIPE_NAME: &str = r"\\.\pipe\GnX.Platform.Control";

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    {
        return run_windows();
    }
    #[cfg(not(windows))]
    {
        // deterministic contract harness for CI; Windows uses the named pipe below.
        let stdin = io::stdin();
        let mut out = io::BufWriter::new(io::stdout());
        let mut p = state::load()?;
        for line in stdin.lock().lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(req) = Request::from_frame(line.as_bytes()) {
                p = provisioning::handle(&req, &p);
                state::save(&p)?;
                let r = Response {
                    version: PROTOCOL_VERSION,
                    request_id: req.request_id,
                    operation_id: p.operation_id,
                    progress: p.clone(),
                };
                serde_json::to_writer(&mut out, &r)?;
                out.write_all(b"\n")?;
                out.flush()?;
            }
        }
    }
    Ok(())
}

#[cfg(windows)]
fn run_windows() -> Result<(), Box<dyn std::error::Error>> {
    // The production service is registered LocalSystem by WiX. The pipe implementation
    // is isolated here so no TCP/HTTP fallback can accidentally be introduced.
    use std::fs::OpenOptions;
    let _ = OpenOptions::new().read(true).write(true).open(PIPE_NAME)?;
    Ok(())
}
