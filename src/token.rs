use std::io::Error;

#[derive(Debug)]
pub enum Token {
    Punctuation(char),
    ClassRegion(String),
    NumberLiteral(isize),
    Identifier(String),
    /* Declarations */
    ClassDeclaration(String),
    FunctionDeclaration(String),
    MethodDeclaration(String),
    ConstantDeclaration(String),
    VariableDeclaration(String),
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
    /* Types */
    UnsignedEightBitInteger(String),
    UnsignedSixteenBitInteger(String),
    UnsignedThirtyTwoBitInteger(String),
    UnsignedSixtyFourBitInteger(String),
    SignedEightBitInteger(String),
    SignedSixteenBitInteger(String),
    SignedThirtyTwoBitInteger(String),
    SignedSixtyFourBitInteger(String),
    Boolean(String),
    UnsignedSize(String),
    SignedSize(String),
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

    pub fn new_operator(operator: &str) -> Result<Self, std::io::Error> {
        if let Ok(op) = operator.parse::<char>() {
            match op {
                '+' => Ok(Token::PlusSign('+')),
                '-' => Ok(Token::MinusSign('-')),
                '*' => Ok(Token::MultiplicationSign('*')),
                '/' => Ok(Token::DivisionSign('/')),
                '=' => Ok(Token::EqualSign('=')),
                ':' => Ok(Token::Colon(':')),
                _ => {
                    let error_message: String = format!("Not a valid operator: {operator}");
                    Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
                }
            }
        } else {
            let error_message: String = format!("{operator}: Not parseable to char");
            Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
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
        match type_name {
            "uint8" => Ok(Token::UnsignedEightBitInteger("uint8".to_string())),
            "uint16" => Ok(Token::UnsignedSixteenBitInteger("uint16".to_string())),
            "uint32" => Ok(Token::UnsignedThirtyTwoBitInteger("uint32".to_string())),
            "uint64" => Ok(Token::UnsignedSixtyFourBitInteger("uint64".to_string())),
            "sint8" => Ok(Token::SignedEightBitInteger("sint8".to_string())),
            "sint16" => Ok(Token::SignedSixteenBitInteger("sint16".to_string())),
            "sint32" => Ok(Token::SignedThirtyTwoBitInteger("sint32".to_string())),
            "sint64" => Ok(Token::SignedSixtyFourBitInteger("sint64".to_string())),
            "ssize" => Ok(Token::SignedSize("ssize".to_string())),
            "usize" => Ok(Token::UnsignedSize("usize".to_string())),
            "boolean" => Ok(Token::Boolean("boolean".to_string())),
            _ => {
                let error_message: String = format!("Not a valid type: {type_name}");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
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
        match declaration_keyword {
            "class" => Ok(Token::ClassDeclaration("class".to_string())),
            "function" => Ok(Token::FunctionDeclaration("function".to_string())),
            "method" => Ok(Token::MethodDeclaration("method".to_string())),
            "constant" => Ok(Token::ConstantDeclaration("constant".to_string())),
            "variable" => Ok(Token::VariableDeclaration("variable".to_string())),
            _ => {
                let error_message: String = format!("{declaration_keyword}: Bad Declaration");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
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
