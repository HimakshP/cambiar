use cambiar::engine::converter::Converter;
use cambiar::engine::png_jpg::PngJpg;

use std::path::Path;

fn main() {
    let input = Path::new("benches/fixtures/large.png");
    let output = Path::new("/tmp/cambiar-profile.jpg");

    let converter = PngJpg;

    for _ in 0..30 {
        converter.convert(input, output).unwrap();
    }
}
