use crate::{
    ast::{Expression, LoxError, ParserError, Token, TokenLiteral, TokenType},
    scanner::Scanner,
    stmt::Statement,
};

#[derive(Debug, Clone, Default)]
pub struct Parser {
    pub current: usize,
    pub line: usize,
    pub statements: Vec<Statement>,
    pub scanner: Scanner,
    pub had_error: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            line: 1,
            ..Default::default()
        }
    }

    pub fn load(&mut self, source: String) -> Result<Vec<Statement>, LoxError> {
        self.scanner.load(source.chars().collect());
        let stmts = self.parse()?;
        self.statements.extend_from_slice(&stmts);
        return Ok(stmts);
    }

    pub fn load_file(&mut self, path: String) -> Result<Vec<Statement>, LoxError> {
        match std::fs::read_to_string(path) {
            Ok(source) => self.load(source),
            Err(e) => Err(LoxError::ParseError(ParserError::Generic(format!(
                "{}",
                e.to_string()
            )))),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, LoxError> {
        let mut statements = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                Err(e) => {
                    let cur_token = self.peek();
                    self.err(cur_token, e.to_string());
                }
            }
        }
        return Ok(statements);
    }

    fn declaration(&mut self) -> Result<Statement, LoxError> {
        if self.consume_if_type(&[TokenType::VAR]) {
            return self.var_declaration();
        } else if self.consume_if_type(&[TokenType::DUMP]) {
            return self.dump_statement();
        }
        return self.statement();
    }

    fn dump_statement(&mut self) -> Result<Statement, LoxError> {
        let err_msg = "Expected ; after dump statement.";
        self.consume(TokenType::SEMICOLON, err_msg.to_string())?;
        return Ok(Statement::Dump);
    }

    fn var_declaration(&mut self) -> Result<Statement, LoxError> {
        let name = self.consume(TokenType::IDENTIFIER, "Expected variable name".to_string())?;
        let mut initializer = None;
        if self.consume_if_type(&[TokenType::EQUAL]) {
            initializer = self.expression().ok();
        }
        if let Err(e) = self.consume(
            TokenType::SEMICOLON,
            "Expected ';' after variale declaration.".to_string(),
        ) {
            let cur_token = self.peek();
            self.err(cur_token, e.to_string());
            return Err(e);
        }
        return Ok(Statement::Var(name, initializer));
    }

    fn statement(&mut self) -> Result<Statement, LoxError> {
        if self.consume_if_type(&[TokenType::PRINT]) {
            return self.print_statement();
        }
        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Result<Statement, LoxError> {
        if self.peek().token_type == TokenType::IDENTIFIER {
            let cur_token = self.peek();
            self.next();
            self.consume(
                TokenType::SEMICOLON,
                "Expected ';' after expression.".to_string(),
            )?;
            return Ok(Statement::Print(Expression::Literal(cur_token.literal)));
        }
        let expr = self.expression()?;
        self.consume(
            TokenType::SEMICOLON,
            "Expected ';' after expression.".to_string(),
        )?;
        return Ok(Statement::Print(expr));
    }

    fn expression_statement(&mut self) -> Result<Statement, LoxError> {
        let expr = self.expression()?;
        self.consume(
            TokenType::SEMICOLON,
            "Expected ';' after expression.".to_string(),
        )?;
        return Ok(Statement::Expression(expr));
    }

    fn expression(&mut self) -> Result<Expression, LoxError> {
        return self.equality();
    }

    pub fn equality(&mut self) -> Result<Expression, LoxError> {
        let mut expr = self.comparison()?;

        while self.consume_if_type(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    pub fn comparison(&mut self) -> Result<Expression, LoxError> {
        let mut expr = self.term()?;

        while self.consume_if_type(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    pub fn term(&mut self) -> Result<Expression, LoxError> {
        let mut expr = self.factor()?;

        while self.consume_if_type(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    pub fn factor(&mut self) -> Result<Expression, LoxError> {
        let mut expr = self.unary()?;

        while self.consume_if_type(&[TokenType::SLASH, TokenType::STAR, TokenType::EXPONENT]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    pub fn unary(&mut self) -> Result<Expression, LoxError> {
        if self.consume_if_type(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            });
        }

        return self.primary();
    }

    pub fn primary(&mut self) -> Result<Expression, LoxError> {
        if self.consume_if_type(&[TokenType::FALSE]) {
            return Ok(Expression::Literal(TokenLiteral::Boolean(false)));
        }
        if self.consume_if_type(&[TokenType::TRUE]) {
            return Ok(Expression::Literal(TokenLiteral::Boolean(true)));
        }

        if self.consume_if_type(&[TokenType::NIL]) {
            return Ok(Expression::Literal(TokenLiteral::Empty));
        }

        if self.consume_if_type(&[TokenType::NUMBER, TokenType::STRING]) {
            let prev = self.previous();
            return Ok(Expression::Literal(prev.literal));
        }

        if self.consume_if_type(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RIGHT_PAREN,
                "Expected ')' after expression.".to_string(),
            )?;
            return Ok(Expression::Grouping(Box::new(expr)));
        }

        if self.peek().token_type == TokenType::IDENTIFIER {
            return Ok(Expression::Literal(self.peek().literal));
        }

        return Err(LoxError::ParseError(ParserError::Generic(
            "Expression Expected".to_string(),
        )));
    }

    pub fn previous(&mut self) -> Token {
        return self.scanner.tokens[self.current - 1].clone();
    }

    pub fn consume_if_type(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(*token_type) {
                self.next();
                return true;
            }
        }
        return false;
    }

    pub fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type;
    }

    pub fn is_at_end(&mut self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }

    pub fn next(&mut self) -> Token {
        if self.is_at_end() {
            return Token {
                token_type: TokenType::EOF,
                lexeme: String::new(),
                literal: TokenLiteral::Empty,
                line: self.line,
            };
        }

        self.current += 1;
        let tok = self.scanner.tokens[self.current - 1].clone();

        return tok;
    }

    pub fn peek(&mut self) -> Token {
        if self.current >= self.scanner.tokens.len() {
            return Token {
                token_type: TokenType::EOF,
                lexeme: String::new(),
                literal: TokenLiteral::Empty,
                line: self.line,
            };
        }

        return self.scanner.tokens[self.current].clone();
    }

    pub fn peek_next(&mut self) -> Token {
        if self.is_at_end() {
            return Token {
                token_type: TokenType::EOF,
                lexeme: String::new(),
                literal: TokenLiteral::Empty,
                line: self.line,
            };
        }

        return self.scanner.tokens[self.current].clone();
    }

    pub fn consume(&mut self, token_type: TokenType, err_msg: String) -> Result<Token, LoxError> {
        if self.check(token_type) {
            return Ok(self.next());
        }
        let cur_token = self.peek();
        self.err(cur_token, err_msg.clone());

        return Err(LoxError::ParseError(ParserError::Generic(
            err_msg.to_string(),
        )));
    }

    pub fn err(&mut self, token: Token, msg: String) {
        self.had_error = true;
        if token.token_type == TokenType::EOF {
            self.report(token.line, "at end".to_string(), msg.to_string());
        } else {
            self.report(
                token.line,
                format!("at '{}'", token.lexeme),
                msg.to_string(),
            );
        }
        self.synchronize();
    }

    fn synchronize(&mut self) {
        self.next();
        while !self.is_at_end() {
            println!("Skipping {:?}", self.peek().token_type);
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }
            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => {
                    return;
                }
                _ => self.next(),
            };
        }
    }

    pub fn report(&self, line: usize, loc: String, msg: String) {
        eprintln!("[line {}] Error {}: {}", line, loc, msg);
    }
}
