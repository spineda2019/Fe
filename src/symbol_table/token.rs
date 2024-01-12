use std::io::Error;

#[derive(Debug)]
pub enum Token {
    NumberLiteral(isize),
    Identifier(String), // identifier, table_pointer
    Operator(char),
    TypeName(String),
    Punctuation(char),
    GroupingSymbol(char),
    DeclarationKeyword(String),
    ClassRegion(String),
    ScopeEdge(char),
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
        match operator.parse::<char>() {
            Ok(x) => Ok(Token::Operator(x)),
            Err(_) => {
                let error_message: String = format!("{operator}: Not parseable to char");
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

    pub fn new_grouping_symbol(grouping_symbol: &str) -> Result<Self, std::io::Error> {
        match grouping_symbol.parse::<char>() {
            Ok(x) => Ok(Token::GroupingSymbol(x)),
            Err(_) => {
                let error_message: String = format!("{grouping_symbol}: Not parseable to char");
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

    pub fn new_scope_edge(scope_edge: &str) -> Result<Self, std::io::Error> {
        match scope_edge.parse::<char>() {
            Ok(x) => Ok(Token::ScopeEdge(x)),
            Err(_) => {
                let error_message: String = format!("{scope_edge}: Not parseable to char");
                Err(Error::new(std::io::ErrorKind::Interrupted, error_message))
            }
        }
    }
}
