use clap::Parser;
use std::fs;
use std::path::PathBuf;
use tree_sitter::{Parser as TsParser, Tree};
use tree_sitter_python::language as python_language;

/// Command-line argument structure using `clap`
#[derive(Parser)]
#[command(name = "fastpy")]
#[command(about = "An extremely fast Python linter and formatter written in Rust")]
struct Args {
    /// Path to the Python file to lint/format
    file: PathBuf,

    /// Enable auto-formatting (output formatted code to stdout)
    #[arg(short, long)]
    format: bool,

    /// Apply fixes directly to file (in-place)
    #[arg(short = 'x', long)]
    fix: bool,

    /// Show parse tree (debug)
    #[arg(short = 'd', long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let code = fs::read_to_string(&args.file)
        .unwrap_or_else(|_| panic!("Failed to read file: {:?}", args.file));

    let tree = parse_code(&code).expect("Failed to parse code");

    if args.debug {
        println!("\n--- Debug Parse Tree ---\n{:?}", tree.root_node());
    }

    println!("\n--- Running Linter ---");
    run_linter(&tree, &code);

    let formatted = format_code(&code);

    if args.format {
        println!("\n--- Formatted Code ---\n{}", formatted);
    }

    if args.fix {
        fs::write(&args.file, formatted).expect("Failed to write changes to file");
        println!("\n✅ File formatted and saved: {:?}", args.file);
    }
}

/// Parses Python source code using tree-sitter
fn parse_code(code: &str) -> Option<Tree> {
    let mut parser = TsParser::new();
    parser.set_language(python_language()).ok()?;
    parser.parse(code, None)
}

/// Very simple linter rule: warn about bad variable/function names
fn run_linter(tree: &Tree, source_code: &str) {
    let root = tree.root_node();
    let mut cursor = root.walk();

    for node in root.children(&mut cursor) {
        match node.kind() {
            "function_definition" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = name_node.utf8_text(source_code.as_bytes()).unwrap_or("");
                    if name == "l" || name == "O" {
                        println!(
                            "[Lint] Function name `{}` is ambiguous (line {})",
                            name,
                            name_node.start_position().row + 1
                        );
                    }
                }
            }
            "assignment" => {
                if let Some(left_node) = node.child_by_field_name("left") {
                    let name = left_node.utf8_text(source_code.as_bytes()).unwrap_or("");
                    if name == "l" {
                        println!(
                            "[Lint] Variable name `l` is ambiguous (line {})",
                            left_node.start_position().row + 1
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

/// A basic formatter: trims trailing spaces (extend as needed)
fn format_code(code: &str) -> String {
    code.lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n") + "\n"
}
