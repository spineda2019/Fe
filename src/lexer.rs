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
const VALID_DECLARATION_KEYWORDS: [&str; 3] = ["class", "function", "method"];
const VALID_CLASS_REGIONS: [&str; 2] = ["public", "private"];

fn classify_word(word: &str) -> Token {
    match word {
        decl if is_a_declaration_keyword(decl) => Token::new_declartion_keyword(decl),
        region if is_a_class_region(region) => Token::new_class_region(region),
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
    let mut tokens: Vec<Token> = Vec::new();
    let file_lines = reader.lines().peekable();

    for line in file_lines {
        let line = match line {
            Ok(x) => x,
            Err(y) => return Err(y),
        };

        let line: &str = line.trim();
        if line.starts_with('#') {
            continue;
        }

        let mut most_recent_lexeme: String = String::new();
        for character in line.chars() {
            dbg!(&character);
            dbg!(&most_recent_lexeme);
            if !separates_a_lexeme(&character) {
                most_recent_lexeme.push(character);
                continue;
            }

            if most_recent_lexeme.starts_with('#') {
                // Immeditately going to next line if we see a comment saves
                // time and memory!
                most_recent_lexeme.clear();
                break;
            }

            if !most_recent_lexeme.is_empty() {
                tokens.push(classify_word(&most_recent_lexeme));
            }
            if !character.is_whitespace() {
                tokens.push(classify_word(&character.to_string()));
            }

            most_recent_lexeme.clear();
        }

        if !most_recent_lexeme.is_empty() {
            dbg!(&most_recent_lexeme);
            dbg!(&line);
            let error_message: &str = "Line did not end in punctuation!";
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("{error_message}: {line}"),
            ));
        }
    }

    Ok(tokens)
}

// ////////////////////////////////////////////////////////////////////////////////////////////  //
//                                  Token Classification Helpers                                 //
// ////////////////////////////////////////////////////////////////////////////////////////////  //

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

#[inline]
fn is_a_class_region(word: &str) -> bool {
    VALID_CLASS_REGIONS.contains(&word)
}

#[inline]
fn is_a_declaration_keyword(word: &str) -> bool {
    VALID_DECLARATION_KEYWORDS.contains(&word)
}
