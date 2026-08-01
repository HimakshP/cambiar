use cambiar::engine::converter::Converter;
use cambiar::engine::csv_json::CsvToJson;

use std::path::Path;

fn main() {
    let input = Path::new("benches/fixtures/large.csv");
    let output = Path::new("/tmp/cambiar-profile.json");

    let converter = CsvToJson;

    for _ in 0..30 {
        converter.convert(input, output).unwrap();
    }
}
