use crate::{scanner::token::Token, Error};

pub struct ErrorFormatter;

impl ErrorFormatter {
    pub fn gen_err(
        message: &str,
        source: &[&str],
        start: usize,
        len: usize,
        line: usize,
    ) -> Error {
        let line_content = &source[line];
        let highlight = Self::highlight(line_content, start, len, line + 1);
        Error::from(format!(
            "{}\n error: {}",
            highlight, message
        ))
    }

    pub fn err_from_token(msg: &str, source: &[&str], token: &Token) -> Error {
        Self::gen_err(msg, source, token.start, token.len(), token.line)
    }

    fn highlight(line_content: &str, start: usize, len: usize, line: usize) -> String {
        let line_str = line.to_string();
        let padding_len = &line_str.len() + 3 + start;
        format!("{} | {}\n", line_str, line_content)
            + &format!("{}{}--here", " ".repeat(padding_len), "^".repeat(len))
    }
}
