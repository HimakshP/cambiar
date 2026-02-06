use clap::Parser;
use std::path::PathBuf;

#[derive(Parser,Debug)]
#[command(
    name = "cambiar",
    version,
    about = "Rust based cli file converter tool"
)]
pub struct Cli{

    ///input file path
    pub input: PathBuf,

    ///output file path 
    pub output: PathBuf,

    #[arg(short, long)]
    pub force: bool,
}

pub fn parse() -> Cli {
    Cli::parse()
}