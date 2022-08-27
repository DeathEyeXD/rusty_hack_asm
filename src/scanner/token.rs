use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TokenKind<'a> {
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
    Amd,

    Jgt,
    Jeq,
    Jge,
    Jlt,
    Jne,
    Jle,
    Jmp,

    // identifiers
    Identifier(&'a str),

    // Literals
    Number(u16, u8),
}

impl<'a> TokenKind<'a> {
    pub fn is_jump_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Jgt
                | TokenKind::Jeq
                | TokenKind::Jge
                | TokenKind::Jlt
                | TokenKind::Jne
                | TokenKind::Jle
                | TokenKind::Jmp
        )
    }

    pub fn is_dest_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::M
                | TokenKind::D
                | TokenKind::Md
                | TokenKind::A
                | TokenKind::Am
                | TokenKind::Ad
                | TokenKind::Amd
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub line: usize,
    pub start: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenKind, line: usize, start: usize) -> Token {
        Token {
            kind: token_type,
            line,
            start,
        }
    }
    pub fn as_bin_code(&self) -> String {
        match self.kind {
            // A-bit, D-bit, M-bit
            TokenKind::M => "001".to_string(),
            TokenKind::D => "010".to_string(),
            TokenKind::Md => "011".to_string(),
            TokenKind::A => "100".to_string(),
            TokenKind::Am => "101".to_string(),
            TokenKind::Ad => "110".to_string(),
            TokenKind::Amd => "111".to_string(),

            TokenKind::Jgt => "001".to_string(),
            TokenKind::Jeq => "010".to_string(),
            TokenKind::Jge => "011".to_string(),
            TokenKind::Jlt => "100".to_string(),
            TokenKind::Jne => "101".to_string(),
            TokenKind::Jle => "110".to_string(),
            TokenKind::Jmp => "111".to_string(),
            TokenKind::Number(n, _) => format!("{:#016b}", n),
            _ => panic!("{}", format!("Cannot convert '{}' to heck binary", self)),
        }
    }

    pub fn len(&self) -> usize {
        match self.kind {
            TokenKind::Number(_, len) => len as usize,
            TokenKind::Identifier(s) => s.len(),
            TokenKind::Eof => 0,
            TokenKind::NewLine
            | TokenKind::A
            | TokenKind::D
            | TokenKind::M
            | TokenKind::Semicolon
            | TokenKind::LeftParen
            | TokenKind::RightParen
            | TokenKind::At
            | TokenKind::Equals
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Not => 1,
            TokenKind::Md | TokenKind::Am | TokenKind::Ad => 2,
            TokenKind::Amd
            | TokenKind::Jgt
            | TokenKind::Jeq
            | TokenKind::Jge
            | TokenKind::Jlt
            | TokenKind::Jne
            | TokenKind::Jle
            | TokenKind::Jmp => 3,
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "token<")?;
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
            TokenKind::Amd => write!(f, "AMd"),
            TokenKind::Jgt => write!(f, "Jgt"),
            TokenKind::Jeq => write!(f, "Jeq"),
            TokenKind::Jge => write!(f, "Jge"),
            TokenKind::Jlt => write!(f, "Jlt"),
            TokenKind::Jne => write!(f, "Jne"),
            TokenKind::Jle => write!(f, "Jle"),
            TokenKind::Jmp => write!(f, "Jmp"),
            TokenKind::Identifier(s) => write!(f, "Identifier: {}", s),
            TokenKind::Number(n, _) => write!(f, "Number: {}", n),
        }?;
        write!(f, ", start {}>", self.start)
    }
}
