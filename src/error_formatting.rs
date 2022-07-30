use crate::{Error, scanner::token::Token};

pub struct ErrorFormatter;

impl ErrorFormatter {
    pub fn gen_err(message: &str, source: &str, start: usize, len: usize, line: usize) -> Error {
        let highlight = Self::highlight(source, start, len, line);
        Error::from(format!("{}\n error: {}", highlight, message))
    }

    pub fn err_from_token(msg: &str, source: &str, token: &Token) -> Error {
        Self::gen_err(msg, source, token.start, token.to_string().len(), token.line)
    }

    fn get_line_contents(source: &str, start: usize) -> (&str, usize) {
        let from = start
            - source[..start]
                .bytes()
                .rev()
                .position(|c| c == b'\n')
                .unwrap_or(start);
        let to = source[from..].bytes().position(|c| c == b'\n').unwrap_or(0) + from;

        (&source[from..to], from)
    }

    fn highlight(source: &str, start: usize, len: usize, line: usize) -> String {
        let (line_contents, line_start) = Self::get_line_contents(source, start);
        let padding_len = line / 10 + 4 + start - line_start;
        format!("{} | {}\n", line, line_contents)
            + &format!("{}{}--here", " ".repeat(padding_len), "^".repeat(len))
    }
}