//https://austinhenley.com/blog/teenytinycompiler1.html

mod lexer;
use lexer::*;
use std::fs::File;
use std::io::{self, Read};
mod token;
use token::*;
mod parser;
use parser::*;

fn main() -> io::Result<()> {
    let file_path = "test.txt";
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!(
        "input test.txt\n------------\n{}------------",
        content.to_string()
    );

    let lexer = Lexer {
        source: content.to_string(),
        cur_char: ' ',
        cur_pos: 0,
    };

    let mut parser = Parser {
        lexer,
        cur_token: None,
        peek_token: None,
        symbols: Vec::new(),
        labels_declared: Vec::new(),
        labels_gotoed: Vec::new(),
    };
    parser.program();

    Ok(())
}
