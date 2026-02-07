use thiserror::Error;

#[derive(Error, Debug)]
pub enum CambiarErrors {

    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),

    #[error("File already exists (use --force to overwrite)")]
    OutputExists,

    #[error("Input file not found")]
    InputNotFound,

    #[error("Failed to read input file")]
    ReadError,

    #[error("Failed to write output file")]
    WriteError,
}
