use std::{fs::File, io::BufReader};

#[macro_use]
extern crate log;

mod cli_parsing;
mod evaluator;
mod parser;

fn main() {
    let args = cli_parsing::parse_arguments();
    env_logger::init();
    info!("Logger initialized");
    let fs = File::open(&args.files[0]).unwrap();
    let mut reader = BufReader::new(fs);
    let parsed = parser::parse_file(&mut reader, args.files[0].to_str().unwrap().to_string());
    let defs = parsed.unwrap().definitions;
    let ycomb = defs.iter().find(|x| x.name == "ycomb").unwrap();
    let converted = evaluator::convert(&ycomb.value, 0);
    println!("{:#?}", converted);
}
