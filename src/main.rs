mod interactive_cli;
mod lexer;
mod parser;
mod token;

use lexer::Lexer;
use std::io::ErrorKind;
use token::Token;

fn compile_fe_file(source_file: &str) -> Result<(), std::io::Error> {
    let lexer: Lexer = Lexer::new();
    println!("Compiling {source_file}!");
    // TODO: lex whitespace delimited words
    // TODO: Classify as tokens
    // TODO: load in to table
    // TODO: Parse Tree?
    let file: std::fs::File = std::fs::File::open(source_file)?;
    let tokens: Vec<Token> = lexer.tokenize_file(&file)?;

    dbg!(&tokens);
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    println!("Hello, Compiler!");
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    if args.len() > 2 {
        // Don't care abt more than 1 arg
        let error_message: &str = "FATAL: Too many args!";
        let error = std::io::Error::new(ErrorKind::Interrupted, error_message);
        return Err(error);
    }

    if let Some(source_file) = &args.get(1) {
        compile_fe_file(source_file)?;
    } else {
        interactive_cli::enter_interactive_mode()?;
    }

    Ok(())
}
