# Cambiar

A lightweight command-line file converter written in Rust.

Cambiar provides a simple interface for converting between common structured data, markup, and image formats directly from the terminal.

## Installation

Install from crates.io:

```bash
cargo install cambiar
```

Verify the installation:

```bash
cambiar --version
```

## Usage

```bash
cambiar <INPUT> <OUTPUT>
```

Cambiar detects the input and output formats from their file extensions.

### Examples

Convert Markdown to plain text:

```bash
cambiar README.md README.txt
```

Convert PNG to JPEG:

```bash
cambiar image.png image.jpg
```

Convert CSV to JSON:

```bash
cambiar data.csv data.json
```

Overwrite an existing output file:

```bash
cambiar image.png image.jpg --force
```

## Supported Conversions

| Input | Output |
| --- | --- |
| CSV | JSON |
| Markdown (`.md`) | Plain text (`.txt`) |
| HTML | Markdown |
| PNG | JPEG |
| JPEG | PNG |

To view supported conversions from the CLI:

```bash
cambiar --list-formats
```

## Design

Cambiar uses a trait-based converter architecture, allowing conversion implementations to remain independent of CLI routing and file validation.

Different formats use different conversion strategies:

- CSV → JSON uses structured serialization.
- Markdown → TXT uses event-based Markdown parsing.
- HTML → Markdown uses markup transformation.
- PNG ↔ JPEG uses image decoding and re-encoding.

## Testing

The project includes integration tests for each converter and end-to-end tests for CLI behavior.

```bash
cargo test
```

The CLI tests cover invalid inputs, unsupported conversions, overwrite protection, and forced overwrites.

## Performance

Performance benchmarks and profiling results are planned for the next release.

The goal is to measure conversion latency and throughput across different file sizes before making optimization claims.

## Limitations

- PNG → JPEG cannot preserve transparency because JPEG has no alpha channel.
- Markdown → TXT preserves readable document content but necessarily discards Markdown formatting.
- Format detection currently relies on file extensions.

## License

MIT