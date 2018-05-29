use super::super::c_instruction_translator;
use super::Instruction;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

pub struct CInstruction {
    line: String,
    c_instr: c_instruction_translator::CInstructionTranslator,
}

impl CInstruction {
    pub fn new(
        line: String,
        c_instr: c_instruction_translator::CInstructionTranslator,
    ) -> CInstruction {
        CInstruction {
            line: line,
            c_instr: c_instr,
        }
    }

    fn dest_comp_jump_string(&self) -> String {
        let mut result = String::with_capacity(16);
        result.push_str("111");
        let comp = match self.comp() {
            Some(string) => self.c_instr.comp_map.get(string.as_str()).cloned().unwrap(),
            None => "000",
        };
        result.push_str(comp);

        let dest = self.c_instr
            .dest_map
            .get(self.dest().unwrap().as_str())
            .cloned()
            .unwrap_or("000");
        result.push_str(dest);

        let jump = match self.jump() {
            Some(string) => self.c_instr.jump_map.get(string.as_str()).cloned().unwrap(),
            None => "000",
        };
        result.push_str(jump);

        result
    }

    fn dest(&self) -> Option<String> {
        // If there's an =, pull all up to the equal. If there's not, return None
        match self.line.find('=') {
            Some(index) => Some(self.line[..index].to_string()),
            None => None,
        }
    }
    fn comp(&self) -> Option<String> {
        // Either after the = or before the ;
        match self.line.find('=') {
            Some(index) => Some(self.line[index + 1..].to_string()),
            None => match self.line.find(';') {
                Some(index) => Some(self.line[..index].to_string()),
                None => None,
            },
        }
    }
    fn jump(&self) -> Option<String> {
        // After the ;, if there's a ;
        match self.line.find(';') {
            Some(index) => Some(self.line[index + 1..].to_string()),
            None => None,
        }
    }
}

impl Instruction for CInstruction {
    fn write_binary(&self, output: &mut File) -> Result<(), Error> {
        write!(output, "{}\n", self.dest_comp_jump_string())
    }

    fn print(&self) {
        println!("{} ", self.line);
        println!("\tdest: \t {:?}", self.dest());
        println!("\tcomp: \t {:?}", self.comp());
        println!("\tjump: \t {:?}", self.jump());
    }
}
