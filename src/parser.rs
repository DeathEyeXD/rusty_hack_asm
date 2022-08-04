use std::cmp;

use crate::ast::{CInstruction, Comp, HackInstruction, Statement};
use crate::evaluator::Evaluator;
use crate::{
    error_formatting::ErrorFormatter,
    scanner::token::{self, Token},
    Error,
    Result
};

pub struct Parser {
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

    pub fn run(mut self) -> Result<Evaluator>{
        if !self.parse(){
            self.print_errors();
            return Err(Error::from(format!("Encountered {}, errors, aborting compilation.", self.errors.len())));
        }
        for statement in &self.statements{
            println!("{:?}", statement);
        }
        Ok(Evaluator::new(self.statements.into_iter().map(|s| match s{
            Statement::Instruction(i) => i,
            Statement::LabelDecl(_) => unimplemented!(),}

        ).collect()))
    }

    fn print_errors(&self) {
        for error in &self.errors {
            eprintln!("{}", error);
        }
    }

    pub fn parse(&mut self) -> bool{ // true means succesfull parsing
        while !self.is_at_end() {
           self.statement(); 
        }
        self.errors.is_empty()
    }

    pub fn had_errors(self) -> bool{
        !self.errors.is_empty()
    }

    fn statement(&mut self) {
        let statement = match self.peek().kind {
            token::TokenKind::LeftParen => self.label_declaration(),
            _ => self.instruction().map(Statement::Instruction),
        };

        if let Some(statement) = statement{
            if self.consume_line_end("Unexpected token after statement, use line feed to separate statements").is_none(){
                return
            }
            self.statements.push(statement);
        }else{
            self.synchronise();
        }
    }

    fn label_declaration(&mut self) -> Option<Statement> {
        let label = self.consume_identifier("Expected label name after '('")?;
        self.consume(
            "Expected ')' after label name",
            token::TokenKind::RightParen,
        )?;
        Some(Statement::LabelDecl(label))
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
            token::TokenKind::Identifier(..) => {unimplemented!()}
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
            }
            _ => {
                self.raise_error_prev("Expected identifier or number after '@'");
                return None;
            } //todo raise an error
        }
        Some(HackInstruction::AInstruction(self.previous().clone()))
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

    fn consume_line_end(&mut self, msg: &str) -> Option<&Token>{
        if self.check_line_end(){
            self.advance();
            Some(self.previous())
        }else{
            self.raise_error_peek(msg);
            None
        }
    }

    fn check_line_end(&mut self) -> bool{
        matches!(self.peek().kind, token::TokenKind::NewLine | token::TokenKind::Eof )
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
        while !self.check_line_end(){
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
            if !self.peek().kind.is_jump_keyword(){
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
        }else{
            self.raise_comp_error(max_comp_len);
            None
        }
    }

    fn raise_comp_error(&mut self, comp_len: usize){
        self.errors.push(ErrorFormatter::gen_err("Expected proper computation in c-instruction", &self.source, self.curr - 1, comp_len, self.peek().line))
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
        self.raise_error(msg, self.curr);
    }
    fn raise_error_peek(&mut self, msg: &str) {
        self.raise_error(msg, self.curr + 1);
    }
}
