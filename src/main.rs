mod cli;
mod engine;
mod errors;
mod formats;

use cli::parse;
use engine::converter::Converter;
use engine::csv_json::CsvToJson;
use engine::png_jpg::PngJpg;
use crate::{engine::md_txt::MdtoTxt, errors::CambiarErrors, formats::FileFormats};

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
        std::process::exit(1);
    }

    pub fn run() -> Result<(), CambiarErrors> {
        let args = parse();
        if args.list_formats {
            println!("Supported conversions:\n");
            println!("  csv -> json");
            println!("  md  -> txt");

            return Ok(());
        }
        let input = args.input.ok_or_else(|| CambiarErrors::MissingInput)?;

        let output = args.output.ok_or_else(|| CambiarErrors::MissingOutput)?;
        let csv_to_json = CsvToJson;
        let md_to_txt = MdtoTxt;
        let png_to_jpg = PngJpg;

        if !input.exists() {
            return Err(CambiarErrors::InputNotFound);
        }

        if output.exists() && !args.force {
            return Err(CambiarErrors::OutputExists);
        }

        let input_fmt = FileFormats::from_path(&input).ok_or_else(|| {
            // a closure containing None and returns following error
            CambiarErrors::UnsupportedFormat(
                // building the error
                input
                    .extension() // returns &OsString type
                    .and_then(|s| s.to_str()) // returns None if the option is None
                    .unwrap_or("unknown") // returns unknown when argument in None
                    .to_string(),
            )
        })?;

        let output_fmt = FileFormats::from_path(&output).ok_or_else(|| {
            CambiarErrors::UnsupportedFormat(
                output
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
            )
        })?;

        println!("Detected formats:");
        println!("  Input:  {}", input_fmt);
        println!("  Output: {}", output_fmt);

        // copy_file(&args.input, &args.output)?; // this is temporary copy functionality
        match (input_fmt, output_fmt) {
            (FileFormats::Csv, FileFormats::Json) => {
                csv_to_json.convert(&input, &output)?;
            }

            (FileFormats::Md, FileFormats::Txt) => {
                md_to_txt.convert(&input, &output)?;
            }

            (FileFormats::Png, FileFormats::Jpg) => {
                png_to_jpg.convert(&input, &output)?;
            }

            _ => {
                return Err(CambiarErrors::UnsupportedConversion(input_fmt, output_fmt));
            }
        }
        println!(
            "✓ Converted {} → {}\n  Output: {}",
            input_fmt,
            output_fmt,
            output.display()
        );

        Ok(())
    }
}
