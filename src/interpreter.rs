
use crate::{
    ast::{Expression, LoxError, Token, TokenLiteral},
    stmt::Statement,
};

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub variables: Vec<Variable>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: Token,
    value: TokenLiteral,
}

impl Interpreter {
    fn execute(&mut self, statement: Statement) -> Result<(), LoxError> {
        match statement {
            Statement::Expression(ex) => {
                if let Err(e) = ex.evaluate() {
                    eprintln!("{}", e.to_string());
                }
            }
            Statement::Print(ex) => {
                if let Expression::Literal(lit) = &ex {
                    if let Some(var) = self.variables.iter().find(|v| v.name.literal == *lit) {
                        println!("{}", var.value);
                        return Ok(());
                    }
                } else if let Err(e) = ex.evaluate() {
                    return Err(LoxError::ParseError(e));
                }
            }
            Statement::Var(name, initializer) => {
                if let Some(val) = initializer {
                    if let Ok(lit) = val.evaluate() {
                        self.variables.push(Variable { name, value: lit });
                    }
                }
            }
            Statement::Dump => {
                dbg!(self);
            }
        }
        return Ok(());
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            if let Err(e) = self.execute(statement) {
                eprintln!("{}", e.to_string());
            }
        }
    }

    pub fn stringify(&self, literal: TokenLiteral) -> String {
        match literal {
            TokenLiteral::Empty => String::new(),
            TokenLiteral::Integer(i) => i.to_string(),
            TokenLiteral::Float(f) => f.to_string(),
            TokenLiteral::String(s) => s,
            TokenLiteral::Boolean(b) => b.to_string(),
        }
    }

    pub fn new() -> Interpreter {
        Interpreter { variables: vec![] }
    }
}
