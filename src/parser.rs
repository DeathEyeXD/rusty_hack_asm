use std::cmp;
use std::collections::{HashMap, HashSet};

use crate::ast::{AInstruction, CInstruction, Comp, HackInstruction};
use crate::evaluator::Evaluator;
use crate::scanner::token::TokenKind;
use crate::{
    error_formatting::ErrorFormatter,
    scanner::token::{self, Token},
    Error, Result,
};

pub struct Parser {
    source: Vec<String>,
    tokens: Vec<Token>,
    curr: usize,
    next_ident_id: usize,
    errors: Vec<Error>,
    instructions: Vec<HackInstruction>,
    var_a_ins_indices: Vec<usize>,
    ident_map: HashMap<String, usize>,
    predefined_idents: HashSet<String>,
}

impl Parser {
    const MAX_ADDRESS: usize = 32767;
    pub fn new(tokens: Vec<Token>, source: Vec<String>) -> Parser {
        let predefined_idents = Self::get_predefined_idents();
        let mut identifier_map = Self::get_default_ident_map();

        Parser {
            tokens,
            source,
            curr: 0,
            next_ident_id: 16,
            errors: Vec::new(),
            instructions: Vec::new(),
            var_a_ins_indices: Vec::new(),
            predefined_idents,
            ident_map: identifier_map,
        }
    }

    fn get_predefined_idents() -> HashSet<String> {
        HashSet::from_iter(vec![
            "SP".to_string(),
            "LCL".to_string(),
            "ARG".to_string(),
            "THIS".to_string(),
            "THAT".to_string(),
            "R0".to_string(),
            "R1".to_string(),
            "R2".to_string(),
            "R3".to_string(),
            "R4".to_string(),
            "R5".to_string(),
            "R6".to_string(),
            "R7".to_string(),
            "R8".to_string(),
            "R9".to_string(),
            "R10".to_string(),
            "R11".to_string(),
            "R12".to_string(),
            "R13".to_string(),
            "R14".to_string(),
            "R15".to_string(),
            "SCREEN".to_string(),
            "KBD".to_string(),
        ])
    }

    fn get_default_ident_map() -> HashMap<String, usize> {
        let mut map = HashMap::with_capacity(23);
        map.insert("SP".to_string(), 0);
        map.insert("LCL".to_string(), 1);
        map.insert("ARG".to_string(), 2);
        map.insert("THIS".to_string(), 3);
        map.insert("THAT".to_string(), 4);
        map.insert("R0".to_string(), 0);
        map.insert("R1".to_string(), 1);
        map.insert("R2".to_string(), 2);
        map.insert("R3".to_string(), 3);
        map.insert("R4".to_string(), 4);
        map.insert("R5".to_string(), 5);
        map.insert("R6".to_string(), 6);
        map.insert("R7".to_string(), 7);
        map.insert("R8".to_string(), 8);
        map.insert("R9".to_string(), 9);
        map.insert("R10".to_string(), 10);
        map.insert("R11".to_string(), 11);
        map.insert("R12".to_string(), 12);
        map.insert("R13".to_string(), 13);
        map.insert("R14".to_string(), 14);
        map.insert("R15".to_string(), 15);
        map.insert("SCREEN".to_string(), 16384);
        map.insert("KBD".to_string(), 24576);
        map
    }

    pub fn run(mut self) -> Result<Evaluator> {
        if !self.parse() {
            self.print_errors();
            return Err(Error::from(format!(
                "Encountered {}, errors, aborting compilation.",
                self.errors.len()
            )));
        }
        self.denote_variables();
        for statement in &self.instructions {
            println!("{:?}", statement);
        }
        Ok(Evaluator::new(self.instructions))
    }

