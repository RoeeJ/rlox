#[test]
fn scan_tokens() {
    use crate::scanner::Scanner;
    let mut scanner = Scanner::default();
    assert_eq!(scanner.tokens.len(), 0); //These all should be default
    assert_eq!(scanner.source.len(), 0); //These all should be default
    assert_eq!(scanner.start, 0); //These all should be default
    assert_eq!(scanner.current, 0); //These all should be default
    assert_eq!(scanner.line, 1); //These all should be default
    scanner.load(
        std::fs::read_to_string("./tests/scanner.lox")
            .expect("Faild to load test.lox")
            .chars()
            .collect(),
    );
    dbg!(&scanner.tokens);
    //Assuming we parsed the file successfully we should have tokens
    assert_ne!(0, scanner.tokens.len());
}
