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
        if self.consume_if_type(&[TokenType::FUN]) {
            return self.function();
        } else if self.consume_if_type(&[TokenType::VAR]) {
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
        if self.consume_if_type(&[TokenType::LEFT_BRACE]) {
            return Ok(Statement::Block(self.block()?));
        }
        if self.consume_if_type(&[TokenType::IF]) {
            return self.if_statement();
        }
        if self.consume_if_type(&[TokenType::WHILE]) {
            return self.while_statement();
        }
        if self.consume_if_type(&[TokenType::FOR]) {
            return self.for_statement();
        }
        if self.consume_if_type(&[TokenType::RETURN]) {
            return self.return_statement();
        }
        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Result<Statement, LoxError> {
        if self.peek().token_type == TokenType::IDENTIFIER {
            let cur_token = self.next();
            self.consume(
                TokenType::SEMICOLON,
                "Expected ';' after expression.".to_string(),
            )?;
            return Ok(Statement::Print(Expression::Variable(cur_token)));
        }
        let expr = self.expression()?;
        self.consume(
            TokenType::SEMICOLON,
            "Expected ';' after expression.".to_string(),
        )?;
        return Ok(Statement::Print(expr));
    }

    fn block(&mut self) -> Result<Vec<Statement>, LoxError> {
        let mut statements = vec![];
        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(
            TokenType::RIGHT_BRACE,
            "Expected '}' after block.".to_string(),
        )?;
        Ok(statements)
    }

    fn if_statement(&mut self) -> Result<Statement, LoxError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'if'.".to_string())?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after if condition.".to_string())?;
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.consume_if_type(&[TokenType::ELSE]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Statement::If(condition, then_branch, else_branch))
    }

    fn while_statement(&mut self) -> Result<Statement, LoxError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'while'.".to_string())?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after condition.".to_string())?;
        let body = Box::new(self.statement()?);
        Ok(Statement::While(condition, body))
    }

    fn for_statement(&mut self) -> Result<Statement, LoxError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'for'.".to_string())?;
        let initializer = if self.consume_if_type(&[TokenType::SEMICOLON]) {
            None
        } else if self.consume_if_type(&[TokenType::VAR]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };
        let condition = if !self.check(TokenType::SEMICOLON) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition.".to_string())?;
        let increment = if !self.check(TokenType::RIGHT_PAREN) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after for clauses.".to_string())?;
        let mut body = self.statement()?;
        if let Some(inc) = increment {
            body = Statement::Block(vec![body, Statement::Expression(inc)]);
        }
        let cond = condition.unwrap_or(Expression::Literal(TokenLiteral::Boolean(true)));
        body = Statement::While(cond, Box::new(body));
        if let Some(init) = initializer {
            body = Statement::Block(vec![init, body]);
        }
        Ok(body)
    }

    fn return_statement(&mut self) -> Result<Statement, LoxError> {
        let keyword = self.previous();
        let value = if !self.check(TokenType::SEMICOLON) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after return value.".to_string(),
        )?;
        Ok(Statement::Return(keyword, value))
    }

    fn function(&mut self) -> Result<Statement, LoxError> {
        let name = self.consume(TokenType::IDENTIFIER, "Expect function name.".to_string())?;
        self.consume(
            TokenType::LEFT_PAREN,
            "Expect '(' after function name.".to_string(),
        )?;
        let mut parameters = vec![];
        if !self.check(TokenType::RIGHT_PAREN) {
            loop {
                parameters.push(
                    self.consume(TokenType::IDENTIFIER, "Expect parameter name.".to_string())?,
                );
                if !self.consume_if_type(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after parameters.".to_string())?;
        self.consume(TokenType::LEFT_BRACE, "Expect '{' before function body.".to_string())?;
        let body = self.block()?;
        Ok(Statement::Function(name, parameters, body))
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
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<Expression, LoxError> {
        let expr = self.or()?;
        if self.consume_if_type(&[TokenType::EQUAL]) {
            let value = self.assignment()?;
            if let Expression::Variable(name) = expr {
                return Ok(Expression::Assign {
                    name,
                    value: Box::new(value),
                });
            }
            return Err(LoxError::ParseError(ParserError::Generic(
                "Invalid assignment target.".to_string(),
            )));
        }
        Ok(expr)
    }

    pub fn or(&mut self) -> Result<Expression, LoxError> {
        let mut expr = self.and()?;

        while self.consume_if_type(&[TokenType::OR]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    pub fn and(&mut self) -> Result<Expression, LoxError> {
        let mut expr = self.equality()?;

        while self.consume_if_type(&[TokenType::AND]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
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

        return self.call();
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, LoxError> {
        let mut arguments = vec![];
        if !self.check(TokenType::RIGHT_PAREN) {
            loop {
                arguments.push(self.expression()?);
                if !self.consume_if_type(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        let paren = self.consume(
            TokenType::RIGHT_PAREN,
            "Expect ')' after arguments.".to_string(),
        )?;
        Ok(Expression::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn call(&mut self) -> Result<Expression, LoxError> {
        let mut expr = self.primary()?;
        loop {
            if self.consume_if_type(&[TokenType::LEFT_PAREN]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
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
            let tok = self.next();
            return Ok(Expression::Variable(tok));
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
