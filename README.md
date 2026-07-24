Cambiar
Fast CLI file converter written in Rust.

Supported conversions
- CSV → JSON
- Markdown → TXT
- HTML → Markdown
- PNG → JPEG
- JPEG → PNG

Installation
cargo install --path .

Usage
cambiar input.png output.jpg
cambiar input.md output.txt

cambiar --list-formats
cambiar --help

Testing
cargo test

Performance
Benchmarks coming with v0.1.1.