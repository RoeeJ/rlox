use std::collections::HashMap;
use std::ops::{Add, Sub};

use crate::{
    ast::{Expression, LoxError, ParserError, Token, TokenLiteral, TokenType},
    stmt::Statement,
};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Default)]
pub struct Interpreter {
    envs: Vec<HashMap<String, TokenLiteral>>,
    functions: HashMap<String, Function>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            envs: vec![HashMap::new()],
            functions: HashMap::new(),
        }
    }

    fn define(&mut self, name: &str, value: TokenLiteral) {
        if let Some(env) = self.envs.last_mut() {
            env.insert(name.to_string(), value);
        }
    }

    fn assign(&mut self, name: &str, value: TokenLiteral) {
        for env in self.envs.iter_mut().rev() {
            if env.contains_key(name) {
                env.insert(name.to_string(), value);
                return;
            }
        }
        self.envs[0].insert(name.to_string(), value);
    }

    fn get(&self, name: &str) -> Option<TokenLiteral> {
        for env in self.envs.iter().rev() {
            if let Some(v) = env.get(name) {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            if let Err(e) = self.execute(&statement) {
                eprintln!("{}", e.to_string());
            }
        }
    }

    pub fn get_var(&self, name: &str) -> Option<TokenLiteral> {
        self.get(name)
    }

    fn execute(&mut self, statement: &Statement) -> Result<Option<TokenLiteral>, LoxError> {
        match statement {
            Statement::Expression(ex) => {
                let _ = self.evaluate(ex)?;
                Ok(None)
            }
            Statement::Print(ex) => {
                let val = self.evaluate(ex)?;
                println!("{}", val);
                Ok(None)
            }
            Statement::Var(name, initializer) => {
                let val = if let Some(expr) = initializer {
                    self.evaluate(expr)?
                } else {
                    TokenLiteral::Empty
                };
                self.define(&name.lexeme, val);
                Ok(None)
            }
            Statement::Block(stmts) => {
                self.envs.push(HashMap::new());
                let mut ret = None;
                for stmt in stmts {
                    if let Some(v) = self.execute(stmt)? {
                        ret = Some(v);
                        break;
                    }
                }
                self.envs.pop();
                Ok(ret)
            }
            Statement::If(cond, then_branch, else_branch) => {
                if self.evaluate(cond)?.is_truthy() {
                    self.execute(then_branch)
                } else if let Some(else_stmt) = else_branch {
                    self.execute(else_stmt)
                } else {
                    Ok(None)
                }
            }
            Statement::While(cond, body) => {
                while self.evaluate(cond)?.is_truthy() {
                    if let Some(v) = self.execute(body)? {
                        return Ok(Some(v));
                    }
                }
                Ok(None)
            }
            Statement::Function(name, params, body) => {
                self.functions.insert(
                    name.lexeme.clone(),
                    Function {
                        name: name.clone(),
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
                Ok(None)
            }
            Statement::Return(_, value) => {
                let val = if let Some(expr) = value {
                    self.evaluate(expr)?
                } else {
                    TokenLiteral::Empty
                };
                Ok(Some(val))
            }
            Statement::Dump => {
                dbg!(self);
                Ok(None)
            }
        }
    }

    pub fn evaluate(&mut self, expr: &Expression) -> Result<TokenLiteral, LoxError> {
        match expr {
            Expression::Literal(lit) => Ok(lit.clone()),
            Expression::Grouping(e) => self.evaluate(e),
            Expression::Unary { operator, right } => {
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::MINUS => {
                        if let TokenLiteral::Integer(n) = right {
                            Ok(TokenLiteral::Integer(-n))
                        } else {
                            Err(LoxError::ParseError(ParserError::UnsupportedAction))
                        }
                    }
                    TokenType::BANG => Ok(TokenLiteral::Boolean(!right.is_truthy())),
                    _ => Err(LoxError::RuntimeException),
                }
            }
            Expression::Binary { left, operator, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::PLUS => Ok(left_val.add(right_val)?),
                    TokenType::MINUS => Ok(left_val.sub(right_val)?),
                    TokenType::STAR => Ok((left_val * right_val)?),
                    TokenType::SLASH => Ok((left_val / right_val)?),
                    TokenType::EXPONENT => Ok(left_val.pow(right_val)?),
                    TokenType::GREATER => Ok(TokenLiteral::Boolean(match (left_val, right_val) {
                        (TokenLiteral::Integer(a), TokenLiteral::Integer(b)) => a > b,
                        _ => false,
                    })),
                    TokenType::GREATER_EQUAL => Ok(TokenLiteral::Boolean(match (left_val, right_val) {
                        (TokenLiteral::Integer(a), TokenLiteral::Integer(b)) => a >= b,
                        _ => false,
                    })),
                    TokenType::LESS => Ok(TokenLiteral::Boolean(match (left_val, right_val) {
                        (TokenLiteral::Integer(a), TokenLiteral::Integer(b)) => a < b,
                        _ => false,
                    })),
                    TokenType::LESS_EQUAL => Ok(TokenLiteral::Boolean(match (left_val, right_val) {
                        (TokenLiteral::Integer(a), TokenLiteral::Integer(b)) => a <= b,
                        _ => false,
                    })),
                    TokenType::BANG_EQUAL => Ok(TokenLiteral::Boolean(!left_val.is_equal(right_val))),
                    TokenType::EQUAL_EQUAL => Ok(TokenLiteral::Boolean(left_val.is_equal(right_val))),
                    _ => Err(LoxError::RuntimeException),
                }
            }
            Expression::Variable(tok) => {
                if let Some(val) = self.get(&tok.lexeme) {
                    Ok(val)
                } else {
                    Ok(tok.literal.clone())
                }
            }
            Expression::Assign { name, value } => {
                let val = self.evaluate(value)?;
                self.assign(&name.lexeme, val.clone());
                Ok(val)
            }
            Expression::Logical { left, operator, right } => {
                let left_val = self.evaluate(left)?;
                if operator.token_type == TokenType::OR {
                    if left_val.is_truthy() {
                        return Ok(left_val);
                    }
                } else if !left_val.is_truthy() {
                    return Ok(left_val);
                }
                self.evaluate(right)
            }
            Expression::Call { callee, arguments, .. } => {
                if let Expression::Variable(name_tok) = &**callee {
                    if let Some(func) = self.functions.get(&name_tok.lexeme).cloned() {
                        let mut args_vals = Vec::new();
                        for arg in arguments {
                            args_vals.push(self.evaluate(arg)?);
                        }
                        self.envs.push(HashMap::new());
                        for (param, val) in func.params.iter().zip(args_vals.into_iter()) {
                            self.define(&param.lexeme, val);
                        }
                        let mut ret = TokenLiteral::Empty;
                        for stmt in &func.body {
                            if let Some(v) = self.execute(stmt)? {
                                ret = v;
                                break;
                            }
                        }
                        self.envs.pop();
                        Ok(ret)
                    } else {
                        Err(LoxError::RuntimeException)
                    }
                } else {
                    Err(LoxError::RuntimeException)
                }
            }
            Expression::Empty => Ok(TokenLiteral::Empty),
        }
    }
}
