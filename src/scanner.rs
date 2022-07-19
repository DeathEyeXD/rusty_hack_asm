use std::fs;

use self::token::Token;
mod token;

struct ScanningError{
    message: String,
}
impl ScanningError {
    fn new(message: &str, source: &str, start: usize, len: usize, line: usize) -> ScanningError {
        let highlight = Self::highlight(source, start, len, line);
        ScanningError{
            message:  format!("{}\n     {}", highlight, message),
        }
    }

    fn get_line_contents<'a>(source: &'a str, start: usize) -> (&'a str, usize){
        let start = source[..start].bytes().rev().position(|c| c == b'\n').unwrap_or(0);
        let end = source[start..].bytes().position(|c| c == b'\n').unwrap_or_else(|| start.saturating_sub(1));
        
        (&source[start..end], start)
    }

    fn highlight(source: &str, start: usize, len: usize, line: usize) -> String{
        let lexeme = &source[start..start+len];
        let (line_contents, line_start) = Self::get_line_contents(source, start);
        let padding_len = line / 100 + 4 + start - line_start; 
        format!("{} | {}\n", line, line_contents) + line_contents + &format!("{}{}--here"," ".repeat(padding_len) ,"^".repeat(len))
    }
}
impl std::fmt::Display for ScanningError{

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}",self.message)
    }

}

struct Scanner{
    source: String,
    tokens: Vec<Token>,
    curr: usize,
    start: usize,
    line: usize,
    errors: Vec<ScanningError>,
}

impl Scanner {

    fn with_source(source: String) -> Scanner{
        Scanner {
            source,
            tokens: Vec::new(),
            curr: 0,
            line: 1,
            start: 0,
            errors: Vec::new(),
        }
    }

    fn from_path(path: &str) -> crate::Result<Scanner> {
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

    fn add_token(& mut self, token_type: token::TokenType) {
        let token = Token::new(token_type, self.line, self.start);
        self.tokens.push(token);
    }

    fn peek(&self) -> u8{
        if self.is_at_end() {
            b'\0'
        } else {
            self.source.as_bytes()[self.curr]
        }
    }

    fn match_next(&mut self, expected: u8) -> bool{
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
        self.add_token(token::TokenType::Number(literal));
    }

    fn curr_lexeme(&self) -> &str {
        &self.source[self.start..self.curr]
    }

    fn identifier(&mut self){
        while self.peek().is_ascii_alphanumeric(){
            self.advance();
        }
        let token_type = match self.curr_lexeme()  {
            "M" => token::TokenType::M,
            "D" => token::TokenType::D,
            "MD" => token::TokenType::Md,
            "A" => token::TokenType::A,
            "AM" => token::TokenType::Am,
            "AD" => token::TokenType::Ad,
            "AMD" => token::TokenType::AMd,
            "JGT" => token::TokenType::Jgt,
            "JEQ" => token::TokenType::Jeq,
            "JGE" => token::TokenType::Jge,
            "JLT" => token::TokenType::Jlt,
            "JNE" => token::TokenType::Jne,
            "JLE" => token::TokenType::Jle,
            "JMP" => token::TokenType::Jmp,
            _ => token::TokenType::Identifier(self.curr_lexeme().to_string()),
        };
        self.add_token(token_type);
    }

    fn skip_comment(&mut self){
        while self.peek() != b'\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn scan_token(&mut self){
        let char = self.advance();
        match char {
            b'@' => self.add_token(token::TokenType::At),
            b'=' => self.add_token(token::TokenType::Equals),
            b'+' => self.add_token(token::TokenType::Plus),
            b'-' => self.add_token(token::TokenType::Minus),
            b'(' => self.add_token(token::TokenType::LeftParen),
            b')' => self.add_token(token::TokenType::RightParen),
            b'/' => {
                if self.match_next(b'/') {
                self.skip_comment();
                } else { 
                    self.raise_error("Unexpected character")
                }
            },
            b';' => self.add_token(token::TokenType::Semicolon),
            b'\n' => {
                self.line += 1;
            }
            b' ' | b'\r' | b'\t' => {}
            _ => {
                if char.is_ascii_digit() {
                    self.number();
                } else if char.is_ascii_alphabetic() {
                    self.identifier()
                } else{
                    //TODO raise an error
                }
            }
        }
    }

    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.curr;
            self.scan_token();
        }
        self.tokens.push(Token::new(token::TokenType::Eof, self.line, self.start));
        &self.tokens
    }

    fn had_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn raise_error(&mut self, message: &str){
        self.errors.push(ScanningError::new(message, &self.source, self.start, self.curr-self.start, self.line));
    }
}
