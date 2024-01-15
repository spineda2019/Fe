#[derive(Debug)]
pub enum Token<'a> {
    // TODO make tokens hold &strs that have lifetime equal to the token enum
    /* Numeric Literals */
    IntegerLiteral(isize),
    FloatLiteral(f64),
    Punctuation(char),
    Identifier(String),
    /* Class Regions */
    PublicClassRegion(&'a str),
    PrivateClassRegion(&'a str),
    /* Declarations */
    ClassDeclaration(&'a str),
    FunctionDeclaration(&'a str),
    MethodDeclaration(&'a str),
    ConstantDeclaration(&'a str),
    VariableDeclaration(&'a str),
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
    IncrementAndAssign(&'a str),
    DecrementAndAssign(&'a str),
    MultiplyAndAssign(&'a str),
    DivideAndAssign(&'a str),
    ReturnTypeArrow(&'a str),
    EqualityCheck(&'a str),
    /* Grouping Symbols */
    LeftParenthesis(char),
    RightParenthesis(char),
    /* Types */
    UnsignedEightBitInteger(&'a str),
    UnsignedSixteenBitInteger(&'a str),
    UnsignedThirtyTwoBitInteger(&'a str),
    UnsignedSixtyFourBitInteger(&'a str),
    SignedEightBitInteger(&'a str),
    SignedSixteenBitInteger(&'a str),
    SignedThirtyTwoBitInteger(&'a str),
    SignedSixtyFourBitInteger(&'a str),
    Boolean(&'a str),
    UnsignedSize(&'a str),
    SignedSize(&'a str),
    /* Scope Edge */
    LeftBracket(char),
    RightBracket(char),
}
