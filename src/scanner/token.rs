use std::fmt::Display;

pub enum TokenType{
    //Symbols
    LeftParen,
    RightParen,
     
    Semicolon,
    Eof,
    // operators
    At,
    Equals,
    Plus,
    Minus,

    // keywords
    M,
    D,
    Md,
    A,
    Am,
    Ad,
    AMd,

    Jgt,
    Jeq,
    Jge,
    Jlt,
    Jne,
    Jle,
    Jmp,

    // identifiers
    Identifier(String),

    // Literals
    Number(usize),
}

pub struct Token{
    pub token_type: TokenType,
    pub line: usize,
    pub start: usize,
}

impl Token{
    pub fn new(token_type: TokenType, line: usize, start: usize) -> Token{
        Token{
            token_type,
            line,
            start,
        }
    }
}

impl Display for Token{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.token_type {
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Eof => write!(f, "Eof"),
            TokenType::At => write!(f, "At"),
            TokenType::Equals => write!(f, "Equals"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::M => write!(f, "M"),
            TokenType::D => write!(f, "D"),
            TokenType::Md => write!(f, "Md"),
            TokenType::A => write!(f, "A"),
            TokenType::Am => write!(f, "Am"),
            TokenType::Ad => write!(f, "Ad"),
            TokenType::AMd => write!(f, "AMd"),
            TokenType::Jgt => write!(f, "Jgt"),
            TokenType::Jeq => write!(f, "Jeq"),
            TokenType::Jge => write!(f, "Jge"),
            TokenType::Jlt => write!(f, "Jlt"),
            TokenType::Jne => write!(f, "Jne"),
            TokenType::Jle => write!(f, "Jle"),
            TokenType::Jmp => write!(f, "Jmp"),
            TokenType::Identifier(s) => write!(f, "Identifier: {}", s),
            TokenType::Number(n) => write!(f, "Number: {}", n),
        }
    }
}