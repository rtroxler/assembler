use super::Instruction;
use std::fs::File;
use std::io::prelude::*;
pub struct AInstruction {
    line: String,
}

impl AInstruction {
    pub fn new(line: String) -> AInstruction {
        AInstruction { line: line }
    }
    fn symbol(&self) -> String {
        // @Symbol
        self.line[1..].to_string()
    }
}

impl Instruction for AInstruction {
    fn write_binary(&self, output: &mut File) {
        // Pass up Result?
        write!(output, "{:016b}\n", self.symbol().parse::<i32>().unwrap());
    }
}
