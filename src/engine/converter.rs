use std::path::Path;

use crate::errors::CambiarErrors;
use crate::formats::FileFormats;

pub trait Converter {

    fn input_format(&self) -> FileFormats;

    fn output_format(&self) -> FileFormats;

    fn convert(
        &self,
        input: &Path,
        output: &Path,
    ) -> Result<(), CambiarErrors>;
}