use std::{
    fmt::Display,
    hash::Hash,
    ops::{Add, Div, Mul, Sub},
};

/// Return the [`TokenType`] for a reserved keyword if `ident` is a keyword.
pub fn keyword_token_type(ident: &str) -> Option<TokenType> {
    match ident {
        "and" => Some(TokenType::AND),
        "class" => Some(TokenType::CLASS),
        "else" => Some(TokenType::ELSE),
        "false" => Some(TokenType::FALSE),
        "for" => Some(TokenType::FOR),
        "fun" => Some(TokenType::FUN),
        "if" => Some(TokenType::IF),
        "nil" => Some(TokenType::NIL),
        "or" => Some(TokenType::OR),
        "print" => Some(TokenType::PRINT),
        "return" => Some(TokenType::RETURN),
        "super" => Some(TokenType::SUPER),
        "this" => Some(TokenType::THIS),
        "true" => Some(TokenType::TRUE),
        "var" => Some(TokenType::VAR),
        "const" => Some(TokenType::CONST),
        "while" => Some(TokenType::WHILE),
        "dump" => Some(TokenType::DUMP),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
pub enum LoxError {
    InvalidToken {
        token_type: TokenType,
        line: usize,
        loc: usize,
    },
    RuntimeException,
    ExitCode(i32),
    ScanError(char),
    ParseError(ParserError),
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::RuntimeException => {
                write!(f, "Unhandled runtime exception")
            }
            LoxError::ExitCode(c) => {
                write!(f, "Lox exited with code {c}")
            }
            LoxError::ScanError(c) => {
                write!(f, "Error while reading char {c}")
            }
            LoxError::ParseError(e) => {
                write!(f, "Parser error: {e}")
            }
            LoxError::InvalidToken {
                token_type,
                line,
                loc,
            } => {
                write!(f, "Invalid token {token_type:?} at {line}:{loc}")
            }
        }
    }
}

impl From<ParserError> for LoxError {
    fn from(value: ParserError) -> Self {
        Self::ParseError(value)
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Assign { name: Token, value: Box<Expression> },
    Logical { left: Box<Expression>, operator: Token, right: Box<Expression> },
    Call { callee: Box<Expression>, paren: Token, arguments: Vec<Expression> },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Literal(TokenLiteral),
    Variable(Token),
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnsupportedAction,
    Generic(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnsupportedAction => write!(f, "Unsupported Action"),
            ParserError::Generic(s) => write!(f, "Generic Error({})", s),
        }
    }
}

