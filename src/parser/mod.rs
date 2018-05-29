use std::fs::File;
use std::io::{BufRead, BufReader};

mod c_instruction_translator;
mod instruction;
use parser::instruction::Instruction;
use parser::instruction::a_instruction::AInstruction;
use parser::instruction::c_instruction::CInstruction;
use parser::instruction::l_instruction::LInstruction;

pub struct Parser {
    filename: String,
}

impl Parser {
    pub fn new(filename: String) -> Parser {
        Parser { filename: filename }
    }
    pub fn parse_file(&self) {
        println!("Parsing file: {:?}", self.filename);
        let file = File::open(&self.filename).expect("File not found");
        // TODO: filename
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
            // base line parses to determine type
            // transform into a Instruction trait object
            let instruction = readable_line.transform();

            // Should probably be elsewhere, but shrug
            instruction.print();
            instruction
                .write_binary(output)
                .expect("Failed to write instruction to file.");
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
    fn new(line: String) -> Line {
        let no_comments = match line.find("//") {
            Some(index) => line[..index].to_string(),
            None => line,
        };
        Line {
            line: no_comments.trim().to_string(),
        }
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

    // Takes the line, eats it, returns an instruction implementing Trait
    // The heap allocation is meh, but it makes the rest of the code so much cleaner
    fn transform(self) -> Box<Instruction> {
        match self.instruction_type() {
            InstructionType::A => Box::new(AInstruction::new(self.line)),
            InstructionType::C => Box::new(CInstruction::new(
                self.line,
                c_instruction_translator::CInstructionTranslator::new(),
            )),
            InstructionType::L => Box::new(LInstruction::new(self.line)),
        }
    }
}
