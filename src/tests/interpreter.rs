use crate::{ast::TokenLiteral, stmt::Statement, interpreter::{self, Interpreter}};

#[test]
fn exponent() {
    use crate::parser::Parser;
    let mut parser = Parser::new();
    match parser.load("5**5;".to_string()) {
        Ok(stmts) => {
            assert_eq!(stmts.len(), 1);
            let stmt = stmts.first().unwrap();
            if let Statement::Expression(expr) = stmt {
                let intr = expr.evaluate().expect("Failed to evaluate");
                assert_eq!(TokenLiteral::Integer(3125), intr);
            }
        }
        Err(e) => {
            dbg!(&e);
        }
    }
}

#[test]
fn mul() {
    use crate::parser::Parser;
    let mut parser = Parser::new();
    match parser.load("5*5;".to_string()) {
        Ok(stmts) => {
            assert_eq!(stmts.len(), 1);
            let stmt = stmts.first().unwrap();
            if let Statement::Expression(expr) = stmt {
                let intr = expr.evaluate().expect("Failed to evaluate");
                assert_eq!(TokenLiteral::Integer(25), intr);
            }
        }
        Err(e) => {
            dbg!(&e);
        }
    }
}

#[test]
fn add() {
    use crate::parser::Parser;
    let mut parser = Parser::new();
    match parser.load("5+5;".to_string()) {
        Ok(stmts) => {
            assert_eq!(stmts.len(), 1);
            let stmt = stmts.first().unwrap();
            if let Statement::Expression(expr) = stmt {
                let intr = expr.evaluate().expect("Failed to evaluate");
                assert_eq!(TokenLiteral::Integer(10), intr);
            }
        }
        Err(e) => {
            dbg!(&e);
        }
    }
}

#[test]
fn sub() {
    use crate::parser::Parser;
    let mut parser = Parser::new();
    match parser.load("5-5;".to_string()) {
        Ok(stmts) => {
            assert_eq!(stmts.len(), 1);
            let stmt = stmts.first().unwrap();
            if let Statement::Expression(expr) = stmt {
                let intr = expr.evaluate().expect("Failed to evaluate");
                assert_eq!(TokenLiteral::Integer(0), intr);
            }
        }
        Err(e) => {
            dbg!(&e);
        }
    }
}

#[test]
fn str() {
    use crate::parser::Parser;
    let mut parser = Parser::new();
    match parser.load("'test';".to_string()) {
        Ok(stmts) => {
            assert_eq!(stmts.len(), 1);
            let stmt = stmts.first().unwrap();
            if let Statement::Expression(expr) = stmt {
                let intr = expr.evaluate().expect("Failed to evaluate");
                assert_eq!(TokenLiteral::String("test".to_string()), intr);
            }
        }
        Err(e) => {
            dbg!(&e);
        }
    }
}

#[test]
fn str_concat() {
    use crate::parser::Parser;
    let mut parser = Parser::new();
    match parser.load("'Hello' + ' ' + 'World!';".to_string()) {
        Ok(stmts) => {
            assert_eq!(stmts.len(), 1);
            let stmt = stmts.first().unwrap();
            if let Statement::Expression(expr) = stmt {
                let intr = expr.evaluate().expect("Failed to evaluate");
                assert_eq!(TokenLiteral::String("Hello World!".to_string()), intr);
            }
        }
        Err(e) => {
            dbg!(&e);
        }
    }
}

#[test]
fn str_plus_num() {
    use crate::parser::Parser;
    let mut parser = Parser::new();

    match parser.load("'Hello' + 5;".to_string()) {
        Ok(stmts) => {
            assert_eq!(stmts.len(), 1);
            let stmt = stmts.first().unwrap();
            if let Statement::Expression(expr) = stmt {
                let intr = expr.evaluate().expect("Failed to evaluate");
                assert_eq!(TokenLiteral::String("Hello5".to_string()), intr);
            }
        }
        Err(e) => {
            dbg!(&e);
        }
    }

    match parser.load("1 + 'Hello' + 5;".to_string()) {
        Ok(stmts) => {
            assert_eq!(stmts.len(), 1);
            let stmt = stmts.first().unwrap();
            if let Statement::Expression(expr) = stmt {
                let intr = expr.evaluate().expect("Failed to evaluate");
                assert_eq!(TokenLiteral::String("1Hello5".to_string()), intr);
            }
        }
        Err(e) => {
            dbg!(&e);
        }
    }
}
#[test]
fn variables() {
    use crate::parser::Parser;
    let mut parser = Parser::new();
    let mut interpreter = Interpreter::new();
    match parser.load_file("./tests/variables.lox".to_string()) {
        Ok(stmts) => {
            interpreter.interpret(stmts);
            dbg!(&interpreter);
            assert!(!parser.scanner.had_error);
        }
        Err(e) => {
            dbg!(&e);
        }
    }
}
