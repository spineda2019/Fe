mod lexer;
mod token;
use std::io::ErrorKind;
use token::Token;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, Compiler!");
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        // Don't care abt more than 1 arg
        let error_message: &str = "FATAL: Need a .fe file input";
        let error = std::io::Error::new(ErrorKind::Interrupted, error_message);
        return Err(error);
    }

    let source_file: &str = &args[1];
    println!("Compiling {source_file}!");
    // TODO: lex whitespace delimited words
    // TODO: Classify as tokens
    // TODO: load in to table
    // TODO: Parse Tree?
    let file: std::fs::File = std::fs::File::open(source_file)?;
    let tokens: Vec<Token> = lexer::tokenize_file(&file)?;

    dbg!(&tokens);
    Ok(())
}
