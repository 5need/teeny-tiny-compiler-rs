//https://austinhenley.com/blog/teenytinycompiler1.html

use std::env;
use std::fs::File;
use std::io::{self, Read};
mod emitter;
mod lexer;
mod parser;
mod token;
use emitter::*;
use lexer::*;
use parser::*;
use token::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("{:?}", file_path);
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
    let mut emitter = Emitter::new("out.c".to_string());
    let mut parser = Parser {
        lexer,
        emitter: &mut emitter,
        cur_token: None,
        peek_token: None,
        symbols: Vec::new(),
        labels_declared: Vec::new(),
        labels_gotoed: Vec::new(),
    };
    parser.program();
    emitter.write_file()?;
    println!("Compiling completed.");

    Ok(())
}
