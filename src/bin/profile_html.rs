use cambiar::engine::converter::Converter;
use cambiar::engine::html_md::HtmlToMd;

use std::path::Path;

fn main() {
    let input = Path::new("benches/fixtures/large.html");
    let output = Path::new("/tmp/cambiar-profile.md");

    let converter = HtmlToMd;

    for _ in 0..30 {
        converter.convert(input, output).unwrap();
    }
}
