#[derive(Debug)]
pub enum Token {
    NumberLiteral(isize),
    Identifier(String), // identifier, table_pointer
    Operator(char),
    TypeName(String),
    Punctuation(char),
}

impl Token {
    pub fn new_number_literal(number_literal: &str) -> Self {
        let number: isize = match number_literal.parse::<isize>() {
            Ok(x) => x,
            Err(_) => panic!("This should not happen! {number_literal}"),
        };
        Token::NumberLiteral(number)
    }

    pub fn new_identifier(identifier: &str) -> Self {
        Token::Identifier(identifier.to_string())
    }

    pub fn new_operator(operator: &str) -> Self {
        let operator_char: char = match operator.parse::<char>() {
            Ok(x) => x,
            Err(_) => panic!("This should not happen! {operator}"),
        };
        Token::Operator(operator_char)
    }

    pub fn new_punctuation(punctuation: &str) -> Self {
        let punctuation_char: char = match punctuation.parse::<char>() {
            Ok(x) => x,
            Err(_) => panic!("This should not happen! {punctuation}"),
        };
        Token::Punctuation(punctuation_char)
    }

    pub fn new_type_name(type_name: &str) -> Self {
        Token::TypeName(type_name.to_string())
    }
}
