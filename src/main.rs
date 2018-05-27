use std::env;
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

struct Parser {
    filename: String,
}

impl Parser {
    fn parse_file(&self) {
        println!("Parsing file: {:?}", self.filename);
        let file = File::open(&self.filename).expect("File not found");
        let line_iter = self.filter_lines(file);
        self.parse_lines(line_iter)
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
    fn parse_lines<I>(&self, line_iter: I)
    where
        I: Iterator<Item = String>,
    {
        for line in line_iter {
            let readable_line = Line { line: line };
            readable_line.print();
        }
    }
}

struct Line {
    line: String,
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
        println!("{} \t {:?}", self.line, self.instruction_type());
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

    //fn symbol(&self) -> String {}
    //fn dest(&self) -> String {}
    //fn comp(&self) -> String {}
    //fn jump(&self) -> String {}
}
