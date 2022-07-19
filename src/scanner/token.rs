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
    Identifier,

    // Literals
    Number(usize),
}

pub struct Token<'a>{
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
    pub start: usize,
}

impl Token<'_>{
    pub fn new(token_type: TokenType, lexeme: &str, line: usize, start: usize) -> Token{
        Token{
            token_type,
            lexeme,
            line,
            start,
        }
    }
}