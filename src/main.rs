use std::{fs::File, io::BufReader};

#[macro_use]
extern crate log;

mod cli_parsing;

fn main() {
    let args = cli_parsing::parse_arguments();
    env_logger::init();
    info!("Logger initialized");
    let fs = File::open(&args.files[0]).unwrap();
    let mut reader = BufReader::new(fs);
}
