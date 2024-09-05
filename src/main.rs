mod lex;
mod parse;
use crate::lex::Lexer;
use crate::parse::Parser;
use std::env::args;

fn main() {
    println!("Tiny Compiler");
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: {} <source file>", args[0]);
        std::process::exit(1);
    }
    
    let source = std::fs::read_to_string(&args[1]);
    match source {
        Ok(contents) => {
            let lexer = Lexer::new(contents);
            let mut parser = Parser::new(lexer);
            
            // Start parsing
            match parser.program() {
                Ok(_) => println!("Parsing completed successfully."),
                Err(e) => {
                    eprintln!("Parsing error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(error) => {
            println!("Source file error: {}", error);
            std::process::exit(1);
        }
    }
}
