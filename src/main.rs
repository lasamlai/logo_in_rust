use crate::interpretator::inter;
use crate::tokenizer::lexer_rules;
use std::env;
use std::fs;

mod interpretator;
mod parser;
mod robot;
mod tokenizer;
mod unsee;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    println!("File: {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lexemes =
        santiago::lexer::lex(&lexer_rules(), &contents).unwrap_or_else(|e| panic!("{}", e));
    let data: Vec<&str> = lexemes.iter().map(|r| r.raw.as_ref()).collect();
    let image = inter(data);

    svg::save("image.svg", &image).unwrap();
}
