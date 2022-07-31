use std::fs;

use crate::{Result, Error, error_formatting::ErrorFormatter, parser::Parser};

use self::token::Token;
pub mod token;


pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    curr: usize,
    start: usize,
    line: usize,
    errors: Vec<Error>,
}

impl Scanner {
    pub fn with_source(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            curr: 0,
            line: 1,
            start: 0,
            errors: Vec::new(),
        }
    }

    pub fn from_path(path: &str) -> crate::Result<Scanner> {
        let source = fs::read_to_string(path)?;
        Ok(Scanner::with_source(source))
    }

    fn is_at_end(&self) -> bool {
        self.curr >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let char = self.source.as_bytes()[self.curr];
        self.curr += 1;
        char
    }

    fn add_token(&mut self, token_type: token::TokenKind) {
        let token = Token::new(token_type, self.line, self.start);
        self.tokens.push(token);
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source.as_bytes()[self.curr]
        }
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.curr] != expected {
            return false;
        }
        self.curr += 1;
        true
    }
    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        let literal: usize = self.curr_lexeme().parse().unwrap();
        self.add_token(token::TokenKind::Number(literal));
    }

    fn curr_lexeme(&self) -> &str {
        &self.source[self.start..self.curr]
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        let token_type = match self.curr_lexeme() {
            "M" => token::TokenKind::M,
            "D" => token::TokenKind::D,
            "MD" => token::TokenKind::Md,
            "A" => token::TokenKind::A,
            "AM" => token::TokenKind::Am,
            "AD" => token::TokenKind::Ad,
            "AMD" => token::TokenKind::AMd,
            "JGT" => token::TokenKind::Jgt,
            "JEQ" => token::TokenKind::Jeq,
            "JGE" => token::TokenKind::Jge,
            "JLT" => token::TokenKind::Jlt,
            "JNE" => token::TokenKind::Jne,
            "JLE" => token::TokenKind::Jle,
            "JMP" => token::TokenKind::Jmp,
            _ => token::TokenKind::Identifier(self.curr_lexeme().to_string()),
        };
        self.add_token(token_type);
    }

    fn skip_comment(&mut self) {
        while self.peek() != b'\n' && !self.is_at_end() {
            self.advance();
        }
    }

    pub fn scan_token(&mut self) {
        let char = self.advance();
        match char {
            b'@' => self.add_token(token::TokenKind::At),
            b'=' => self.add_token(token::TokenKind::Equals),
            b'+' => self.add_token(token::TokenKind::Plus),
            b'-' => self.add_token(token::TokenKind::Minus),
            b'|' => self.add_token(token::TokenKind::Or),
            b'&' => self.add_token(token::TokenKind::And),
            b'!' => self.add_token(token::TokenKind::Not),
            b'(' => self.add_token(token::TokenKind::LeftParen),
            b')' => self.add_token(token::TokenKind::RightParen),
            b'/' => {
                if self.match_next(b'/') {
                    self.skip_comment();
                } else {
                    self.raise_error("Unexpected character, did you mean '//'?")
                }
            }
            b';' => self.add_token(token::TokenKind::Semicolon),
            b'\n' => {
                self.line += 1;
                self.add_token(token::TokenKind::NewLine);
            }
            b' ' | b'\r' | b'\t' => {}
            _ => {
                if char.is_ascii_digit() {
                    self.number();
                } else if char.is_ascii_alphabetic() {
                    self.identifier()
                } else {
                    self.raise_error("Unexpected character")
                }
            }
        }
    }

    pub fn scan_tokens(&mut self) -> bool {
        while !self.is_at_end() {
            self.start = self.curr;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(token::TokenKind::Eof, self.line, self.start));

        self.errors.is_empty()
    }

    pub fn run(mut self) -> Result<Parser>{
        if self.scan_tokens() {
            Ok(self.into_parser())
        } else {
            self.print_errors();
            Err(Error::from(format!("Encountered {} errors, aborting compilation", self.errors.len())))
        }
    }

    pub fn had_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn raise_error(&mut self, message: &str) {
        self.errors.push(ErrorFormatter::gen_err(
            message,
            &self.source,
            self.start,
            self.curr - self.start,
            self.line,
        ));
    }

    pub fn print_errors(&self) {
        if !self.had_errors() {
            return;
        }
        for error in &self.errors[..self.errors.len() - 1] {
            eprintln!("{}\n", error);
        }
        eprintln!("{}", self.errors[self.errors.len() - 1]);
        eprintln!("Encountered {} errors, aborting", self.errors.len());
    }

    fn into_parser(self) -> Parser{
        Parser::new(self.tokens, self.source)
    }
}
