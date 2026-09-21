use gnx_control_protocol::PROTOCOL_VERSION;
pub fn pipe_name() -> &'static str {
    r"\.pipeGnX.Platform.Control"
}
pub fn protocol_version() -> u16 {
    PROTOCOL_VERSION
}
