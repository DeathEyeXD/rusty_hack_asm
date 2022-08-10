use crate::scanner::token::{self, Token};

#[derive(Debug)]
pub enum HackInstruction {
    AInstruction(AInstruction),
    CInstruction(CInstruction),
}

#[derive(Debug)]
pub enum AInstruction {
    Number(usize),
    Identifier(String),
}


#[derive(Debug)]
pub struct CInstruction {
    dest: Option<Token>,
    comp: Comp,
    jump: Option<Token>,
}

impl CInstruction {
    pub fn new(dest: Option<Token>, comp: Comp, jump: Option<Token>) -> Self {
        Self { dest, comp, jump }
    }
}

#[derive(Debug)]
pub enum Comp {
    Zero,
    One,
    MinusOne,
    D,
    A,
    NotD,
    NotA,
    MinusD,
    MinusA,
    DPlusOne,
    APlusOne,
    DMinusOne,
    AMinusOne,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,

    M,
    NotM,
    MinusM,
    MPlusOne,
    MMinusOne,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

impl Comp {
    const TOKEN_ONE: token::TokenKind = token::TokenKind::Number(1);
    const TOKEN_ZERO: token::TokenKind = token::TokenKind::Number(0);

    pub fn to_binary(&self) -> String {
        match self {
            Comp::Zero => "0101010",
            Comp::One => "0111111",
            Comp::MinusOne => "0111010",
            Comp::D => "0001100",
            Comp::A => "0110000",
            Comp::NotD => "0001101",
            Comp::NotA => "0110001",
            Comp::MinusD => "0001111",
            Comp::MinusA => "0110011",
            Comp::DPlusOne => "0011111",
            Comp::APlusOne => "0110111",
            Comp::DMinusOne => "0001110",
            Comp::AMinusOne => "0110010",
            Comp::DPlusA => "0000010",
            Comp::DMinusA => "0010011",
            Comp::AMinusD => "0000111",
            Comp::DAndA => "0000000",
            Comp::DOrA => "0010101",
            Comp::M => "1110000",
            Comp::NotM => "1110001",
            Comp::MinusM => "1110011",
            Comp::MPlusOne => "1110111",
            Comp::MMinusOne => "1110010",
            Comp::DPlusM => "1000010",
            Comp::DMinusM => "1010011",
            Comp::MMinusD => "1000111",
            Comp::DAndM => "1000000",
            Comp::DOrM => "1010101",
        }
        .to_string()
    }

    pub fn from_tokens(tokens: &[token::TokenKind]) -> Option<Self> {
        match tokens {
            [token::TokenKind::D, token::TokenKind::Or, token::TokenKind::A, ..] => Some(Comp::DOrA),
            [token::TokenKind::D, token::TokenKind::And, token::TokenKind::A, ..] => {
                Some(Comp::DAndA)
            }
            [token::TokenKind::A, token::TokenKind::Minus, token::TokenKind::D, ..] => {
                Some(Comp::AMinusD)
            }
            [token::TokenKind::D, token::TokenKind::Minus, token::TokenKind::A, ..] => {
                Some(Comp::DMinusA)
            }
            [token::TokenKind::D, token::TokenKind::Plus, token::TokenKind::A, ..] => {
                Some(Comp::DPlusA)
            }
            [token::TokenKind::A, token::TokenKind::Minus, Self::TOKEN_ONE, ..] => {
                Some(Comp::AMinusOne)
            }
            [token::TokenKind::D, token::TokenKind::Minus, Self::TOKEN_ONE, ..] => {
                Some(Comp::DMinusOne)
            }
            [token::TokenKind::A, token::TokenKind::Plus, Self::TOKEN_ONE, ..] => {
                Some(Comp::APlusOne)
            }
            [token::TokenKind::D, token::TokenKind::Plus, Self::TOKEN_ONE, ..] => {
                Some(Comp::DPlusOne)
            }
            [token::TokenKind::Minus, token::TokenKind::A, ..] => Some(Comp::MinusA),
            [token::TokenKind::Minus, token::TokenKind::D, ..] => Some(Comp::MinusD),
            [token::TokenKind::Not, token::TokenKind::A, ..] => Some(Comp::NotA),
            [token::TokenKind::Not, token::TokenKind::D, ..] => Some(Comp::NotD),
            [token::TokenKind::Minus, Self::TOKEN_ONE, ..] => Some(Comp::MinusOne),

            [token::TokenKind::D, token::TokenKind::Or, token::TokenKind::M, ..] => Some(Comp::DOrM),
            [token::TokenKind::D, token::TokenKind::And, token::TokenKind::M, ..] => {
                Some(Comp::DAndM)
            }
            [token::TokenKind::M, token::TokenKind::Minus, token::TokenKind::D, ..] => {
                Some(Comp::MMinusD)
            }
            [token::TokenKind::D, token::TokenKind::Minus, token::TokenKind::M, ..] => {
                Some(Comp::DMinusM)
            }
            [token::TokenKind::D, token::TokenKind::Plus, token::TokenKind::M, ..] => {
                Some(Comp::DPlusM)
            }
            [token::TokenKind::M, token::TokenKind::Minus, Self::TOKEN_ONE, ..] => {
                Some(Comp::MMinusOne)
            }
            [token::TokenKind::M, token::TokenKind::Plus, Self::TOKEN_ONE, ..] => {
                Some(Comp::MPlusOne)
            }

            [token::TokenKind::Minus, token::TokenKind::M, ..] => Some(Comp::MinusM),
            [token::TokenKind::Not, token::TokenKind::M, ..] => Some(Comp::NotM),
            [token::TokenKind::M, ..] => Some(Comp::M),
            [Self::TOKEN_ONE, ..] => Some(Comp::One),
            [Self::TOKEN_ZERO, ..] => Some(Comp::Zero),
            [token::TokenKind::A, ..] => Some(Comp::A),
            [token::TokenKind::D, ..] => Some(Comp::D),

            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Comp::Zero | Comp::One | Comp::A | Comp::M | Comp::D => 1,
            Comp::MinusOne
            | Comp::MinusA
            | Comp::MinusD
            | Comp::MinusM
            | Comp::NotA
            | Comp::NotD
            | Comp::NotM => 2,
            Comp::APlusOne
            | Comp::AMinusOne
            | Comp::DPlusOne
            | Comp::DMinusOne
            | Comp::MPlusOne
            | Comp::MMinusOne
            | Comp::DPlusA
            | Comp::DPlusM
            | Comp::DMinusM
            | Comp::DMinusA
            | Comp::AMinusD
            | Comp::MMinusD
            | Comp::DAndA
            | Comp::DOrA
            | Comp::DAndM
            | Comp::DOrM => 3,
        }
    }
}
impl HackInstruction {
    pub fn to_binary(&self) -> String {
        match self {
            HackInstruction::AInstruction(ins) => match ins{
                AInstruction::Identifier(_) => {
                    panic!("Internal error: cannot directly convert a instruction with an identifier to binary");
                }
                AInstruction::Number(val) => format!("0{:015b}", val),
            },
            HackInstruction::CInstruction(cinst) => {
                let mut binary = String::with_capacity(16);
                binary.push_str("111");
                binary.push_str(&cinst.comp.to_binary());
                binary.push_str(
                    &cinst
                        .dest
                        .as_ref()
                        .map_or(String::from("000"), |token| token.to_binary()),
                );
                binary.push_str(
                    &cinst
                        .jump
                        .as_ref()
                        .map_or(String::from("000"), |token| token.to_binary()),
                );
                binary
            }
        }
    }
}
