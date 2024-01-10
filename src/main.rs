mod lexer;
mod symbol_table;
use std::io::ErrorKind;
use symbol_table::{token::Token, SymbolTable};

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
    // TODO: Open File
    // TODO: Parse whitespace delimited words
    // TODO: Classify as tokens
    // TODO: load in to table
    let file: std::fs::File = std::fs::File::open(source_file)?;
    let tokens: Vec<Token> = lexer::tokenize_file(&file)?;

    let symbol_table: SymbolTable = SymbolTable::init();

    Ok(())
}
