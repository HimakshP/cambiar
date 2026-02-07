mod cli;
mod formats;
mod errors;

use cli::parse;

use crate::{errors::CambiarErrors, formats::FileFormats};

fn main() {

    if let Err(e) = run() {
        println!("{}", e);
        std::process::exit(1);
    }

    pub fn run() -> Result<(), CambiarErrors>{

        let args = parse();

        if !args.input.exists() {
        return Err(CambiarErrors::InputNotFound);
    }

    
    if args.output.exists() && !args.force {
        return Err(CambiarErrors::OutputExists);
    }

    let input_fmt = FileFormats::from_path(&args.input)
    .ok_or_else(|| {                                       // a closure containing None and returns following error
        CambiarErrors::UnsupportedFormat(                      // building the error
            args.input
            .extension()                         // returns &OsString type
            .and_then(|s| s.to_str())      // returns None if the option is None
            .unwrap_or("unknown")                 // returns unknown when argument in None
            .to_string(),
        )
    })?;


    let output_fmt = FileFormats::from_path(&args.output)
        .ok_or_else(|| {
            CambiarErrors::UnsupportedFormat(
                args.output
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
            )
        })?;
        

    println!("Detected formats:");
    println!("  Input:  {:?}", input_fmt);
    println!("  Output: {:?}", output_fmt);

    Ok(())

    }

}
