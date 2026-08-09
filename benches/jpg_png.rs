use cambiar::engine::converter::Converter;
use cambiar::engine::png_jpg::JpgPng;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

use std::path::PathBuf;

fn benchmark_jpg_png(c: &mut Criterion) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let files = [
        ("tiny", root.join("benches/fixtures/tiny.jpg")),
        ("small", root.join("benches/fixtures/small.jpg")),
        ("medium", root.join("benches/fixtures/mid.jpg")),
        ("large", root.join("benches/fixtures/large.jpg")),
    ];

    let mut group = c.benchmark_group("jpg_to_png");
    for (name, input) in files {
        let size = std::fs::metadata(&input).unwrap().len();

        group.throughput(Throughput::Bytes(size));

        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("output.png");

        let converter = JpgPng;

        group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, input| {
            b.iter(|| {
                converter.convert(input, &output).unwrap();
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_jpg_png);
criterion_main!(benches);
