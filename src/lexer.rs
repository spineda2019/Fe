use crate::symbol_table::token::Token;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

const VALID_OPERATORS: [char; 6] = ['*', '+', '/', '-', '=', ':'];
const VALID_GROUPING_SYMBOLS: [char; 2] = ['(', ')'];
const VALID_SCOPE_SYMBOLS: [char; 2] = ['{', '}'];
const VALID_PUNCTUATIONS: [char; 1] = [';'];
const VALID_TYPE_NAMES: [&str; 9] = [
    "sint8", "uint8", "sint16", "uint16", "sint32", "uint32", "sint64", "uint64", "size",
];

#[inline]
fn is_a_fe_type(word: &str) -> bool {
    VALID_TYPE_NAMES.contains(&word)
}

#[inline]
fn separates_a_lexeme(character: &char) -> bool {
    VALID_OPERATORS.contains(character)
        || character.is_whitespace()
        || VALID_PUNCTUATIONS.contains(character)
        || VALID_SCOPE_SYMBOLS.contains(character)
}

#[inline]
fn is_a_grouping_symbol(word: &str) -> bool {
    let charred_word: char = match word.parse::<char>() {
        Ok(x) => x,
        Err(_) => return false,
    };

    VALID_GROUPING_SYMBOLS.contains(&charred_word)
}

#[inline]
fn is_an_operator(word: &str) -> bool {
    let charred_word: char = match word.parse::<char>() {
        Ok(x) => x,
        Err(_) => return false,
    };

    VALID_OPERATORS.contains(&charred_word)
}

#[inline]
fn is_a_punctuation(word: &str) -> bool {
    let charred_word: char = match word.parse::<char>() {
        Ok(x) => x,
        Err(_) => return false,
    };

    VALID_PUNCTUATIONS.contains(&charred_word)
}

fn classify_word(word: &str) -> Token {
    match word {
        op if is_an_operator(op) => Token::new_operator(op),
        gr if is_a_grouping_symbol(gr) => Token::new_grouping_symbol(gr),
        punc if is_a_punctuation(punc) => Token::new_punctuation(punc),
        ty if is_a_fe_type(ty) => Token::new_type_name(ty),
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

        let mut most_recent_lexeme: String = String::new();
        for character in line.chars() {
            dbg!(&character);
            if !separates_a_lexeme(&character) {
                most_recent_lexeme.push(character);
                continue;
            }

            if most_recent_lexeme == "//" {
                // Immeditately going to next line if we see a comment saves
                // time and memory!
                break;
            }

            dbg!(&most_recent_lexeme);

            if !most_recent_lexeme.is_empty() {
                tokens.push(classify_word(&most_recent_lexeme));
            }
            if !character.is_whitespace() {
                tokens.push(classify_word(&character.to_string()));
            }

            most_recent_lexeme.clear();
        }
        if !most_recent_lexeme.is_empty() {
            let error_message: &str = "Line did not end in punctuation!";
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("{error_message}: {line}"),
            ));
        }
    }

    Ok(tokens)
}
