use cambiar::engine::converter::Converter;
use cambiar::engine::csv_json::CsvToJson;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

use std::path::PathBuf;

fn benchmark_csv_json(c: &mut Criterion) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let files = [
        ("tiny", root.join("benches/fixtures/tiny.csv")),
        ("small", root.join("benches/fixtures/small.csv")),
        ("medium", root.join("benches/fixtures/mid.csv")),
        ("large", root.join("benches/fixtures/large.csv")),
    ];

    let mut group = c.benchmark_group("csv_to_json");
    for (name, input) in files {
        let size = std::fs::metadata(&input).unwrap().len();

        group.throughput(Throughput::Bytes(size));

        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("output.json");

        let converter = CsvToJson;

        group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, input| {
            b.iter(|| {
                converter.convert(input, &output).unwrap();
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_csv_json);
criterion_main!(benches);
