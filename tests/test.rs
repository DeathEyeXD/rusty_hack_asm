use assert_cmd::assert;
use hack_asm::Result;
use std::fs::{self};

fn run_binary(file: &str) -> assert::Assert{
    assert_cmd::Command::cargo_bin("hack_asm").unwrap().arg(file).assert()
}

fn compile_and_compare(filename: &str) -> Result<()> {
    let source_file = format!("tests/data/asm/{}.asm", filename);

    run_binary(&source_file).success();
    compare(filename)

}

fn compare(base_filename: &str) -> Result<()> {
    let bin_file = format!("tests/data/asm/{}.hack", base_filename);
    let expected_file = format!("tests/data/expected/{}.hack", base_filename);


    assert!(fs::read_to_string(&bin_file)? == fs::read_to_string(&expected_file)? , "generated file {}.asm does not match expected data", base_filename);
    // fs::remove_file(&expected_file)?;
    Ok(())
}

#[test]
fn test_compilation_add() -> Result<()> {
    compile_and_compare("Add")
}

#[test]
fn test_compilation_max_symboless() -> Result<()> {
    compile_and_compare("MaxL")
}

#[test]
fn test_compilation_pong_symbolless() -> Result<()> {
    compile_and_compare("PongL")
}
