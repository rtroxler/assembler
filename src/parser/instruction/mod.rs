use std::fs::File;
use std::io;

pub mod a_instruction;
pub mod c_instruction;
pub mod l_instruction;

pub trait Instruction {
    fn write_binary(&self, _output: &mut File) -> Result<(), io::Error> {
        Ok(())
    }
    fn print(&self) {}
}
