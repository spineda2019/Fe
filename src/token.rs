use std::io::Error;

#[derive(Debug)]
pub enum Token {
    Punctuation(char),
    NumberLiteral(isize),
    Identifier(String),
    /* */
    PublicClassRegion(String),
    PrivateClassRegion(String),
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
}
