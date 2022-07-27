use std::cmp;

use self::ast::{HackInstruction, Statement, Comp, CInstruction};
use crate::scanner::token::{self, Token};

mod ast;

struct ParsingError {}
struct Parser {
    tokens: Vec<Token>,
    curr: usize,
    errors: Vec<ParsingError>,
    statements: Vec<Statement>,
}



impl Parser {
    const MAX_ADDRESS: usize = 32767;
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            curr: 0,
            errors: Vec::new(),
            statements: Vec::new(),
        }
    }

    pub fn statement(&mut self) -> Option<Statement> {
        match self.peek().kind {
            token::TokenKind::LeftParen => unimplemented!(),
            _ => self.instruction().map_or(None, |i| Some(Statement::Instruction(i))),
        }
    }

    fn instruction(&mut self) -> Option<HackInstruction> {
        if self.check(token::TokenKind::At) {
            self.advance();
            self.a_instruction()
        } else {
            self.c_instruction()
        }
    }

    fn a_instruction(&mut self) -> Option<HackInstruction> {
        let token = self.token();
        match token.kind {
            token::TokenKind::Identifier(..) => Some(HackInstruction::AInstruction(token.clone())),
            token::TokenKind::Number(num) => {
                if num > Self::MAX_ADDRESS {
                    return None; //todo raise an error
                }
                Some(HackInstruction::AInstruction(token.clone()))
            }
            _ => None, //todo raise an error
        }
    }

    fn c_instruction(&mut self) -> Option<HackInstruction> {
        let dest = if self.check(token::TokenKind::Equals) {
            self.advance();
            Some(self.previous().clone())
        } else {
            None
        };
        let comp = self.comp()?;
        let jump = if self.check(token::TokenKind::Semicolon) {
            self.advance();
            Some(self.previous().clone())
        } else {
            None
        };
        Some(HackInstruction::CInstruction(CInstruction::new(dest, comp, jump)))
    }

    fn comp(&mut self) -> Option<Comp> {
        let comp_len = cmp::min(3, self.tokens.len() - self.curr);
let tokens = &self.tokens[self.curr..self.curr+comp_len].iter().map(|token| token.kind.clone()).collect::<Vec<token::TokenKind>>();
        Comp::from_tokens(tokens)

    }

    pub fn is_at_end(&self) -> bool {
        self.check(token::TokenKind::Eof)
    }
    pub fn check(&self, token_kind: token::TokenKind) -> bool {
        matches!(&self.peek().kind, x if *x == token_kind)
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.curr]
    }

    pub fn previous(&self) -> &Token {
        &self.tokens[self.curr - 1]
    }

    pub fn token(&self) -> &Token {
        &self.tokens[self.curr]
    }

    pub fn advance(&mut self) {
        if !self.is_at_end() {
            self.curr += 1;
        }
    }
}
