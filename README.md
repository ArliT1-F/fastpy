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
```

## Usage
```bash
fastpy <file.py> [OPTIONS]
```
### Options:
- `-f`, `--format` ---> Print formatted code to stdout
- `-x`, `--fix` ---> Apply formatting directly to the file
- `-d`, `--debug` ---> Print the parse tree for debugging

### Example:
```bash
fastpy example.py --format
fastpy script.py --fix
fastpy test.py --debug
```

### Example Output:
```bash
--- Running Linter ---
[Lint] Variable name 'l' is ambiguous (line 4)
[Lint] Variable name 'O' is ambiguous (line 12)

--- Formatted Code ---
def test():
    print("cleaned up")

✅ File formatted and saved: "test.py"
```

## Roadmap
- Auto-fix common issues
- Configurable linter rules
- Support for entire directories
- JSON output for integrations

## Licence
MIT