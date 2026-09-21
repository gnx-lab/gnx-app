mod agent_client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = locate_ui();
    if !ui.exists() {
        return Err(format!("Setup UI asset missing: {}", ui.display()).into());
    }
    #[cfg(windows)]
    launch_local_ui(&ui)?;
    Ok(())
}

fn locate_ui() -> std::path::PathBuf {
    if let Ok(path) = std::env::var("GNX_SETUP_UI") {
        return path.into();
    }
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join(r"ui\index.html")))
        .unwrap_or_else(|| std::path::PathBuf::from("apps/setup/ui/index.html"))
}

#[cfg(windows)]
fn launch_local_ui(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::UI::Shell::ShellExecuteW;
    let operation: Vec<u16> = std::ffi::OsStr::new("open")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let file: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let result = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            1,
        )
    };
    if result as isize <= 32 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(())
}
