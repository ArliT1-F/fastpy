# fastpy

An extremely fast Python linter and code formatter written in Rust.  
It uses [tree-sitter](https://tree-sitter.github.io/) to parse Python syntax at blazing speed and provides instant feedback on code issues, formatting, and structure.

## Features

- ⚡ Blazing fast thanks to Rust and tree-sitter
- 🧠 Simple lint rules (bad variable/function names, ambiguous symbols)
- 🧹 Basic formatter (trims trailing whitespace)
- 🔧 Optional in-place file formatting
- 🧪 Debug mode for syntax tree visualization
- CLI built with `clap`

## Installation

1. Install Rust: https://rustup.rs  
2. Clone this repo and build:

```bash
git clone https://github.com/yourname/fastpy
cd fastpy
cargo build --release
