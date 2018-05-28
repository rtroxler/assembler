use std::env;
use std::io::prelude::*;

use std::fs::File;

use std::io::{BufRead, BufReader};

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
        let mut output = File::create("Add.hack").unwrap();
        let line_iter = self.filter_lines(file);
        self.parse_lines(line_iter, &mut output);
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
    fn parse_lines<I>(&self, line_iter: I, output: &mut File)
    where
        I: Iterator<Item = String>,
    {
        for line in line_iter {
            let readable_line = Line::new(line);
            readable_line.print();

            // Should probably be elsewhere, but shrug
            readable_line.output_binary(output);
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
                println!("\tconst: \t {:?}", self.symbol_as_constant().unwrap());
            }
        }
    }

    fn output_binary(&self, output: &mut File) {
        match self.instruction_type() {
            InstructionType::A => write!(
                output,
                "{:016b}\n",
                self.symbol().unwrap().parse::<i32>().unwrap()
            ),
            InstructionType::L => write!(output, "{:016b}\n", 12345),
            InstructionType::C => write!(output, "{:016b}\n", 12345),
        };
    }

    fn instruction_type(&self) -> InstructionType {
        if self.line.starts_with("@") {
            InstructionType::A
        } else if self.line.starts_with("(") && self.line.ends_with(")") {
            InstructionType::L
        } else {
            InstructionType::C
        }
    }

    fn symbol(&self) -> Option<String> {
        match self.instruction_type() {
            InstructionType::A => Some(self.line[1..].to_string()), // @Symbol
            InstructionType::L => Some(self.line[1..self.line.len() - 1].to_string()), // (Symbol)
            InstructionType::C => None,
        }
    }

    fn symbol_as_constant(&self) -> Option<u32> {
        match self.instruction_type() {
            InstructionType::A => Some(self.symbol().unwrap().parse::<u32>().unwrap()), // @Symbol
            InstructionType::L => Some(self.symbol().unwrap().parse::<u32>().unwrap()), // (Symbol)
            InstructionType::C => None,
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
