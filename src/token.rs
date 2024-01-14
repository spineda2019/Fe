#[derive(Debug)]
pub enum Token {
    // TODO make tokens hold &strs that have lifetime equal to the token enum
    /* Numeric Literals */
    IntegerLiteral(isize),
    FloatLiteral(f64),
    Punctuation(char),
    Identifier(String),
    /* Class Regions */
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
    GreaterThan(char),
    LessThan(char),
    /* Compound Operators */
    IncrementAndAssign(String),
    DecrementAndAssign(String),
    MultiplyAndAssign(String),
    DivideAndAssign(String),
    ReturnTypeArrow(String),
    EqualityCheck(String),
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
