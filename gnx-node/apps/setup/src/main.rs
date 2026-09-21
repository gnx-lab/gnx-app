//! Setup shell. The packaged UI is local-only; WebView2 integration is intentionally
//! optional at compile time so the protocol and service can be built on CI hosts.
mod agent_client;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("GnX Node Setup — local WebView2 UI assets: ui/index.html");
    let _ = agent_client::pipe_name();
    Ok(())
}
