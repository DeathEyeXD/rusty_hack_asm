
use std::fs;

use crate::Result;
use crate::ast::HackInstruction;
pub struct Evaluator{
    instructions: Vec<HackInstruction>,
}

impl Evaluator {
    const EXTENSION: &'static str = "hack";
    pub fn new(instructions: Vec<HackInstruction>) -> Self {
        Evaluator {
            instructions,
        }
    }

    pub fn evaluate(&self) -> String{
        let output: Vec<String> = self.instructions.iter().map(|instruction| instruction.to_binary()).collect();
        output.join("\r\n")
    }

    fn get_output_filename(&self, mut filename: &str) -> String{
        for (id, char) in filename.chars().rev().enumerate() {
            match char {
                '.' => {
                    let id = filename.len() - id - 1;
                    filename = &filename[..id];
                    break;
                }
                '/' | '\\' => break,
                _ => continue,
            }
        }

        format!("{}.{}", filename, Self::EXTENSION)
    }

    pub fn gen_output_file(self, source: &str) -> Result<()>{
        fs::write(self.get_output_filename(source), self.evaluate())?;
        Ok(())
    }
}