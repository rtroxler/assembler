use super::Instruction;
use std::fs::File;
use std::io;
pub struct LInstruction {
    line: String,
}

impl LInstruction {
    pub fn new(line: String) -> LInstruction {
        LInstruction { line: line }
    }

    fn symbol(&self) -> String {
        // (Symbol)
        self.line[1..self.line.len() - 1].to_string()
    }
}

impl Instruction for LInstruction {
    fn write_binary(&self, _output: &mut File) -> Result<(), io::Error> {
        Ok(())
    }

    fn print(&self) {
        println!("{} ", self.line);
        println!("\tsymb: \t {:?}", self.symbol());
    }
}
