#[macro_use]
extern crate log;

mod cli_parsing;

fn main() {
    let args = cli_parsing::parse_arguments();
    env_logger::init();
    info!("Starting up");
    info!("Passed lambda files:");
    for ele in args.files {
        info!("  {:?}", ele);
    }
    error!("Error message");
    warn!("Warning message");
    info!("Information message");
    debug!("Debug message");
    trace!("Trace message");
}