impl Expression {
    pub fn evaluate(&self) -> Result<TokenLiteral, ParserError> {
        return match self {
            crate::ast::Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                match operator.token_type {
                    TokenType::MINUS => {
                        if !self.check_number_operand(operator, &right) {
                            return Err(ParserError::UnsupportedAction);
                        }
                        return left.sub(right);
                    }
                    TokenType::PLUS => {
                        return left.add(right);
                    }
                    TokenType::SLASH => {
                        if !self.check_number_operand(operator, &right) {
                            return Err(ParserError::UnsupportedAction);
                        }
                        return left / right;
                    }
                    TokenType::STAR => {
                        if !self.check_number_operand(operator, &right) {
                            return Err(ParserError::UnsupportedAction);
                        }
                        return left * right;
                    }
                    TokenType::EXPONENT => {
                        if !self.check_number_operand(operator, &right) {
                            return Err(ParserError::UnsupportedAction);
                        }
                        return left.pow(right);
                    }
                    TokenType::GREATER => {
                        if !self.check_number_operand(operator, &right) {
                            return Err(ParserError::UnsupportedAction);
                        }
                        if let TokenLiteral::Integer(left) = left {
                            if let TokenLiteral::Integer(right) = right {
                                return Ok(TokenLiteral::Boolean(left > right));
                            }
                        }
                        return Ok(TokenLiteral::Empty);
                    }
                    TokenType::GREATER_EQUAL => {
                        if !self.check_number_operand(operator, &right) {
                            return Err(ParserError::UnsupportedAction);
                        }
                        if let TokenLiteral::Integer(left) = left {
                            if let TokenLiteral::Integer(right) = right {
                                return Ok(TokenLiteral::Boolean(left >= right));
                            }
                        }
                        return Ok(TokenLiteral::Empty);
                    }
                    TokenType::LESS => {
                        if !self.check_number_operand(operator, &right) {
                            return Err(ParserError::UnsupportedAction);
                        }
                        if let TokenLiteral::Integer(left) = left {
                            if let TokenLiteral::Integer(right) = right {
                                return Ok(TokenLiteral::Boolean(left < right));
                            }
                        }
                        return Ok(TokenLiteral::Empty);
                    }
                    TokenType::LESS_EQUAL => {
                        if !self.check_number_operand(operator, &right) {
                            return Err(ParserError::UnsupportedAction);
                        }
                        if let TokenLiteral::Integer(left) = left {
                            if let TokenLiteral::Integer(right) = right {
                                return Ok(TokenLiteral::Boolean(left <= right));
                            }
                        }
                        return Ok(TokenLiteral::Empty);
                    }
                    TokenType::BANG_EQUAL => {
                        return Ok(TokenLiteral::Boolean(!left.is_equal(right)));
                    }
                    TokenType::EQUAL_EQUAL => {
                        return Ok(TokenLiteral::Boolean(left.is_equal(right)));
                    }
                    _ => todo!(),
                }
            }
            crate::ast::Expression::Unary { operator, right } => {
                let right = right.evaluate()?;
                match operator.token_type {
                    TokenType::MINUS => {
                        if !self.check_number_operand(operator, &right) {
                            return Err(ParserError::UnsupportedAction);
                        }
                        if let TokenLiteral::Integer(n) = right {
                            return Ok(TokenLiteral::Integer(-n));
                        }
                        todo!()
                    }
                    TokenType::BANG => {
                        return Ok(TokenLiteral::Boolean(!right.is_truthy()));
                    }
                    _ => todo!(),
                }
            }
            Expression::Grouping(sub_expr) => sub_expr.evaluate(),
            Expression::Literal(lit) => Ok(lit.clone()),
            Expression::Empty => Ok(TokenLiteral::Empty),
            Expression::Variable(token) => Ok(token.literal.clone()),
            Expression::Assign { .. }
            | Expression::Logical { .. }
            | Expression::Call { .. } => Err(ParserError::UnsupportedAction),
        };
    }

    fn check_number_operand(&self, _operator: &Token, operand: &TokenLiteral) -> bool {
        match operand {
            TokenLiteral::Integer(_) | TokenLiteral::Float(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: TokenLiteral,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenLiteral {
    Empty,
    Integer(isize),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl Display for TokenLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            TokenLiteral::Empty => String::new(),
            TokenLiteral::Integer(i) => i.to_string(),
            TokenLiteral::Float(f) => f.to_string(),
            TokenLiteral::String(s) => s.clone(),
            TokenLiteral::Boolean(b) => b.to_string(),
        };

        write!(f, "{}", val)
    }
}

impl TokenLiteral {
    pub fn pow(&self, rhs: TokenLiteral) -> Result<TokenLiteral, ParserError> {
        match self {
            TokenLiteral::Integer(i) => match rhs {
                TokenLiteral::Integer(ii) => Ok(TokenLiteral::Integer(i.pow(ii as u32))),
                _ => Ok(TokenLiteral::Empty),
            },
            TokenLiteral::Float(f) => match rhs {
                TokenLiteral::Float(ff) => Ok(TokenLiteral::Float(f.powf(ff))),
                TokenLiteral::Integer(fi) => Ok(TokenLiteral::Float(f.powi(fi as i32))),
                _ => Ok(TokenLiteral::Empty),
            },
            _ => Ok(TokenLiteral::Empty),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            TokenLiteral::Empty => false,
            TokenLiteral::Integer(n) => *n != 0,
            TokenLiteral::Float(n) => *n != 0.0,
            TokenLiteral::String(_) => true,
            TokenLiteral::Boolean(b) => *b,
        }
    }

    pub fn is_equal(&self, rhs: TokenLiteral) -> bool {
        match self {
            TokenLiteral::Empty => false,
            TokenLiteral::Integer(left) => {
                return match rhs {
                    TokenLiteral::Float(right) => return right == *left as f64,
                    TokenLiteral::Integer(right) => return right == *left,
                    _ => false,
                };
            }
            TokenLiteral::Float(left) => {
                return match rhs {
                    TokenLiteral::Float(right) => return right == *left,
                    TokenLiteral::Integer(right) => return *left == right as f64,
                    _ => false,
                };
            }
            TokenLiteral::String(left) => {
                if let TokenLiteral::String(right) = rhs {
                    return *left == right;
                }
                return false;
            }
            TokenLiteral::Boolean(left) => {
                if let TokenLiteral::Boolean(right) = rhs {
                    return *left == right;
                }
                return false;
            }
        }
    }
}

impl Mul for TokenLiteral {
    type Output = Result<TokenLiteral, ParserError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            TokenLiteral::Float(lhs) => {
                return match rhs {
                    TokenLiteral::Float(rhs) => Ok(TokenLiteral::Float(lhs * rhs)),
                    TokenLiteral::Integer(rhs) => Ok(TokenLiteral::Float(lhs * rhs as f64)),
                    _ => todo!(),
                };
            }
            TokenLiteral::Integer(lhs) => {
                return match rhs {
                    TokenLiteral::Float(rhs) => Ok(TokenLiteral::Float((lhs as f64) * rhs)),
                    TokenLiteral::Integer(rhs) => Ok(TokenLiteral::Integer(lhs * rhs)),
                    _ => todo!(),
                };
            }
            _ => todo!(),
        }
    }
}