    fn denote_variables(&mut self) {
        for &id in self.var_a_ins_indices.iter() {
            if let HackInstruction::AInstruction(ins) = &mut self.instructions[id] {
                let old = std::mem::replace(ins, AInstruction::Number(0));
                if let AInstruction::Identifier(ident) = old {
                    let val = self.ident_map.entry(ident).or_insert_with(|| {
                        let index = self.next_ident_id;
                        self.next_ident_id += 1;
                        index
                    });
                    self.instructions[id] =
                        HackInstruction::AInstruction(AInstruction::Number(*val));
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

    pub fn had_errors(self) -> bool {
        !self.errors.is_empty()
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

    fn add_label_ident(&mut self, ident: String) {
        if self.predefined_idents.contains(&ident) {
            self.raise_error_prev(&format!(
                "Identifier {} is predefined and cannot be redefined",
                ident,
            ));
        } else if self.ident_map.contains_key(&ident) {
            self.raise_error_prev(&format!("Cannot declare label {} more than once", ident,));
        } else {
            self.ident_map.insert(ident, self.instructions.len());
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

    fn instruction(&mut self) -> Option<HackInstruction> {
        if self.check(token::TokenKind::At) {
            self.a_instruction()
        } else {
            self.c_instruction()
        }
    }

    fn a_instruction(&mut self) -> Option<HackInstruction> {
        self.advance(); //skip @ token
        match self.peek().kind {
            token::TokenKind::Identifier(ref ident) => {
                let ident = ident.clone();
                self.advance();
                self.var_a_ins_indices.push(self.instructions.len());
                Some(HackInstruction::AInstruction(AInstruction::Identifier(
                    ident,
                )))
            }
            token::TokenKind::Number(num) => {
                self.advance();
                if num > Self::MAX_ADDRESS {
                    self.raise_error_prev(
                        &format!(
                            "Address out of range, max allowed value is {}",
                            Self::MAX_ADDRESS
                        ), //its not a error that requires synchronising, so we don't return None, but we report the error so it won't compile
                    );
                }
                Some(HackInstruction::AInstruction(AInstruction::Number(num)))
            }
            _ => {
                self.raise_error_prev("Expected identifier or number after '@'");
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

    fn consume_identifier(&mut self, msg: &str) -> Option<String> {
        if let token::TokenKind::Identifier(identifier) = &self.peek().kind {
            let identifier = identifier.clone();
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

    fn c_instruction(&mut self) -> Option<HackInstruction> {
        self.advance();
        let dest = if self.check(token::TokenKind::Equals) {
            if !self.previous().kind.is_dest_keyword() {
                self.raise_error_prev("Expected destination after '='");
                return None;
            }
            let t = self.previous().clone();
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
            Some(self.previous().clone())
        } else {
            None
        };
        Some(HackInstruction::CInstruction(CInstruction::new(
            dest, comp, jump,
        )))
    }

    fn comp(&mut self) -> Option<Comp> {
        let start = self.curr - 1;
        let max_comp_len = cmp::min(3, self.tokens.len() - start);
        let tokens = &self.tokens[start..start + max_comp_len]
            .iter()
            .map(|token| token.kind.clone())
            .collect::<Vec<token::TokenKind>>();
        let comp = Comp::from_tokens(tokens);
        if let Some(comp) = comp {
            for _ in 0..comp.len() - 1 {
                self.advance();
            }
            Some(comp)
        } else {
            self.raise_comp_error(max_comp_len);
            None
        }
    }

    fn raise_comp_error(&mut self, comp_len: usize) {
        let line = self.peek().line;
        self.errors.push(ErrorFormatter::gen_err(
            "Expected proper computation in c-instruction",
            &self.source,
            self.previous().start,
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

    fn previous(&self) -> &Token {
        &self.tokens[self.curr - 1]
    }

    fn peek(&self) -> &Token {
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
    fn raise_error_prev(&mut self, msg: &str) {
        self.raise_error(msg, self.curr - 1);
    }
    fn raise_error_peek(&mut self, msg: &str) {
        self.raise_error(msg, self.curr);
    }
}
