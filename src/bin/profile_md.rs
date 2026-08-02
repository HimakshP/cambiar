use cambiar::engine::converter::Converter;
use cambiar::engine::md_txt::MdtoTxt;

use std::path::Path;

fn main() {
    let input = Path::new("benches/fixtures/large.md");
    let output = Path::new("/tmp/cambiar-profile.json");

    let converter = MdtoTxt;

    for _ in 0..30 {
        converter.convert(input, output).unwrap();
    }
}
