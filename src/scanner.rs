use std::fs;

use crate::{error_formatting::ErrorFormatter, parser::Parser, Error, Result};

use self::token::Token;
pub mod token;

pub struct Scanner {
    source: Vec<String>,
    tokens: Vec<Token>,
    curr: usize,
    start: usize,
    line: usize,
    errors: Vec<Error>,
}

impl Scanner {
    pub fn with_source(source: String) -> Scanner {
        let source = source.lines().map(String::from).collect();
        Scanner {
            source,
            tokens: Vec::new(),
            curr: 0,
            line: 0,
            start: 0,
            errors: Vec::new(),
        }
    }

    fn advance_line(&mut self) {
        self.line += 1;
        self.start = 0;
        self.curr = 0;
    }

    pub fn from_path(path: &str) -> crate::Result<Scanner> {
        let source = fs::read_to_string(path)?;
        Ok(Scanner::with_source(source))
    }

    fn is_at_end(&self) -> bool {
        self.line >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let char = self.peek();
        self.curr += 1;
        char
    }

    fn add_token(&mut self, token_type: token::TokenKind) {
        let token = Token::new(token_type, self.line, self.start);
        self.tokens.push(token);
    }

    fn is_at_line_end(&self) -> bool {
        self.curr >= self.source[self.line].len()
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else if self.is_at_line_end() {
            b'\n'
        } else {
            self.source[self.line].as_bytes()[self.curr]
        }
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }
        self.advance();
        true
    }
    fn number(&mut self) {
        while self.peek().is_ascii_digit() || self.peek() == b'_'{
            self.advance();
        }
        let literal: usize = self.curr_lexeme().parse().unwrap();
        self.add_token(token::TokenKind::Number(literal));
    }

    fn curr_lexeme(&self) -> &str {
        &self.source[self.line][self.start..self.curr]
    }

    fn identifier(&mut self) {
        while matches!(self.peek(), b'_' | b'.' | b'$' | b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9') {
            self.advance();
        }
        let token_type = match self.curr_lexeme() {
            "M" => token::TokenKind::M,
            "D" => token::TokenKind::D,
            "MD" => token::TokenKind::Md,
            "A" => token::TokenKind::A,
            "AM" => token::TokenKind::Am,
            "AD" => token::TokenKind::Ad,
            "AMD" => token::TokenKind::Amd,
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
        while !self.is_at_line_end() && !self.is_at_end() {
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
                if let Some(t) = self.tokens.last() {
                    if t.kind != token::TokenKind::NewLine {
                        self.add_token(token::TokenKind::NewLine);
                    }
                }
                self.advance_line();
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
        };
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

    fn print_tokens(&self) {
        for token in &self.tokens {
            println!("{}", token);
            if token.kind == token::TokenKind::NewLine {
                println!();
            }
        }
    }

    pub fn run(mut self) -> Result<Parser> {
        if self.scan_tokens() {
            self.print_tokens();
            Ok(self.into_parser())
        } else {
            self.print_errors();
            Err(Error::from(format!(
                "Encountered {} errors, aborting compilation",
                self.errors.len()
            )))
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

    fn into_parser(self) -> Parser {
        Parser::new(self.tokens, self.source)
    }
}
