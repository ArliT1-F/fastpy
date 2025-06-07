use tree_sitter::{Parser, Node};
use tree_sitter_python::language;

fn parse_code(code: &str) -> Option<tree_sitter::Tree> {
    let mut parser = Parser::new();
    parser.set_language(language()).ok()?;
    parser.parse(code, None)
}