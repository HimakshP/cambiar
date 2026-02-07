use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use crate::errors::CambiarErrors;

const BUF_SIZE: usize = 8 * 1024; // 8kb

pub fn copy_file(input: &Path, output: &Path) -> Result<(), CambiarErrors> {

    let infile = File::open(input)
    .map_err(|_| {
        CambiarErrors::InputNotFound
    })?;

    let outfile = File::create(output)
    .map_err(|_| {
        CambiarErrors::OutputExists
    })?;

    let mut reader = BufReader::new(infile);   // maintains an in-memory buffer of the reads
    let mut writer = BufWriter::new(outfile);  // wraps the writer and buffers its output

    let mut buffer = [0u8; BUF_SIZE];


    loop {
        let bytes_read = reader
        .read(&mut buffer)                    // pulling bytes from reader into buffer array
        .map_err(|_| CambiarErrors::ReadError)?;

    if bytes_read == 0 {
        break;
    }

    writer
    .write_all(&buffer[..bytes_read])            // calls write repeatedly until no data left or error ; writes till bytes_read in buffer array
    .map_err(|_| CambiarErrors::WriteError)?;

    }

    
    writer.flush().ok();                                               // all intermediately buffered contents reach their destination
    Ok(())
}