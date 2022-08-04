use hack_asm::run;
use hack_asm::Result;
use hack_asm::Error;

fn main() -> Result<()> {
    let path = file_path()?;
    run(path)
}

fn file_path() -> Result<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(Error::from("No file specified. Usage: hack_asm <file>"));
    }
    Ok(args.into_iter().nth(1).unwrap())
}
