use std::cmp;
use std::collections::{HashMap};

use crate::ast::{AInstruction, CInstruction, Comp, HackInstruction};
use crate::scanner::token::TokenKind;
use crate::{
    error_formatting::ErrorFormatter,
    scanner::token::{self, Token},
    Error, Result,
};

pub struct Parser<'a> {
    source: &'a [&'a str],
    tokens: &'a [Token<'a>],
    curr: usize,
    next_ident_id: u16,
    errors: Vec<Error>,
    instructions: Vec<HackInstruction<'a>>,
    var_a_ins_indices: Vec<usize>,
    ident_map: HashMap<&'a str, u16>,
}

impl<'a> Parser<'a> {
    const MAX_ADDRESS: u16 = 32767;
    pub fn new(tokens: &'a [Token<'a>], source: &'a [&'a str]) -> Self {
        let identifier_map = Self::get_default_ident_map();

        Parser {
            tokens,
            source,
            curr: 0,
            next_ident_id: 16,
            errors: Vec::new(),
            instructions: Vec::new(),
            ident_map: identifier_map,
            var_a_ins_indices: Vec::new(),
        }
    }

    fn is_predefined_ident(&self, ident: &str) -> bool {
        matches!(
            ident,
            "SP" | "LCL"
                | "ARG"
                | "THIS"
                | "THAT"
                | "R0"
                | "R1"
                | "R2"
                | "R3"
                | "R4"
                | "R5"
                | "R6"
                | "R7"
                | "R8"
                | "R9"
                | "R10"
                | "R11"
                | "R12"
                | "R13"
                | "R14"
                | "R15"
                | "SCREEN"
                | "KBD"
        )
    }

    fn get_default_ident_map() -> HashMap<&'a str, u16> {
        let mut map = HashMap::with_capacity(23);
        map.insert("SP", 0);
        map.insert("LCL", 1);
        map.insert("ARG", 2);
        map.insert("THIS", 3);
        map.insert("THAT", 4);
        map.insert("R0", 0);
        map.insert("R1", 1);
        map.insert("R2", 2);
        map.insert("R3", 3);
        map.insert("R4", 4);
        map.insert("R5", 5);
        map.insert("R6", 6);
        map.insert("R7", 7);
        map.insert("R8", 8);
        map.insert("R9", 9);
        map.insert("R10", 10);
        map.insert("R11", 11);
        map.insert("R12", 12);
        map.insert("R13", 13);
        map.insert("R14", 14);
        map.insert("R15", 15);
        map.insert("SCREEN", 16384);
        map.insert("KBD", 24576);
        map
    }

    pub fn run(mut self) -> Result<Vec<HackInstruction<'a>>> {
        if !self.parse() {
            self.print_errors();
            return Err(Error::from(format!(
                "Encountered {}, errors, aborting compilation.",
                self.errors.len()
            )));
        }
        self.denote_variables();
        Ok(self.instructions)
    }

    fn denote_variables(&mut self) {
        for &id in self.var_a_ins_indices.iter() {
            if let HackInstruction::A(ins) = &mut self.instructions[id] {
                let old = std::mem::replace(ins, AInstruction::Number(0));
                if let AInstruction::Identifier(ident) = old {
                    let val = self.ident_map.entry(ident).or_insert_with(|| {
                        let index = self.next_ident_id;
                        self.next_ident_id += 1;
                        index
                    });
                    self.instructions[id] = HackInstruction::A(AInstruction::Number(*val));
                    continue;
                }
            }
            panic!(
                "Internal error: Unexpected instruction {:?}",
                self.instructions[id]
            );
        }
    }

    fn print_errors(&self) {
        for error in &self.errors {
            eprintln!("{}", error);
        }
    }

    pub fn parse(&mut self) -> bool {
        // true means succesfull parsing
        while !self.is_at_end() {
            self.statement();
        }
        self.errors.is_empty()
    }

    fn statement(&mut self) {
        let success = match self.peek().kind {
            token::TokenKind::LeftParen => self.label_declaration(),
            _ => {
                if let Some(ins) = self.instruction() {
                    self.instructions.push(ins);
                    true
                } else {
                    false
                }
            }
        };

        if !success
            || !self.consume_line_end(
                "Unexpected token after statement, use line feed to separate statements",
            )
        {
            self.synchronise();
        }
    }

    fn add_label_ident(&mut self, ident: &'a str) {
        if self.is_predefined_ident(ident) {
            self.raise_error_prev(&format!(
                "Identifier {} is predefined and cannot be redefined",
                ident,
            ));
        } else if self.ident_map.contains_key(ident) {
            self.raise_error_prev(&format!("Cannot declare label {} more than once", ident,));
        } else {
            self.ident_map.insert(ident, self.instructions.len() as u16);
        }
    }

    fn label_declaration(&mut self) -> bool {
        self.advance();
        if let Some(label) = self.consume_identifier("Expected label name after '('") {
            if self
                .consume(
                    "Expected ')' after label name",
                    token::TokenKind::RightParen,
                )
                .is_none()
            {
                return false;
            }
            self.add_label_ident(label);
            true
        } else {
            false
        }
    }

    fn instruction(&mut self) -> Option<HackInstruction<'a>> {
        if self.check(token::TokenKind::At) {
            self.a_instruction()
        } else {
            self.c_instruction()
        }
    }

    fn a_instruction(&mut self) -> Option<HackInstruction<'a>> {
        self.advance(); //skip @ token
        match self.peek().kind {
            token::TokenKind::Identifier(ident) => {
                self.advance();
                self.var_a_ins_indices.push(self.instructions.len());
                Some(HackInstruction::A(AInstruction::Identifier(ident)))
            }
            token::TokenKind::Number(num, _) => {
                self.advance();
                if num > Self::MAX_ADDRESS {
                    self.raise_error_prev(
                        &format!(
                            "Address out of range, max allowed value is {}",
                            Self::MAX_ADDRESS
                        ), //its not a error that requires synchronising, so we don't return None, but we report the error so it won't compile
                    );
                }
                Some(HackInstruction::A(AInstruction::Number(num)))
            }
            _ => {
                self.raise_error_peek("Expected identifier or number after '@'");
                None
            } //todo raise an error
        }
    }

    fn consume(&mut self, msg: &str, expected: token::TokenKind) -> Option<&Token> {
        if expected == self.peek().kind {
            self.advance();
            Some(self.previous())
        } else {
            self.raise_error_peek(msg);
            None
        }
    }

    fn consume_line_end(&mut self, msg: &str) -> bool {
        if self.check_line_end() {
            self.advance();
            true
        } else {
            self.raise_error_peek(msg);
            false
        }
    }

    fn check_line_end(&mut self) -> bool {
        matches!(
            self.peek().kind,
            token::TokenKind::NewLine | token::TokenKind::Eof
        )
    }

    fn consume_identifier(&mut self, msg: &str) -> Option<&'a str> {
        if let token::TokenKind::Identifier(identifier) = self.peek().kind {
            self.advance();
            Some(identifier)
        } else {
            self.raise_error_peek(msg);
            None
        }
    }

    fn synchronise(&mut self) {
        while !self.check_line_end() {
            self.advance();
        }
        self.advance(); //skip newline token
    }

    fn c_instruction(&mut self) -> Option<HackInstruction<'a>> {
        self.advance();
        let dest = if self.check(token::TokenKind::Equals) {
            if !self.previous().kind.is_dest_keyword() {
                self.raise_error_prev("Expected destination after '='");
                return None;
            }
            let t = &self.tokens[self.curr - 1];
            self.advance(); // skip '='
            self.advance();
            Some(t)
        } else {
            None
        };
        let comp = self.comp()?;
        let jump = if self.check(token::TokenKind::Semicolon) {
            self.advance(); //skip semicolon
            if !self.peek().kind.is_jump_keyword() {
                // semicolon is the previous token, and we report the error at next (newline) token so it appears after the semicolon
                self.raise_error_peek("Expected jump keyword after ';'");
                return None;
            }
            self.advance();
            let t = &self.tokens[self.curr - 1];
            Some(t)
        } else {
            None
        };
        Some(HackInstruction::C(CInstruction::new(dest, comp, jump)))
    }

    fn comp(&mut self) -> Option<Comp> {
        let start = self.curr - 1;
        let max_comp_len = cmp::min(3, self.tokens.len() - start);
        if max_comp_len == 0 {
            self.raise_comp_error(0);
            return None;
        }
        let tokens = &self.tokens[start..start + max_comp_len]
            .iter()
            .map(|token| token.kind)
            .collect::<Vec<token::TokenKind>>();

        if let Some(comp) = Comp::from_tokens(tokens) {
            for _ in 0..comp.len() - 1 {
                self.advance();
            }
            Some(comp)
        } else {
            self.raise_comp_error(max_comp_len);
            None
        }
    }

    fn raise_comp_error(&mut self, mut comp_len: usize) {
        let line = self.peek().line;
        let start = self.previous().start;
        if comp_len > 0
            && matches!(
                self.tokens[self.curr + comp_len - 2].kind,
                TokenKind::NewLine
            )
        {
            comp_len -= 1;
        }
        self.errors.push(ErrorFormatter::gen_err(
            "Expected proper computation in c-instruction",
            self.source,
            start,
            comp_len,
            line,
        ))
    }

    fn is_at_end(&self) -> bool {
        self.check(token::TokenKind::Eof)
    }
    fn check(&self, token_kind: token::TokenKind) -> bool {
        matches!(&self.peek().kind, x if *x == token_kind)
    }

    fn previous(&self) -> &Token<'a> {
        &self.tokens[self.curr - 1]
    }

    fn peek(&self) -> &Token<'a> {
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
            .push(ErrorFormatter::err_from_token(msg, self.source, token));
    }
    fn raise_error_prev(&mut self, msg: &str) {
        self.raise_error(msg, self.curr - 1);
    }
    fn raise_error_peek(&mut self, msg: &str) {
        self.raise_error(msg, self.curr);
    }
}
