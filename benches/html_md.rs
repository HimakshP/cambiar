use cambiar::engine::converter::Converter;
use cambiar::engine::html_md::HtmlToMd;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

use std::path::PathBuf;

fn benchmark_html_md(c: &mut Criterion) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let files = [
        ("tiny", root.join("benches/fixtures/tiny.html")),
        ("small", root.join("benches/fixtures/small.html")),
        ("medium", root.join("benches/fixtures/mid.html")),
        ("large", root.join("benches/fixtures/large.html")),
    ];

    let mut group = c.benchmark_group("html_to_md");
    for (name, input) in files {
        let size = std::fs::metadata(&input).unwrap().len();

        group.throughput(Throughput::Bytes(size));

        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("output.md");

        let converter = HtmlToMd;

        group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, input| {
            b.iter(|| {
                converter.convert(input, &output).unwrap();
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_html_md);
criterion_main!(benches);