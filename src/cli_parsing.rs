use clap::Parser;
use log::Level;

#[derive(Parser)]
#[clap(author = "Et-17 <leland.kilborn@hotmail.com")]
#[clap(about = "A fast lambda calculus interpreter designed for large codebases")]
pub struct Cli {
    #[clap(help = "The files to process")]
    #[clap(required = true)]
    #[clap(parse(from_os_str))]
    pub files: Vec<std::path::PathBuf>,
    #[clap(help = "Verbosity level")]
    #[clap(short = 'v')]
    #[clap(long = "verbosity")]
    #[clap(default_value = "0")]
    #[clap(value_parser = clap::value_parser!(u8).range(0..=4))]
    #[clap(action = clap::ArgAction::Count)]
    pub verbosity: u8,
}

pub fn parse_arguments() -> Cli {
    let args: Cli = Cli::parse();
    std::env::set_var(
        "RUST_LOG",
        Level::iter().nth(args.verbosity.into()).unwrap().as_str(),
    );
    return args;
}
