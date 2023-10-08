use crate::ast::{Expression, Token};

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Dump,
    Var(Token,Option<Expression>),
}
