use std::env;
use std::io::prelude::*;

use std::fs::File;

use std::io::{BufRead, BufReader};

use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let parser = Parser {
        filename: filename.to_string(),
    };
    parser.parse_file();
}

// Bout time to pull this into it's own module
struct Parser {
    filename: String,
}

impl Parser {
    fn parse_file(&self) {
        println!("Parsing file: {:?}", self.filename);
        let file = File::open(&self.filename).expect("File not found");
        // TODO: filename
        let mut output = File::create("Add.hack").unwrap();
        let c_instr = CInstructionTranslator::new();
        let line_iter = self.filter_lines(file);
        self.parse_lines(line_iter, &mut output, &c_instr);
    }

    // Filter lines for comments and whitespace
    fn filter_lines(&self, file: File) -> impl Iterator<Item = String> {
        BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| !line.starts_with("//"))
            .filter(|line| !line.is_empty())
    }

    // Take an iterator of lines and map them to Lines
    fn parse_lines<I>(&self, line_iter: I, output: &mut File, c_instr: &CInstructionTranslator)
    where
        I: Iterator<Item = String>,
    {
        for line in line_iter {
            let readable_line = Line::new(line);
            readable_line.print();

            // Should probably be elsewhere, but shrug
            readable_line.output_binary(output, c_instr);
        }
    }
}

struct Line {
    line: String,
}

impl Line {
    fn new(line: String) -> Line {
        let no_comments = match line.find("//") {
            Some(index) => line[..index].to_string(),
            None => line,
        };
        Line {
            line: no_comments.trim().to_string(),
        }
    }
}

// should a line just have a instruction_type?
#[derive(Debug)]
enum InstructionType {
    A, // Address
    C, // Computation
    L, // Symbol ( generates no machine code )
}

impl Line {
    fn instruction_type(&self) -> InstructionType {
        if self.line.starts_with("@") {
            InstructionType::A
        } else if self.line.starts_with("(") && self.line.ends_with(")") {
            InstructionType::L
        } else {
            InstructionType::C
        }
    }

    fn print(&self) {
        println!("{} ", self.line,);
        println!("\tType: \t {:?}", self.instruction_type());

        match self.instruction_type() {
            InstructionType::C => {
                println!("\tdest: \t {:?}", self.dest());
                println!("\tcomp: \t {:?}", self.comp());
                println!("\tjump: \t {:?}", self.jump());
                println!("");
            }
            _ => {
                println!("\tsymb: \t {:?}", self.symbol().unwrap());
            }
        }
    }

    fn output_binary(&self, output: &mut File, c_instr: &CInstructionTranslator) {
        match self.instruction_type() {
            InstructionType::A => write!(
                output,
                "{:016b}\n",
                self.symbol().unwrap().parse::<i32>().unwrap()
            ),
            InstructionType::L => write!(output, "{:016b}\n", 12345), // Lookup symbol from table (eventually)
            InstructionType::C => {
                write!(output, "{}\n", self.dest_comp_jump_string(c_instr).unwrap())
            }
        };
    }

    fn symbol(&self) -> Option<String> {
        match self.instruction_type() {
            InstructionType::A => Some(self.line[1..].to_string()), // @Symbol
            InstructionType::L => Some(self.line[1..self.line.len() - 1].to_string()), // (Symbol)
            InstructionType::C => None,
        }
    }

    fn dest_comp_jump_string(&self, c_instr: &CInstructionTranslator) -> Option<String> {
        match self.instruction_type() {
            InstructionType::C => {
                let mut result = String::with_capacity(16);
                result.push_str("111");
                let comp = match self.comp() {
                    Some(string) => c_instr.comp_map.get(string.as_str()).cloned().unwrap(),
                    None => "000",
                };
                result.push_str(comp);

                let dest = c_instr
                    .dest_map
                    .get(self.dest().unwrap().as_str())
                    .cloned()
                    .unwrap_or("000");
                result.push_str(dest);

                let jump = match self.jump() {
                    Some(string) => c_instr.jump_map.get(string.as_str()).cloned().unwrap(),
                    None => "000",
                };
                result.push_str(jump);

                Some(result)
            }
            _ => None,
        }
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

struct CInstructionTranslator {
    dest_map: HashMap<&'static str, &'static str>,
    jump_map: HashMap<&'static str, &'static str>,
    comp_map: HashMap<&'static str, &'static str>,
}

impl CInstructionTranslator {
    fn new() -> CInstructionTranslator {
        let mut dest_map = HashMap::new();
        dest_map.insert("M", "001");
        dest_map.insert("D", "010");
        dest_map.insert("A", "100");
        dest_map.insert("MD", "011");
        dest_map.insert("AM", "101");
        dest_map.insert("AD", "110");
        dest_map.insert("AMD", "111");

        let mut jump_map = HashMap::new();
        jump_map.insert("JGT", "001");
        jump_map.insert("JEQ", "010");
        jump_map.insert("JGE", "100");
        jump_map.insert("JLT", "011");
        jump_map.insert("JNE", "101");
        jump_map.insert("JLE", "110");
        jump_map.insert("JMP", "111");

        let mut comp_map = HashMap::new();
        comp_map.insert("0", "0101010");
        comp_map.insert("1", "0111111");
        comp_map.insert("-1", "0111010");
        comp_map.insert("D", "0001100");
        comp_map.insert("A", "0110000");
        comp_map.insert("!D", "0001101");
        comp_map.insert("!A", "0110001");
        comp_map.insert("-D", "0001111");
        comp_map.insert("-A", "0110011");
        comp_map.insert("D+1", "0011111");
        comp_map.insert("A+1", "0110111");
        comp_map.insert("A+1", "0110111");
        comp_map.insert("D-1", "0001110");
        comp_map.insert("A-1", "0110010");
        comp_map.insert("D+A", "0000010");
        comp_map.insert("D-A", "0010011");
        comp_map.insert("A-D", "0000111");
        comp_map.insert("D&A", "0000000");
        comp_map.insert("D|A", "0010101");
        comp_map.insert("M", "1110000");
        comp_map.insert("!M", "1110001");
        comp_map.insert("-M", "1110011");
        comp_map.insert("M+1", "1110111");
        comp_map.insert("M-1", "1110010");
        comp_map.insert("D+M", "1000010");
        comp_map.insert("D-M", "1010011");
        comp_map.insert("M-D", "1000111");
        comp_map.insert("D&M", "1000000");
        comp_map.insert("D|M", "1010101");

        CInstructionTranslator {
            dest_map: dest_map,
            jump_map: jump_map,
            comp_map: comp_map,
        }
    }
}
