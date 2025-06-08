use log::{info, warn, error, debug};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tree_sitter::{Parser as TSParser, Tree};
use walkdir::WalkDir;
use itertools::Itertools;
use tree_sitter_python::language as tree_sitter_python;

#[derive(Parser, Debug)]
#[command(name = "fastpy")]
#[command(about = "An extremely fast Python linter and formatter written in Rust", long_about = None)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(long)]
    fix: bool,

    #[arg(long)]
    json: bool,
}

#[derive(Serialize)]
struct LintMessage {
    file: String,
    line: usize,
    message: String,
}

#[derive(Debug)]
struct Fix {
    start: usize,
    end: usize,
    replacement: String,
}

#[derive(Deserialize, Default)]
struct LintConfig {
    disallow_ambiguous_names: Option<bool>,
    #[allow(dead_code)]
    max_line_length: Option<usize>,
}

fn load_config(path: &Path) -> Option<LintConfig> {
    let config_path = path.parent()?.join(".fastpy.toml");
    let contents = fs::read_to_string(config_path).ok()?;
    toml::from_str(&contents).ok()
}

fn parse_code(code: &str) -> Option<Tree> {
    let language = tree_sitter_python();
    let mut parser = TSParser::new();
    parser.set_language(language).ok()?;
    parser.parse(code, None)
}

fn run_linter(tree: &Tree, source_code: &str, config: &LintConfig) -> Vec<LintMessage> {
    let mut messages = Vec::new();
    messages.extend(lint_ambiguous_names(tree, source_code, config));
    messages
}

fn lint_ambiguous_names(tree: &Tree, source_code: &str, config: &LintConfig) -> Vec<LintMessage> {
    let mut messages = Vec::new();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for node in root.children(&mut cursor) {
        if node.kind() == "assignment" {
            if let Some(left_node) = node.child_by_field_name("left") {
                let name = left_node.utf8_text(source_code.as_bytes()).unwrap_or("");
                if config.disallow_ambiguous_names.unwrap_or(true) && (name == "l" || name == "O") {
                    messages.push(LintMessage {
                        file: "".into(),
                        line: left_node.start_position().row + 1,
                        message: format!("Ambiguous variable name '{}'.", name),
                    });
                }
            }
        }
    }

    messages
}

fn find_fixes(tree: &Tree, source_code: &str) -> Vec<Fix> {
    let mut fixes = Vec::new();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for node in root.children(&mut cursor) {
        if node.kind() == "assignment" {
            if let Some(left_node) = node.child_by_field_name("left") {
                let name = left_node.utf8_text(source_code.as_bytes()).unwrap_or("");
                if name == "l" {
                    fixes.push(Fix {
                        start: left_node.start_byte(),
                        end: left_node.end_byte(),
                        replacement: "line".to_string(),
                    });
                }
            }
        }
    }

    fixes
}

fn apply_fixes(code: &str, fixes: Vec<Fix>) -> String {
    let mut result = String::new();
    let mut last = 0;
    for fix in fixes.iter().sorted_by_key(|f| f.start) {
        result.push_str(&code[last..fix.start]);
        result.push_str(&fix.replacement);
        last = fix.end;
    }
    result.push_str(&code[last..]);
    result
}

fn find_python_files(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("py"))
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let paths = if args.file.is_dir() {
        find_python_files(&args.file)
    } else {
        vec![args.file.clone()]
    };

    for path in paths {
        info!("\n📂 Processing {:?}", path);

        let code = fs::read_to_string(&path).expect("Unable to read file");
        let config = load_config(&path).unwrap_or_default();

        if let Some(tree) = parse_code(&code) {
            let messages = run_linter(&tree, &code, &config);
            if args.json {
                let messages_json: Vec<LintMessage> = messages
                    .into_iter()
                    .map(|mut m| {
                        m.file = path.to_string_lossy().into();
                        m
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&messages_json).unwrap());
            } else {
                for msg in &messages {
                    println!("[line {}] {}", msg.line, msg.message);
                }
            }

            let fixes = find_fixes(&tree, &code);
            if !fixes.is_empty() {
                info!("\n✏️ Auto-fixes available:");
                for fix in &fixes {
                    info!("{:?}", fix);
                }

                let fixed_code = apply_fixes(&code, fixes);
                if args.fix {
                    fs::write(&path, &fixed_code).expect("Failed to write fixed file");
                    info!("✅ Auto-fixed and saved");
                }
            }
        } else {
            error!("❌ Failed to parse {:?}", path);
        }
    }
}
