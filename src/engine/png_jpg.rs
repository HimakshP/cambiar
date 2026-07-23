use std::path::Path;

use image::codecs::jpeg::JpegEncoder;

use crate::{
    engine::converter::Converter, errors::CambiarErrors, formats::{FileFormats},
};

pub struct PngJpg;
pub struct JpgPng;

impl Converter for PngJpg {
    fn input_format(&self) -> crate::formats::FileFormats {
        FileFormats::Png
    }

    fn output_format(&self) -> crate::formats::FileFormats {
        FileFormats::Jpg
    }

    fn convert(&self, input: &Path, output: &Path) -> Result<(), CambiarErrors> {

        let img = image::open(input).map_err(|_| CambiarErrors::ReadError)?; // convert file into dynamic image type

        let rgb = img.to_rgb8(); // convert dynimage into rgb pixels

        let jpeg_file = std::fs::File::create(output).map_err(|_| CambiarErrors::WriteError)?; // creating a new file to write to

        let mut encoder = JpegEncoder::new_with_quality(jpeg_file, 90); // creating an encoder with custom quality 

        encoder.encode_image(&rgb).map_err(|_| CambiarErrors::WriteError) // encoding raw rgb pixels into a jpeg file

    }
}