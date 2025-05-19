use crate::ast::{keyword_token_type, LoxError, Token, TokenLiteral, TokenType};

#[derive(Debug, Clone)]
pub struct Scanner {
    pub had_error: bool,
    pub source: Vec<char>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub tokens: Vec<Token>,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            line: 1,
            had_error: false,
            source: vec![],
            start: 0,
            current: 0,
            tokens: vec![],
        }
    }
}

impl Scanner {
    ///loads source and scans it for tokens
    pub fn load(&mut self, source: Vec<char>) {
        self.source.extend(source);
        self.scan_tokens();
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token().expect("Failed to scan token");
        }
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.next();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, TokenLiteral::Empty),

            ')' => self.add_token(TokenType::RIGHT_PAREN, TokenLiteral::Empty),

            '{' => self.add_token(TokenType::LEFT_BRACE, TokenLiteral::Empty),

            '}' => self.add_token(TokenType::RIGHT_BRACE, TokenLiteral::Empty),

            ',' => self.add_token(TokenType::COMMA, TokenLiteral::Empty),

            '.' => self.add_token(TokenType::DOT, TokenLiteral::Empty),

            '-' => self.add_token(TokenType::MINUS, TokenLiteral::Empty),

            '+' => self.add_token(TokenType::PLUS, TokenLiteral::Empty),

            ';' => self.add_token(TokenType::SEMICOLON, TokenLiteral::Empty),

            '^' => self.add_token(TokenType::EXPONENT, TokenLiteral::Empty),

            '*' => {
                let tok_type = if self.consume_if_next('*') {
                    TokenType::EXPONENT
                } else {
                    TokenType::STAR
                };
                self.add_token(tok_type, TokenLiteral::Empty);
            }

            '!' => {
                let tok_type = if self.consume_if_next('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(tok_type, TokenLiteral::Empty);
            }

            '=' => {
                let tok_type = if self.consume_if_next('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(tok_type, TokenLiteral::Empty);
            }

            '<' => {
                let tok_type = if self.consume_if_next('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(tok_type, TokenLiteral::Empty);
            }

            '>' => {
                let tok_type = if self.consume_if_next('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(tok_type, TokenLiteral::Empty);
            }

            '/' => {
                if self.consume_if_next('/') {
                    self.comment();
                } else if self.consume_if_next('*') {
                    self.block_comment();
                } else {
                    self.add_token(TokenType::SLASH, TokenLiteral::Empty)
                }
            }

            '"' | '\'' => {
                self.string(c);
            }

            ' ' | '\r' | '\t' => {}

            '\n' => {
                self.line += 1;
            }

            c => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    self.err(self.line, &format!("Unexpected character: {}", c));
                    return Err(LoxError::ScanError(c));
                }
            }
        };
        return Ok(());
    }

    fn consume_if_next(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != c {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn number(&mut self) {
        let mut is_float = false;

        while self.peek().is_ascii_digit() {
            self.next();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            is_float = true;
            self.next();
        }

        while self.peek().is_ascii_digit() {
            self.next();
        }
        if is_float {
            self.add_token(
                TokenType::NUMBER,
                TokenLiteral::Float(
                    self.source[self.start..self.current]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap_or_default(),
                ),
            );
        } else {
            self.add_token(
                TokenType::NUMBER,
                TokenLiteral::Integer(
                    self.source[self.start..self.current]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap_or_default(),
                ),
            );
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.next();
        }

        let ident: String = self.source[self.start..self.current].iter().collect();

        if let Some(idm) = keyword_token_type(&ident) {
            self.add_token(idm, TokenLiteral::Empty);
        } else {
            self.add_token(TokenType::IDENTIFIER, TokenLiteral::String(ident));
        }
    }

    fn string(&mut self, c: char) {
        while self.peek() != c && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.next();
        }
        if self.is_at_end() {
            println!("{}", &self.source.iter().collect::<String>());
            self.err(self.line, "Unterminated string");
        }

        self.next();

        let lit = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect::<String>();
        self.add_token(TokenType::STRING, TokenLiteral::String(lit));
    }

    fn comment(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.next();
        }
        self.add_token(
            TokenType::COMMENT,
            TokenLiteral::String(
                self.source[self.start + 2..self.current]
                    .iter()
                    .collect::<String>(),
            ),
        );
    }

    fn block_comment(&mut self) {
        while self.peek() != '*' && self.peek_next() == '/' && self.is_at_end() {
            self.next();
        }
        if self.is_at_end() {
            self.err(self.line, "Unterminated block comment!");
            return;
        }
        self.current += 2;
        self.add_token(
            TokenType::BLOCK_COMMENT,
            TokenLiteral::String(
                self.source[self.start + 2..self.current - 2]
                    .iter()
                    .collect::<String>(),
            ),
        );
    }

    fn add_token(&mut self, token_type: TokenType, literal: TokenLiteral) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: text.iter().collect(),
            literal,
            line: self.line,
        });
    }

    fn next(&mut self) -> char {
        if self.current >= self.source.len() {
            return 0x00 as char;
        }
        self.current += 1;
        return self.source[self.current - 1];
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return 0x00.into();
        }
        return self.source[self.current];
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return 0x00.into();
        }
        return self.source[self.current + 1];
    }

    fn err(&mut self, line: usize, msg: &str) {
        self.report(line, "", msg)
    }
    fn report(&mut self, line: usize, loc: &str, msg: &str) {
        eprintln!("[line: {}] Error {}: {}", line, loc, msg);
        self.had_error = true;
    }
}
