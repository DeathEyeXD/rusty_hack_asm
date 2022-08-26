use std::{fs, time::Instant};

use crate::{evaluator::HackCodeGenerator, parser::Parser, scanner::Scanner};

mod ast;
mod error_formatting;
mod evaluator;
mod parser;
mod scanner;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn run(path: String) -> Result<()> {
    let t = Instant::now();

    let source = fs::read_to_string(&path)?;
    let source = source.lines().collect::<Vec<&str>>();

    let scanner = Scanner::new(&source);

    #[cfg(feature = "measure")]
    let parse_time = Instant::now();

    let tokens = scanner.run()?;
    let parser = Parser::new(&tokens, &source);
    let instructions = parser.run()?;

    #[cfg(feature = "measure")]
    println!("Parsing took {:?}", parse_time.elapsed());

    let evaluator = HackCodeGenerator::new(instructions);

    let output = evaluator.gen_output_file(&path)?;

    println!("Succesfully compiled '{}' in {:?}", output, t.elapsed());
    Ok(())
}