impl Div for TokenLiteral {
    type Output = Result<TokenLiteral, ParserError>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            TokenLiteral::Float(lhs) => {
                return match rhs {
                    TokenLiteral::Float(rhs) => Ok(TokenLiteral::Float(lhs / rhs)),
                    TokenLiteral::Integer(rhs) => Ok(TokenLiteral::Float(lhs / rhs as f64)),
                    _ => todo!(),
                };
            }
            TokenLiteral::Integer(lhs) => {
                return match rhs {
                    TokenLiteral::Float(rhs) => Ok(TokenLiteral::Float((lhs as f64) / rhs)),
                    TokenLiteral::Integer(rhs) => Ok(TokenLiteral::Float(lhs as f64 / rhs as f64)),
                    _ => todo!(),
                };
            }
            _ => todo!(),
        }
    }
}

impl Sub for TokenLiteral {
    type Output = Result<TokenLiteral, ParserError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            TokenLiteral::Float(lhs) => {
                return match rhs {
                    TokenLiteral::Float(rhs) => Ok(TokenLiteral::Float(lhs - rhs)),
                    TokenLiteral::Integer(rhs) => Ok(TokenLiteral::Float(lhs - rhs as f64)),
                    _ => todo!(),
                };
            }
            TokenLiteral::Integer(lhs) => {
                return match rhs {
                    TokenLiteral::Float(rhs) => Ok(TokenLiteral::Float((lhs as f64) - rhs)),
                    TokenLiteral::Integer(rhs) => Ok(TokenLiteral::Integer(lhs - rhs)),
                    _ => todo!(),
                };
            }
            _ => todo!(),
        }
    }
}

impl Add for TokenLiteral {
    type Output = Result<TokenLiteral, ParserError>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            TokenLiteral::Float(lhs) => {
                match rhs {
                    TokenLiteral::Float(rhs) => return Ok(TokenLiteral::Float(lhs + rhs)),
                    TokenLiteral::Integer(rhs) => return Ok(TokenLiteral::Float(lhs + rhs as f64)),
                    TokenLiteral::String(rhs) => {
                        return Ok(TokenLiteral::String(format!("{}{}", lhs, rhs)))
                    }
                    _ => todo!(),
                };
            }
            TokenLiteral::Integer(lhs) => {
                match rhs {
                    TokenLiteral::Float(rhs) => return Ok(TokenLiteral::Float((lhs as f64) + rhs)),
                    TokenLiteral::Integer(rhs) => return Ok(TokenLiteral::Integer(lhs + rhs)),
                    TokenLiteral::String(rhs) => {
                        return Ok(TokenLiteral::String(format!("{}{}", lhs, rhs)))
                    }
                    _ => todo!(),
                };
            }
            TokenLiteral::String(lhs) => {
                match rhs {
                    TokenLiteral::String(rhs) => {
                        return Ok(TokenLiteral::String(format!("{}{}", lhs, rhs)))
                    }
                    TokenLiteral::Integer(rhs) => {
                        return Ok(TokenLiteral::String(format!("{}{}", lhs, rhs)))
                    }
                    _ => {}
                };
            }
            _ => {}
        };
        return Err(ParserError::UnsupportedAction);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    EXPONENT,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    CONST,
    WHILE,

    EOF,

    COMMENT,
    BLOCK_COMMENT,
    DUMP,
}
