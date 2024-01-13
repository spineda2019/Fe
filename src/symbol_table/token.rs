use std::io::Error;

#[derive(Debug)]
pub enum Token {
    NumberLiteral(isize),
    Identifier(String),
    /* Operators */
    PlusSign(char),
    MinusSign(char),
    MultiplicationSign(char),
    DivisionSign(char),
    EqualSign(char),
    Colon(char),
    /* Grouping Symbols */
    LeftParenthesis(char),
    RightParenthesis(char),
    TypeName(String),
    Punctuation(char),
    DeclarationKeyword(String),
    ClassRegion(String),
    /* Scope Edge */
    LeftBracket(char),
    RightBracket(char),
}

impl Token {
    pub fn new_number_literal(number_literal: &str) -> Result<Self, std::io::Error> {
        match number_literal.parse::<isize>() {
            Ok(x) => Ok(Token::NumberLiteral(x)),
            Err(_) => {
                let error_message: String = format!("{number_literal}: Not parseable to isize");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    pub fn new_identifier(identifier: &str) -> Result<Self, std::io::Error> {
        Ok(Token::Identifier(identifier.to_string()))
    }

    pub fn new_equal_sign(equal_sign: &str) -> Result<Self, std::io::Error> {
        match equal_sign.parse::<char>() {
            Ok(x) => Ok(Token::EqualSign(x)),
            Err(_) => {
                let error_message: String = format!("{equal_sign}: Not parseable to char");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    pub fn new_plus_sign(plus_sign: &str) -> Result<Self, std::io::Error> {
        match plus_sign.parse::<char>() {
            Ok(x) => Ok(Token::PlusSign(x)),
            Err(_) => {
                let error_message: String = format!("{plus_sign}: Not parseable to char");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    pub fn new_minus_sign(minus_sign: &str) -> Result<Self, std::io::Error> {
        match minus_sign.parse::<char>() {
            Ok(x) => Ok(Token::MinusSign(x)),
            Err(_) => {
                let error_message: String = format!("{minus_sign}: Not parseable to char");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    pub fn new_division_sign(division_sign: &str) -> Result<Self, std::io::Error> {
        match division_sign.parse::<char>() {
            Ok(x) => Ok(Token::DivisionSign(x)),
            Err(_) => {
                let error_message: String = format!("{division_sign}: Not parseable to char");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    pub fn new_multiplication_sign(multiplication_sign: &str) -> Result<Self, std::io::Error> {
        match multiplication_sign.parse::<char>() {
            Ok(x) => Ok(Token::MultiplicationSign(x)),
            Err(_) => {
                let error_message: String = format!("{multiplication_sign}: Not parseable to char");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    pub fn new_colon(colon: &str) -> Result<Self, std::io::Error> {
        match colon.parse::<char>() {
            Ok(x) => Ok(Token::Colon(x)),
            Err(_) => {
                let error_message: String = format!("{colon}: Not parseable to char");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    pub fn new_punctuation(punctuation: &str) -> Result<Self, std::io::Error> {
        match punctuation.parse::<char>() {
            Ok(x) => Ok(Token::Punctuation(x)),
            Err(_) => {
                let error_message: String = format!("{punctuation}: Not parseable to char");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    pub fn new_type_name(type_name: &str) -> Result<Self, std::io::Error> {
        Ok(Token::TypeName(type_name.to_string()))
    }

    pub fn new_left_parenthesis(left_parenthesis: &str) -> Result<Self, std::io::Error> {
        match left_parenthesis.parse::<char>() {
            Ok(x) => Ok(Token::LeftParenthesis(x)),
            Err(_) => {
                let error_message: String = format!("{left_parenthesis}: Not parseable to char");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    pub fn new_right_parenthesis(right_parenthesis: &str) -> Result<Self, std::io::Error> {
        match right_parenthesis.parse::<char>() {
            Ok(x) => Ok(Token::RightParenthesis(x)),
            Err(_) => {
                let error_message: String = format!("{right_parenthesis}: Not parseable to char");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    pub fn new_class_region(class_region: &str) -> Result<Self, std::io::Error> {
        Ok(Token::ClassRegion(class_region.to_string()))
    }

    pub fn new_declartion_keyword(declaration_keyword: &str) -> Result<Self, std::io::Error> {
        Ok(Token::DeclarationKeyword(declaration_keyword.to_string()))
    }

    pub fn new_left_bracket(left_bracket: &str) -> Result<Self, std::io::Error> {
        match left_bracket.parse::<char>() {
            Ok(x) => Ok(Token::LeftBracket(x)),
            Err(_) => {
                let error_message: String = format!("{left_bracket}: Not parseable to char");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }

    pub fn new_right_bracket(right_bracket: &str) -> Result<Self, std::io::Error> {
        match right_bracket.parse::<char>() {
            Ok(x) => Ok(Token::RightBracket(x)),
            Err(_) => {
                let error_message: String = format!("{right_bracket}: Not parseable to char");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }
}
