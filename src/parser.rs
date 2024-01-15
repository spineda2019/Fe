use crate::token::Token;

pub enum AbstractSyntaxTreeNode {
    BinaryOperation {
        operator: Token,
        left_child: Box<AbstractSyntaxTreeNode>,
        right_child: Box<AbstractSyntaxTreeNode>,
    },
    UnaryOperation {
        operator: Token,
        operand: Box<Token>,
    },
    Literal {
        value: Token,
    },
}
