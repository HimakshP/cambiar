use cambiar::engine::png_jpg::PngJpg;
use cambiar::engine::converter::Converter;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

use std::path::PathBuf;

fn benchmark_png_jpg(c: &mut Criterion) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let files = [
        ("tiny", root.join("benches/fixtures/tiny.png")),
        ("small", root.join("benches/fixtures/small.png")),
        ("medium", root.join("benches/fixtures/mid.png")),
        ("large", root.join("benches/fixtures/large.png")),
    ];

    let mut group = c.benchmark_group("png_to_jpg");
    for (name, input) in files {
        let size = std::fs::metadata(&input).unwrap().len();

        group.throughput(Throughput::Bytes(size));

        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("output.jpg");

        let converter = PngJpg;

        group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, input| {
            b.iter(|| {
                converter.convert(input, &output).unwrap();
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_png_jpg);
criterion_main!(benches);
