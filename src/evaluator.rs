
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
        let mut output = String::new();
        for i in &self.instructions{
            output.push_str(&i.to_binary());
            output.push_str("\r\n");
        }
        output
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