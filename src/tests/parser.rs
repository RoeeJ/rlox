#[test]
fn parse() {
    use crate::parser::Parser;

    let mut parser = Parser::new();
    parser.load_file("./tests/parser.lox".to_string()).expect("Failed to load file");
    parser.parse().expect("Failed to parse");
}
