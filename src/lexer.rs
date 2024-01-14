use crate::token::Token;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

pub struct Lexer<'a> {
    valid_operators: [char; 8],
    valid_compound_operators: [&'a str; 5],
    valid_grouping_symbols: [char; 2],
    valid_scope_symbols: [char; 2],
    valid_punctuations: [char; 1],
    valid_type_names: [&'a str; 10],
    valid_declaration_keywords: [&'a str; 5],
    valid_class_regions: [&'a str; 2],
}

impl<'a> Lexer<'a> {
    const VALID_OPERATORS: [char; 8] = ['*', '+', '/', '-', '=', ':', '>', '<'];
    const VALID_COMPOUND_OPERATORS: [&'a str; 5] = ["+=", "-=", "/=", "*=", "->"];
    const VALID_GROUPING_SYMBOLS: [char; 2] = ['(', ')'];
    const VALID_SCOPE_SYMBOLS: [char; 2] = ['{', '}'];
    const VALID_PUNCTUATIONS: [char; 1] = [';'];
    const VALID_TYPE_NAMES: [&'a str; 10] = [
        "sint8", "uint8", "sint16", "uint16", "sint32", "uint32", "sint64", "uint64", "usize",
        "ssize",
    ];
    const VALID_DECLARATION_KEYWORDS: [&'a str; 5] =
        ["class", "function", "method", "variable", "constant"];
    const VALID_CLASS_REGIONS: [&'a str; 2] = ["public", "private"];

    pub fn new() -> Self {
        Lexer {
            valid_operators: Self::VALID_OPERATORS,
            valid_compound_operators: Self::VALID_COMPOUND_OPERATORS,
            valid_grouping_symbols: Self::VALID_GROUPING_SYMBOLS,
            valid_scope_symbols: Self::VALID_SCOPE_SYMBOLS,
            valid_punctuations: Self::VALID_PUNCTUATIONS,
            valid_type_names: Self::VALID_TYPE_NAMES,
            valid_declaration_keywords: Self::VALID_DECLARATION_KEYWORDS,
            valid_class_regions: Self::VALID_CLASS_REGIONS,
        }
    }

    /// Check if the current character encountered is part of a compound operator
    /// # Arguments
    /// * `current_char` - The current character being processed by the lexer.
    /// * `peeked_char` - the character immediately after current_char in the file. Not consumed.
    /// * `potential_incomplete_operator` - The most recent built up lexeme from the lexer. If the
    /// current character doesn't make syntactic sense with this, we will return an error
    fn peeking_reveals_compound(
        &self,
        current_char: &char,
        peeked_char: &Option<&char>,
        potential_incomplete_operator: &str,
    ) -> bool {
        // want to return true if look left OR right shows that the current_char is part of an
        // operator
        if !self.is_an_operator(&current_char.to_string()) || current_char.is_whitespace() {
            return false;
        }

        if let Some(next_char) = peeked_char {
            if self.is_an_operator(&next_char.to_string()) {
                return true;
            }

            let mut combined_operator: String = potential_incomplete_operator.to_string();
            combined_operator.push(*current_char);

            println!("checking: {combined_operator}");

            for operator in Self::VALID_COMPOUND_OPERATORS {
                if operator.starts_with(combined_operator.trim()) {
                    return true;
                }
            }
        }

        false
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
                if !self.separates_a_lexeme(&character)
                    || self.peeking_reveals_compound(
                        &character,
                        &characters.peek(),
                        &most_recent_lexeme,
                    )
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
    fn is_a_compound_operator(&self, word: &str) -> bool {
        Self::VALID_COMPOUND_OPERATORS.contains(&word)
    }

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
            "{" => Ok(Token::LeftBracket('{')),
            "}" => Ok(Token::RightBracket('}')),
            comp if self.is_a_compound_operator(comp) => self.new_compound_operator(comp),
            decl if self.is_a_declaration_keyword(decl) => self.new_declaration_keyword(decl),
            region if self.is_a_class_region(region) => self.new_class_region(region),
            op if self.is_an_operator(op) => self.new_operator(op),
            "(" => Ok(Token::LeftParenthesis('(')),
            ")" => Ok(Token::RightParenthesis(')')),
            punc if self.is_a_punctuation(punc) => self.new_punctuation(punc),
            ty if self.is_a_fe_type(ty) => self.new_type_name(ty),
            f if f.parse::<f64>().is_ok() => self.new_float_literal(f),
            y if y.parse::<isize>().is_ok() => self.new_integer_literal(y),
            other => Ok(Token::Identifier(other.to_string())),
        }
    }

    fn new_compound_operator(&self, word: &str) -> Result<Token, Error> {
        match (Self::VALID_COMPOUND_OPERATORS.contains(&word), word) {
            (true, _) => {
                eprintln!("Bad token -> {word} when creating compound operator");
                panic!("Compiler encountered something allowed but not yet implemented")
            }

            (false, _) => {
                let error_message: String = format!("Not a valid compound operator: {word}");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }
    fn new_float_literal(&self, float_literal: &str) -> Result<Token, Error> {
        match float_literal.parse::<f64>() {
            Ok(x) => Ok(Token::FloatLiteral(x)),
            Err(_) => {
                let error_message: String = format!("{float_literal}: Not parseable to isize");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    fn new_integer_literal(&self, integer_literal: &str) -> Result<Token, Error> {
        match integer_literal.parse::<isize>() {
            Ok(x) => Ok(Token::IntegerLiteral(x)),
            Err(_) => {
                let error_message: String = format!("{integer_literal}: Not parseable to isize");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    fn new_punctuation(&self, word: &str) -> Result<Token, Error> {
        if let Ok(punc) = word.parse::<char>() {
            match (Self::VALID_PUNCTUATIONS.contains(&punc), punc) {
                (true, ';') => Ok(Token::Punctuation(';')),
                (true, _) => {
                    eprintln!("Bad token -> {word} when creating punctuation");
                    panic!("Compiler encountered something allowed but not yet implemented")
                }
                (false, _) => {
                    let error_message: String = format!("Not a valid punctuation: {word}");
                    Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
                }
            }
        } else {
            let error_message: String = format!("{word}: Not parseable to char");
            Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
        }
    }

    fn new_type_name(&self, word: &str) -> Result<Token, Error> {
        match (Self::VALID_TYPE_NAMES.contains(&word), word) {
            (true, "uint8") => Ok(Token::UnsignedEightBitInteger("uint8".to_string())),
            (true, "uint16") => Ok(Token::UnsignedSixteenBitInteger("uint16".to_string())),
            (true, "uint32") => Ok(Token::UnsignedThirtyTwoBitInteger("uint32".to_string())),
            (true, "uint64") => Ok(Token::UnsignedSixtyFourBitInteger("uint64".to_string())),
            (true, "sint8") => Ok(Token::SignedEightBitInteger("sint8".to_string())),
            (true, "sint16") => Ok(Token::SignedSixteenBitInteger("sint16".to_string())),
            (true, "sint32") => Ok(Token::SignedThirtyTwoBitInteger("sint32".to_string())),
            (true, "sint64") => Ok(Token::SignedSixtyFourBitInteger("sint64".to_string())),
            (true, "ssize") => Ok(Token::SignedSize("ssize".to_string())),
            (true, "usize") => Ok(Token::UnsignedSize("usize".to_string())),
            (true, "boolean") => Ok(Token::Boolean("boolean".to_string())),
            (true, _) => {
                eprintln!("Bad token -> {word} when creating type name");
                panic!("Compiler encountered something that is allowed but not yet implemented")
            }
            (false, _) => {
                let error_message: String = format!("Not a valid type: {word}");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    fn new_operator(&self, word: &str) -> Result<Token, Error> {
        if let Ok(op) = word.parse::<char>() {
            match (Self::VALID_OPERATORS.contains(&op), op) {
                (true, '+') => Ok(Token::PlusSign('+')),
                (true, '-') => Ok(Token::MinusSign('-')),
                (true, '*') => Ok(Token::MultiplicationSign('*')),
                (true, '/') => Ok(Token::DivisionSign('/')),
                (true, '=') => Ok(Token::EqualSign('=')),
                (true, ':') => Ok(Token::Colon(':')),
                (true, '>') => Ok(Token::GreaterThan('>')),
                (true, '<') => Ok(Token::LessThan('<')),
                (true, _) => {
                    eprintln!("Bad token -> {word} when creating operator");
                    panic!("Compiler encountered something allowed but not yet implemented")
                }
                (false, _) => {
                    let error_message: String = format!("Not a valid keyword!: {word}");
                    Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
                }
            }
        } else {
            let error_message: String = format!("{word}: Not parseable to char");
            Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
        }
    }

    fn new_class_region(&self, word: &str) -> Result<Token, Error> {
        match (Self::VALID_CLASS_REGIONS.contains(&word), word) {
            (true, "public") => Ok(Token::PublicClassRegion("public".to_string())),
            (true, "private") => Ok(Token::PrivateClassRegion("private".to_string())),
            (true, _) => {
                eprintln!("Bad token -> {word} when creating class region");
                panic!("Compiler encountered something that is allowed but not yet implemented")
            }
            (false, _) => {
                let error_message: String = format!("Not a valid keyword!: {word}");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    fn new_declaration_keyword(&self, word: &str) -> Result<Token, Error> {
        match (Self::VALID_DECLARATION_KEYWORDS.contains(&word), word) {
            (true, "class") => Ok(Token::ClassDeclaration("class".to_string())),
            (true, "method") => Ok(Token::MethodDeclaration("method".to_string())),
            (true, "function") => Ok(Token::FunctionDeclaration("function".to_string())),
            (true, "constant") => Ok(Token::ConstantDeclaration("constant".to_string())),
            (true, "variable") => Ok(Token::VariableDeclaration("variable".to_string())),
            (true, _) => {
                eprintln!("Bad token -> {word} when creating declaration keyword");
                panic!("The compiler encountered a type that is allowed but not yet implemented")
            }
            (false, _) => {
                let error_message: String = format!("Not a valid keyword!: {word}");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }
}
