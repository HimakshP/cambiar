use std::fs::File;
use std::path::Path;

use csv::Reader;
use serde_json::{Map, Value};

use crate::engine::converter::Converter;
use crate::errors::CambiarErrors;
use crate::formats::FileFormats;

pub struct CsvToJson;

impl Converter for CsvToJson {
    fn input_format(&self) -> FileFormats {
        FileFormats::Csv
    }

    fn output_format(&self) -> FileFormats {
        FileFormats::Json
    }

    fn convert(&self, input: &Path, output: &Path) -> Result<(), CambiarErrors> {
        let infile = File::open(input).map_err(|_| CambiarErrors::ReadError)?;

        let mut rdr = Reader::from_reader(infile);

        let headers = rdr.headers().map_err(|_| CambiarErrors::ReadError)?.clone();

        let mut records = Vec::new();

        for result in rdr.records() {
            let record = result.map_err(|_| CambiarErrors::ReadError)?;

            let mut obj = Map::new();

            for (key, value) in headers.iter().zip(record.iter()) {
                obj.insert(key.to_string(), Value::String(value.to_string()));
            }
            records.push(Value::Object(obj));
        }

        let outfile = File::create(output).map_err(|_| CambiarErrors::WriteError)?;

        serde_json::to_writer_pretty(outfile, &records).map_err(|_| CambiarErrors::WriteError)?;

        Ok(())
    }
}
