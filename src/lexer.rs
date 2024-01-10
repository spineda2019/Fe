use crate::symbol_table::token::Token;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const VALID_OPERATORS: [char; 7] = ['*', '+', '/', '-', '=', ':', ';'];

#[inline]
fn is_an_operator(word: &str) -> bool {
    let charred_word: char = match word.parse::<char>() {
        Ok(x) => x,
        Err(_) => return false,
    };

    VALID_OPERATORS.contains(&charred_word)
}

fn classify_word(word: &str) -> Token {
    match word {
        x if is_an_operator(x) => Token::new_operator(x),
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

        let line: &str = line.trim();
        if line.starts_with("//") {
            continue;
        }

        let mut most_recent_token_string: String = String::new();
        for character in line.chars() {
            if !VALID_OPERATORS.contains(&character) && !character.is_whitespace() {
                most_recent_token_string.push(character);
                continue;
            }
            if most_recent_token_string == "//" {
                // Immeditately going to next line if we see a comment saves
                // time and memory!
                break;
            }

            if !most_recent_token_string.is_empty() {
                tokens.push(classify_word(&most_recent_token_string));
            }
            if !character.is_whitespace() {
                tokens.push(classify_word(&character.to_string()));
            }

            most_recent_token_string.clear();
        }
    }

    Ok(tokens)
}
