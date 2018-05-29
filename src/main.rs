use std::env;

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let parser = parser::Parser::new(filename.to_string());
    parser.parse_file();
}
