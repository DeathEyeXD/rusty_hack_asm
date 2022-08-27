use rusty_hack_asm::run;
use rusty_hack_asm::Error;
use rusty_hack_asm::Result;

fn main() -> Result<()> {
    let path = file_path()?;
    run(path)
}

fn file_path() -> Result<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(Error::from(
            "No file specified. Usage: rusty_rusty_hack_asm <file>",
        ));
    }
    Ok(args.into_iter().nth(1).unwrap())
}
