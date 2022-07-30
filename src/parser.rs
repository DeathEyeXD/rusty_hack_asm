use std::cmp;

use self::ast::{CInstruction, Comp, HackInstruction, Statement};
use crate::{
    error_formatting::ErrorFormatter,
    scanner::token::{self, Token},
    Error,
};

mod ast;

struct ParsingError {}
struct Parser {
    source: String,
    tokens: Vec<Token>,
    curr: usize,
    errors: Vec<Error>,
    statements: Vec<Statement>,
}

impl Parser {
    const MAX_ADDRESS: usize = 32767;
    pub fn new(tokens: Vec<Token>, source: String) -> Parser {
        Parser {
            tokens,
            source,
            curr: 0,
            errors: Vec::new(),
            statements: Vec::new(),
        }
    }

    pub fn statement(&mut self) -> Option<Statement> {
        match self.peek().kind {
            token::TokenKind::LeftParen => unimplemented!(),
            _ => self.instruction().map(Statement::Instruction),
        }
    }

    fn instruction(&mut self) -> Option<HackInstruction> {
        let ins = if self.check(token::TokenKind::At) {
            self.a_instruction()
        } else {
            self.c_instruction()
        };

        if let Some(ins) = ins {
            self.consume(
                "Unexpected token after instruction",
                token::TokenKind::NewLine,
            );
            Some(ins)
        } else {
            self.synchronise();
            None
        }
    }

    fn a_instruction(&mut self) -> Option<HackInstruction> {
        self.advance(); //skip @ token
        match self.token().kind {
            token::TokenKind::Identifier(..) => {}
            token::TokenKind::Number(num) => {
                if num > Self::MAX_ADDRESS {
                    self.raise_error_curr(
                        &format!(
                        "Address out of range, max allowed value is {}",
                            Self::MAX_ADDRESS
                    ) //its not a unrecoverable error, so we don't return None, but we report the error so it won't compile
                );
                }
            }
            _ => {
                self.raise_error_curr("Expected identifier or number after '@'");
                return None;
            } //todo raise an error
        }
        Some(HackInstruction::AInstruction(self.token().clone()))
    }

    fn consume(&mut self, msg: &str, expected: token::TokenKind) -> Option<&Token> {
        if expected == self.peek().kind {
            self.advance();
            Some(self.previous())
        } else {
            self.raise_error_peak(msg);
            None
        }
    }

    fn synchronise(&mut self) {
        while !self.check(token::TokenKind::NewLine) {
            self.advance();
        }
        self.advance(); //skip newline token
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
        Some(HackInstruction::CInstruction(CInstruction::new(
            dest, comp, jump,
        )))
    }

    fn comp(&mut self) -> Option<Comp> {
        let comp_len = cmp::min(3, self.tokens.len() - self.curr);
        let tokens = &self.tokens[self.curr..self.curr + comp_len]
            .iter()
            .map(|token| token.kind.clone())
            .collect::<Vec<token::TokenKind>>();
        Comp::from_tokens(tokens)
    }

    fn is_at_end(&self) -> bool {
        self.check(token::TokenKind::Eof)
    }
    fn check(&self, token_kind: token::TokenKind) -> bool {
        matches!(&self.peek().kind, x if *x == token_kind)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.curr + 1]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.curr - 1]
    }

    fn token(&self) -> &Token {
        &self.tokens[self.curr]
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.curr += 1;
        }
    }
    fn raise_error(&mut self, msg: &str, token_id: usize) {
        let token = &self.tokens[token_id];
        self.errors
            .push(ErrorFormatter::err_from_token(msg, &self.source, token));
    }
    fn raise_error_curr(&mut self, msg: &str) {
        self.raise_error(msg, self.curr);
    }
    fn raise_error_peak(&mut self, msg: &str) {
        self.raise_error(msg, self.curr + 1);
    }
}
