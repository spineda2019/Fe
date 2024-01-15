use crate::lexer::Lexer;
use crate::token::Token;
use std::io::{stdin, stdout, Write};

pub fn enter_interactive_mode() -> Result<(), std::io::Error> {
    let lexer: Lexer = Lexer::new();
    println!("\nFe Interactive JIT v0.1");

    let mut instruction: String = String::new();
    loop {
        print!("Fe>> ");
        stdout().flush()?;

        stdin().read_line(&mut instruction)?;
        let line_tokens: Vec<Token> = lexer.tokenize_line(&instruction)?;
        dbg!(&line_tokens);
        stdout().flush()?;

        if instruction.trim() == "exit()" {
            break;
        }

        instruction.clear();
    }

    Ok(())
}
