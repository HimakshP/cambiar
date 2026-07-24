use crate::{engine::converter::Converter, errors::CambiarErrors, formats::FileFormats};

pub struct HtmlToMd;

impl Converter for HtmlToMd {
    fn input_format(&self) -> crate::formats::FileFormats {
        FileFormats::Html
    }

    fn output_format(&self) -> crate::formats::FileFormats {
        FileFormats::Md
    }

    fn convert(&self, input: &std::path::Path, output: &std::path::Path) -> Result<(), crate::errors::CambiarErrors> {

        let html = std::fs::read_to_string(input).map_err(|_| CambiarErrors::ReadError)?;

        let markdown= htmd::convert(&html).map_err(|_| CambiarErrors::ConverterError)?;

        std::fs::write(output, markdown).map_err(|_| CambiarErrors::WriteError)?;
        
        Ok(())
    }
}