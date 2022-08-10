use std::time::Instant;

mod scanner;
mod parser;
mod error_formatting;
mod evaluator;
mod ast;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn run(path: String) -> Result<()> {
    let t = Instant::now();
    let scanner = scanner::Scanner::from_path(&path)?;

    let parser = scanner.run()?;
    let evaluator = parser.run()?;

    let output = evaluator.gen_output_file(&path)?;

    println!("Succesfully compiled '{}' in {:?}", output, t.elapsed());
    Ok(())
}