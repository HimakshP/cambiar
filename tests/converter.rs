use std::{assert_eq, println};

use cambiar::engine::converter::Converter;
use cambiar::engine::csv_json::CsvToJson;
use cambiar::engine::html_md::HtmlToMd;
use cambiar::engine::md_txt::MdtoTxt;
use cambiar::engine::png_jpg::{JpgPng, PngJpg};

#[test]
fn converts_markdown_to_text() {
    let dir = tempfile::tempdir().unwrap();

    let input = dir.path().join("input.md");
    let output = dir.path().join("output.txt");

    std::fs::write(&input, "# Cambiar\n\nThis is **fast**.\n\n- CSV\n- JSON").unwrap();

    let converter = MdtoTxt;
    converter.convert(&input, &output).unwrap();

    let result = std::fs::read_to_string(&output).unwrap();

    // println!("{result}");

    assert_eq!(result, "Cambiar\n\nThis is fast.\n\n• CSV\n• JSON\n");
}

#[test]
fn converts_csv_to_json() {
    let dir = tempfile::tempdir().unwrap();

    let input = dir.path().join("input.csv");
    let output = dir.path().join("output.json");

    std::fs::write(
        &input,
        "name,age
Alice,21
Bob,25",
    )
    .unwrap();

    let converter = CsvToJson;

    converter.convert(&input, &output).unwrap();

    let result = std::fs::read_to_string(&output).unwrap();

    let actual: serde_json::Value = serde_json::from_str(&result).unwrap();

    println!("{actual}");
    let expected = serde_json::json!([
        {
            "name": "Alice",
            "age": "21"
        },
        {
            "name": "Bob",
            "age": "25"
        }
    ]);

    assert_eq!(actual, expected);
}

#[test]
fn converts_html_to_md() {
    let dir = tempfile::tempdir().unwrap();

    let input = dir.path().join("input.html");
    let output = dir.path().join("output.md");

    std::fs::write(
        &input,
        "<h1>Cambiar</h1>
    <p>A <strong>fast</strong> converter.</p>
    <ul>
        <li>CSV</li>
        <li>Images</li>
    </ul>",
    )
    .unwrap();

    let converter = HtmlToMd;

    converter.convert(&input, &output).unwrap();

    let result = std::fs::read_to_string(&output).unwrap();

    let expected = htmd::convert(
        "<h1>Cambiar</h1>
    <p>A <strong>fast</strong> converter.</p>
    <ul>
        <li>CSV</li>
        <li>Images</li>
    </ul>",
    )
    .unwrap();

    // println!("{result}");

    assert_eq!(result.trim(), expected.trim());
}

#[test]
fn converts_png_to_jpg() {
    let dir = tempfile::tempdir().unwrap();

    let input =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/judah.png");
    let output = dir.path().join("output.jpg");

    let converter = PngJpg;

    converter.convert(&input, &output).unwrap();

    let result = image::open(&output).unwrap();

    assert_eq!(result.width(), 900);
    assert_eq!(result.height(), 841);

    let format = image::ImageReader::open(&output)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .format();

    assert_eq!(format, Some(image::ImageFormat::Jpeg));
    // println!("{:?}", result);
}

#[test]
fn converts_jpg_to_png() {
    let input = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/as.jpg");

    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("output.png");

    let converter = JpgPng;
    converter.convert(&input, &output).unwrap();

    let format = image::ImageReader::open(&output)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .format();

    assert_eq!(format, Some(image::ImageFormat::Png));

    let result = image::open(&output).unwrap();

    assert_eq!(result.width(), 1246);
    assert_eq!(result.height(), 1287);
}
