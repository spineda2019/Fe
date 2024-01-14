#[derive(Debug)]
pub enum Token {
    /* Numeric Literals */
    IntegerLiteral(isize),
    FloatLiteral(f64),
    Punctuation(char),
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
