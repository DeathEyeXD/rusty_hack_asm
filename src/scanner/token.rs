use std::fmt::Display;

#[derive(Clone, PartialEq, Eq)]
pub enum TokenKind{
    //Symbols
    LeftParen,
    RightParen,
     
    Semicolon,
    // Artificial
    Eof,
    NewLine,
    // operators
    At,
    Equals,
    Plus,
    Minus,
    And,
    Or,
    Not,

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

#[derive(Clone, PartialEq, Eq)]
pub struct Token{
    pub kind: TokenKind,
    pub line: usize,
    pub start: usize,
}

impl Token{
    pub fn new(token_type: TokenKind, line: usize, start: usize) -> Token{
        Token{
            kind: token_type,
            line,
            start,
        }
    }
    pub fn to_binary(&self) -> String{
        match self.kind{
            TokenKind::M => "111".to_string(),
            TokenKind::D => "110".to_string(),
            TokenKind::Md => "111".to_string(),
            TokenKind::A => "100".to_string(),
            TokenKind::Am => "101".to_string(),
            TokenKind::Ad => "110".to_string(),
            TokenKind::AMd => "111".to_string(),
            TokenKind::Jgt => "001".to_string(),
            TokenKind::Jeq => "010".to_string(),
            TokenKind::Jge => "011".to_string(),
            TokenKind::Jlt => "100".to_string(),
            TokenKind::Jne => "101".to_string(),
            TokenKind::Jle => "110".to_string(),
            TokenKind::Jmp => "111".to_string(),
            TokenKind::Number(n) => format!("{:#016b}", n),
            _ => panic!("{}", format!("Cannot convert '{}' to heck binary", self)),
        }
    }
}

impl Display for Token{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"token<")?;
        match &self.kind {
            TokenKind::LeftParen => write!(f, "LeftParen"),
            TokenKind::RightParen => write!(f, "RightParen"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::NewLine => write!(f, "NewLine"),
            TokenKind::At => write!(f, "At"),
            TokenKind::Equals => write!(f, "Equals"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::And => write!(f, "And"),
            TokenKind::Or => write!(f, "Or"),
            TokenKind::Not => write!(f, "Not"),
            TokenKind::M => write!(f, "M"),
            TokenKind::D => write!(f, "D"),
            TokenKind::Md => write!(f, "Md"),
            TokenKind::A => write!(f, "A"),
            TokenKind::Am => write!(f, "Am"),
            TokenKind::Ad => write!(f, "Ad"),
            TokenKind::AMd => write!(f, "AMd"),
            TokenKind::Jgt => write!(f, "Jgt"),
            TokenKind::Jeq => write!(f, "Jeq"),
            TokenKind::Jge => write!(f, "Jge"),
            TokenKind::Jlt => write!(f, "Jlt"),
            TokenKind::Jne => write!(f, "Jne"),
            TokenKind::Jle => write!(f, "Jle"),
            TokenKind::Jmp => write!(f, "Jmp"),
            TokenKind::Identifier(s) => write!(f, "Identifier: {}", s),
            TokenKind::Number(n) => write!(f, "Number: {}", n),
        
        }?;
        write!(f, ">")
    }
}