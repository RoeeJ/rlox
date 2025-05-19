#![allow(unused_imports)]
use crate::{
    ast::TokenLiteral,
    interpreter::Interpreter,
    parser::Parser,
    stmt::Statement,
};
#[test]
fn print() {
    let mut parser = Parser::new();
    match parser.load_file("./tests/print.lox".to_string()) {
        Ok(stmts) => {
            dbg!(&stmts);
            assert_eq!(stmts.len(), 4);
        }
        Err(e) => {
            dbg!(&e);
        }
    }
}

#[test]
fn exponent() {
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

#[test]
fn function_call() {
    use crate::{parser::Parser, interpreter::Interpreter, stmt::Statement, ast::{Expression, TokenLiteral}};
    let mut parser = Parser::new();
    let stmts = parser
        .load("fun add(a, b){ return a + b; } add(1, 2);".to_string())
        .expect("failed to parse");
    let mut interpreter = Interpreter::new();
    // first statement defines the function
    interpreter.interpret(vec![stmts[0].clone()]);
    if let Statement::Expression(expr) = &stmts[1] {
        let val = interpreter.evaluate(expr).expect("failed to eval");
        assert_eq!(TokenLiteral::Integer(3), val);
    } else {
        panic!("unexpected statement");
    }
}

#[test]
fn while_loop() {
    use crate::{parser::Parser, interpreter::Interpreter, ast::TokenLiteral};
    let mut parser = Parser::new();
    let stmts = parser
        .load("var a = 0; while (a < 3) { a = a + 1; }".to_string())
        .expect("failed to parse");
    let mut interpreter = Interpreter::new();
    interpreter.interpret(stmts);
    assert_eq!(interpreter.get_var("a"), Some(TokenLiteral::Integer(3)));
}
