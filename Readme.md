# Markdown Preview Tool

A tool that converts Markdown to HTML and opens it in a web browser for preview.

## Introduction

The Markdown Preview Tool is a command-line utility written in Rust. It allows you to convert Markdown files to HTML and open them in a web browser for previewing.

## Installation

To use the Markdown Preview Tool, you need to have Rust installed on your system. If you don't have Rust installed, you can get it from the official website: [https://www.rust-lang.org/](https://www.rust-lang.org/)

Once you have Rust installed, you can clone this repository and build the tool using Cargo, the Rust package manager:

```sh
git clone https://github.com/sabry-awad97/markdown-preview-tool.git
cd markdown-preview-tool
cargo build --release
```

After building the tool, you can find the executable in the `target/release` directory.

### Usage

---

To preview a Markdown file, run the following command:

```sh
./mdp <markdown_file>
./mdp <markdown_file> --css <css_file>
```

Replace `<markdown_file>` with the path to your Markdown file.

To Preview a Markdown file with a custom CSS file:

```sh
./mdp <markdown_file> --css <css_file>
```

The tool will convert the Markdown file to HTML, save it to a temporary file, and open it in a web browser for previewing.

## License

This project is licensed under the [MIT License](LICENSE).
