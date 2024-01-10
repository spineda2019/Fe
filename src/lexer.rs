use crate::symbol_table::token::Token;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const VALID_OPERATORS: [&str; 4] = ["*", "+", "/", "-"];

fn classify_word(word: &str) -> Token {
    match word {
        x if VALID_OPERATORS.contains(&x) => Token::new_operator(x),
        y if y.parse::<isize>().is_ok() => Token::new_number_literal(y),
        z => Token::new_identifier(z),
    }
}

pub fn tokenize_file(file: &File) -> Result<Vec<Token>, std::io::Error> {
    let reader: BufReader<&File> = BufReader::new(file);
    let mut line: String;
    let mut tokens: Vec<Token> = Vec::new();

    for file_line in reader.lines() {
        line = match file_line {
            Ok(x) => x,
            Err(y) => return Err(y),
        };

        for word in line.split_whitespace() {
            tokens.push(classify_word(word));
        }
    }

    Ok(tokens)
}
