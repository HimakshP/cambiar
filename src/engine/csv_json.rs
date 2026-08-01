use std::fs::File;
use std::path::Path;

use csv::Reader;
use serde::ser::{SerializeSeq, Serializer};
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

        let outfile = File::create(output).map_err(|_| CambiarErrors::WriteError)?;

        let mut serializer = serde_json::Serializer::pretty(outfile);

        let mut seq = serializer
            .serialize_seq(None)
            .map_err(|_| CambiarErrors::WriteError)?;

        for result in rdr.records() {
            let record = result.map_err(|_| CambiarErrors::ReadError)?;

            let mut obj = Map::new();

            for (key, value) in headers.iter().zip(record.iter()) {
                obj.insert(key.to_string(), Value::String(value.to_string()));
            }

            let _ = seq.serialize_element(&obj);
        }

        let _ = seq.end();

        Ok(())
    }
}
