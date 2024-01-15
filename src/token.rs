#[derive(Debug)]
pub enum Token {
    // TODO make tokens hold &strs that have lifetime equal to the token enum
    /* Numeric Literals */
    IntegerLiteral(isize),
    FloatLiteral(f64),
    Punctuation(char),
    Identifier(String),
    /* Class Regions */
    PublicClassRegion(&'static str),
    PrivateClassRegion(&'static str),
    /* Declarations */
    ClassDeclaration(&'static str),
    FunctionDeclaration(&'static str),
    MethodDeclaration(&'static str),
    ConstantDeclaration(&'static str),
    VariableDeclaration(&'static str),
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
    IncrementAndAssign(&'static str),
    DecrementAndAssign(&'static str),
    MultiplyAndAssign(&'static str),
    DivideAndAssign(&'static str),
    ReturnTypeArrow(&'static str),
    EqualityCheck(&'static str),
    /* Grouping Symbols */
    LeftParenthesis(char),
    RightParenthesis(char),
    /* Types */
    UnsignedEightBitInteger(&'static str),
    UnsignedSixteenBitInteger(&'static str),
    UnsignedThirtyTwoBitInteger(&'static str),
    UnsignedSixtyFourBitInteger(&'static str),
    SignedEightBitInteger(&'static str),
    SignedSixteenBitInteger(&'static str),
    SignedThirtyTwoBitInteger(&'static str),
    SignedSixtyFourBitInteger(&'static str),
    Boolean(&'static str),
    UnsignedSize(&'static str),
    SignedSize(&'static str),
    /* Scope Edge */
    LeftBracket(char),
    RightBracket(char),
}
