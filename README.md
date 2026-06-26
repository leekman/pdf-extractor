# PDF Extractor

A lightweight desktop application that converts PDF files to Markdown. Built with Rust using [egui](https://github.com/emileguip/egui) for the GUI and [pdf-extract](https://crates.io/crates/pdf-extract) for native PDF text extraction.

## Features

- Drag & drop or file picker to open PDFs
- Extracts text content and displays it as editable Markdown
- Save output as `.md` files
- Single binary, no external dependencies

## Download

Pre-built binaries for Linux, macOS (ARM64 & x86_64), and Windows are available on the [Releases](../../releases/latest) page.

## Build from source

```bash
cargo build --release
```

The binary will be at `target/release/pdf-extractor`.

## License

[MIT](LICENSE)
