mod control_pipe;
mod provisioning;
mod state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    control_pipe::run()
}
