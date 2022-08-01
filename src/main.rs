mod scanner;
mod parser;
mod error_formatting;
mod evaluator;
mod ast;


pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
fn main() -> Result<()> {
    run()
}

fn file_path() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Usage: assembler <path>");
    }
    args.into_iter().nth(1).unwrap()
}

fn run() -> Result<()> {
    let path = file_path();
    let scanner = scanner::Scanner::from_path(&path)?;

    let parser = scanner.run()?;
    let evaluator = parser.run()?;

    evaluator.gen_output_file(&path)
    
}
