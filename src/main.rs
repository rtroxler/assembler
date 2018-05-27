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
        //let line_iter = self.filter_lines(file)
        self.parse_lines(file)
    }

    //fn filter_lines(&self, file: File) -> Iterator {
    //
    //BufReader::new(file)
    //.lines()
    //.map(|line| line.unwrap())
    //.filter(|line| !line.starts_with("//"))
    //.filter(|line| !line.is_empty());
    //}

    fn parse_lines(&self, file: File) {
        let line_iter = BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| !line.starts_with("//"))
            .filter(|line| !line.is_empty());
        for line in line_iter {
            let readable_line = Line { line: line };
            readable_line.print();
        }
    }
}

struct Line {
    line: String,
}

impl Line {
    fn print(&self) {
        println!("{}", self.line);
    }
    //fn commandType(&self) -> String {}

    //fn symbol(&self) -> String {}
    //fn dest(&self) -> String {}
    //fn comp(&self) -> String {}
    //fn jump(&self) -> String {}
}
