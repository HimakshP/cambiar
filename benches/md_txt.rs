use cambiar::engine::converter::Converter;
use cambiar::engine::md_txt::MdtoTxt;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

use std::path::PathBuf;

fn benchmark_md_txt(c: &mut Criterion) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let files = [
        ("tiny", root.join("benches/fixtures/tiny.md")),
        ("small", root.join("benches/fixtures/small.md")),
        ("medium", root.join("benches/fixtures/mid.md")),
        ("large", root.join("benches/fixtures/large.md")),
    ];

    let mut group = c.benchmark_group("md_to_txt");
    for (name, input) in files {
        let size = std::fs::metadata(&input).unwrap().len();

        group.throughput(Throughput::Bytes(size));

        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("output.txt");

        let converter = MdtoTxt;

        group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, input| {
            b.iter(|| {
                converter.convert(input, &output).unwrap();
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_md_txt);
criterion_main!(benches);
