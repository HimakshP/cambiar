use cambiar::engine::{converter::{self}, csv_json::CsvToJson};
use criterion::{criterion_group,criterion_main, Criterion};

fn csv_json(c: &mut Criterion) {
    c.bench_function(
        "convert",
        |b| b.iter(|| {
            // Code to benchmark goes here
        let input =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benches/fixtures/tiny.csv");
            
        let dir = tempfile::tempdir().unwrap();

        let output = dir.path().join("output.json");

        converter::Converter::convert(&CsvToJson, input.as_path(), output.as_path()).unwrap()
        }), 
    );
}

criterion_group!(benches, csv_json);
criterion_main!(benches);
