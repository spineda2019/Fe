pub mod token;
use token::Token;

pub struct SymbolTable {
    table: std::collections::HashMap<Token, Option<String>>,
}

impl SymbolTable {
    pub fn init() -> Self {
        let map: std::collections::HashMap<Token, Option<String>> =
            std::collections::HashMap::<Token, Option<String>>::new();
        SymbolTable { table: map }
    }
}
