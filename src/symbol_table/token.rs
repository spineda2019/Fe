pub enum Token {
    NumberLiteral(isize),
    Identifier((String, isize)), // identifier, table_pointer
    Operator(char),
}
