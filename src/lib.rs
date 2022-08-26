use std::{time::Instant, fs};

use crate::{scanner::Scanner, parser::Parser, evaluator::HackCodeGenerator};

mod scanner;
mod parser;
mod error_formatting;
mod evaluator;
mod ast;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn run(path: String) -> Result<()> {
    let t = Instant::now();

    let source = fs::read_to_string(&path)?;
    let source = source.lines().collect::<Vec<&str>>();

    let scanner = Scanner::new(&source);

    let tokens = scanner.run()?;
    let parser = Parser::new(&tokens, &source);
    let instructions = parser.run()?;
    let evaluator = HackCodeGenerator::new(instructions);
    

    let output = evaluator.gen_output_file(&path)?;

    println!("Succesfully compiled '{}' in {:?}", output, t.elapsed());
    Ok(())
}