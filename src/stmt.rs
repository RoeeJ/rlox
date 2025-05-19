use crate::ast::{Expression, Token};

#[derive(Debug, Clone)]
pub enum Statement {
    Block(Vec<Statement>),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
    Function(Token, Vec<Token>, Vec<Statement>),
    Return(Token, Option<Expression>),
    Expression(Expression),
    Print(Expression),
    Dump,
    Var(Token,Option<Expression>),
}
