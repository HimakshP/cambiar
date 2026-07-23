use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "cambiar",
    version,
    about = "A fast, lightweight CLI file converter written in Rust",
    long_about = "Cambiar is a command-line file conversion tool written in Rust.\n\
                  It supports conversion between structured data, and document."
)]
pub struct Cli {
    ///input file path
    pub input: Option<PathBuf>,

    ///output file path
    pub output: Option<PathBuf>,

    #[arg(short, long)]
    pub force: bool,

    /// List all supported file conversions
    #[arg(long)]
    pub list_formats: bool,
}

pub fn parse() -> Cli {
    Cli::parse()
}
