use clap::Parser;

#[derive(Parser)]
#[clap(author = "Et-17 <leland.kilborn@hotmail.com")]
#[clap(about = "A fast lambda calculus interpreter designed for large codebases")]
pub struct Cli {
    #[clap(help = "The files to process")]
    #[clap(required = true)]
    #[clap(parse(from_os_str))]
    pub files: Vec<std::path::PathBuf>,
}

pub fn parse_arguments() -> Cli {
    Cli::parse()
}
