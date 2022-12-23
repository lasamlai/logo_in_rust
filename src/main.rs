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
    let data = match santiago::lexer::lex(&lexer_rules(), &contents) {
        Ok(lexemes) => lexemes
            .into_iter()
            .map(|r| r.raw.clone())
            .collect::<Vec<String>>()
            .join(" "),
        Err(e) => {
            panic!("{}", e);
        }
    };
    let image = inter(data);

    svg::save("image.svg", &image).unwrap();
}
