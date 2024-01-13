use crate::token::Token;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

pub struct Lexer<'a> {
    valid_operators: [char; 6],
    valid_compound_operators: [&'a str; 5],
    valid_grouping_symbols: [char; 2],
    valid_scope_symbols: [char; 2],
    valid_punctuations: [char; 1],
    valid_type_names: [&'a str; 10],
    valid_declaration_keywords: [&'a str; 5],
    valid_class_regions: [&'a str; 2],
}

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        Lexer {
            valid_operators: ['*', '+', '/', '-', '=', ':'],
            valid_compound_operators: ["+=", "-=", "/=", "*=", "->"],
            valid_grouping_symbols: ['(', ')'],
            valid_scope_symbols: ['{', '}'],
            valid_punctuations: [';'],
            valid_type_names: [
                "sint8", "uint8", "sint16", "uint16", "sint32", "uint32", "sint64", "uint64",
                "usize", "ssize",
            ],
            valid_declaration_keywords: ["class", "function", "method", "variable", "constant"],
            valid_class_regions: ["public", "private"],
        }
    }

    /// This lexes the input file, and is "the lexer"
    pub fn tokenize_file(&self, file: &File) -> Result<Vec<Token>, std::io::Error> {
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

            let mut characters = line.chars().peekable();

            let mut most_recent_lexeme: String = String::new();
            while let Some(character) = characters.next() {
                dbg!(&character);
                dbg!(&most_recent_lexeme);
                // TODO: Implement some peeking
                if !self.separates_a_lexeme(&character)
                    || peeking_reveals_compound(
                        &character,
                        &characters.peek(),
                        &most_recent_lexeme,
                    )?
                {
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
                    tokens.push(self.classify_word(&most_recent_lexeme)?);
                }
                if !character.is_whitespace() {
                    tokens.push(self.classify_word(&character.to_string())?);
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

    // ////////////////////////////////////////////////////////////////////////////////////////  //
    //                                Token Classification Helpers                               //
    // ////////////////////////////////////////////////////////////////////////////////////////  //
    #[inline]
    fn is_an_operator(&self, word: &str) -> bool {
        let charred_word: char = match word.parse::<char>() {
            Ok(x) => x,
            Err(_) => return false,
        };

        self.valid_operators.contains(&charred_word)
    }

    #[inline]
    fn is_a_fe_type(&self, word: &str) -> bool {
        self.valid_type_names.contains(&word)
    }

    #[inline]
    fn separates_a_lexeme(&self, character: &char) -> bool {
        self.valid_operators.contains(character)
            || character.is_whitespace()
            || self.valid_punctuations.contains(character)
            || self.valid_scope_symbols.contains(character)
            || self.valid_grouping_symbols.contains(character)
    }

    #[inline]
    fn is_a_punctuation(&self, word: &str) -> bool {
        let charred_word: char = match word.parse::<char>() {
            Ok(x) => x,
            Err(_) => return false,
        };

        self.valid_punctuations.contains(&charred_word)
    }

    #[inline]
    fn is_a_class_region(&self, word: &str) -> bool {
        self.valid_class_regions.contains(&word)
    }

    #[inline]
    fn is_a_declaration_keyword(&self, word: &str) -> bool {
        self.valid_declaration_keywords.contains(&word)
    }
    /// Classify the most recent processed lexeme from the lexer as a Token
    fn classify_word(&self, word: &str) -> Result<Token, Error> {
        match word {
            "{" => Token::new_left_bracket("{"),
            "}" => Token::new_right_bracket("}"),
            decl if self.is_a_declaration_keyword(decl) => Token::new_declartion_keyword(decl),
            region if self.is_a_class_region(region) => Token::new_class_region(region),
            op if self.is_an_operator(op) => Token::new_operator(op),
            "(" => Token::new_left_parenthesis("("),
            ")" => Token::new_right_parenthesis(")"),
            punc if self.is_a_punctuation(punc) => Token::new_punctuation(punc),
            ty if self.is_a_fe_type(ty) => Token::new_type_name(ty),
            y if y.parse::<isize>().is_ok() => Token::new_number_literal(y),
            z => Token::new_identifier(z),
        }
    }
}

/// Check if the current character encountered is part of a compound operator
/// # Arguments
/// * `current_char` - The current character being processed by the lexer.
/// * `peeked_char` - the character immediately after current_char in the file. Not consumed.
/// * `potential_incomplete_operator` - The most recent built up lexeme from the lexer. If the
/// current character doesn't make syntactic sense with this, we will return an error
fn peeking_reveals_compound(
    current_char: &char,
    peeked_char: &Option<&char>,
    potential_incomplete_operator: &str,
) -> Result<bool, Error> {
    return Ok(false);
    todo!();
}
