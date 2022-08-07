use crate::{Error, scanner::token::Token};

pub struct ErrorFormatter;

impl ErrorFormatter {
    pub fn gen_err(message: &str, source: &[String], start: usize, len: usize, line: usize) -> Error {
        let line_content = &source[line];
        let highlight = Self::highlight(line_content, start, len, line + 1);
        Error::from(format!("{}\n error: {}", highlight, message))
    }

    pub fn err_from_token(msg: &str, source: &[String], token: &Token) -> Error {
        Self::gen_err(msg, source, token.start, token.to_string().len(), token.line)
    }

    fn highlight(line_content: &str, start: usize, len: usize, line: usize) -> String {
        let padding_len = line / 10 + 4 + start;
        format!("{} | {}\n", line, line_content)
            + &format!("{}{}--here", " ".repeat(padding_len), "^".repeat(len))
    }
}