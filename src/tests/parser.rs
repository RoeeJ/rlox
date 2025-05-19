#[test]
fn parse() {
    use crate::parser::Parser;

    let mut parser = Parser::new();
    parser.load_file("./tests/parser.lox".to_string()).expect("Failed to load file");
    parser.parse().expect("Failed to parse");
}

#[test]
fn var_declaration() {
    use crate::{parser::Parser, stmt::Statement, ast::{Expression, TokenLiteral}};

    let mut parser = Parser::new();
    let stmts = parser
        .load("var a = 42;".to_string())
        .expect("failed to parse var");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::Var(token, Some(Expression::Literal(TokenLiteral::Integer(n)))) => {
            assert_eq!(token.lexeme, "a");
            assert_eq!(*n, 42);
        }
        other => panic!("unexpected stmt {:?}", other),
    }
}

#[test]
fn identifier_expression() {
    use crate::{parser::Parser, stmt::Statement, ast::Expression};
    let mut parser = Parser::new();
    let stmts = parser
        .load("print value;".to_string())
        .expect("failed to parse print");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::Print(Expression::Variable(tok)) => assert_eq!(tok.lexeme, "value"),
        other => panic!("unexpected stmt {:?}", other),
    }
}

#[test]
fn function_declaration_and_call() {
    use crate::{parser::Parser, stmt::Statement, ast::Expression};
    let mut parser = Parser::new();
    let stmts = parser
        .load("fun greet(a){ print a; } greet('hi');".to_string())
        .expect("failed to parse");
    assert_eq!(stmts.len(), 2);
    match &stmts[0] {
        Statement::Function(name, params, body) => {
            assert_eq!(name.lexeme, "greet");
            assert_eq!(params.len(), 1);
            assert!(!body.is_empty());
        }
        other => panic!("unexpected stmt {:?}", other),
    }
    match &stmts[1] {
        Statement::Expression(Expression::Call { .. }) => {}
        other => panic!("unexpected stmt {:?}", other),
    }
}

#[test]
fn if_else_statement() {
    use crate::{parser::Parser, stmt::Statement};
    let mut parser = Parser::new();
    let stmts = parser
        .load("if (true) { print 1; } else { print 2; }".to_string())
        .expect("failed to parse");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::If(_, _, Some(_)) => {}
        other => panic!("unexpected stmt {:?}", other),
    }
}
