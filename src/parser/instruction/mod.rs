use std::fs::File;
pub mod a_instruction;
pub mod c_instruction;
pub mod l_instruction;

pub trait Instruction {
    fn write_binary(&self, _output: &mut File) {}
    fn print(&self) {
        //println!("{} ", self.line());
    }
}
