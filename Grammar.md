# Formal Grammar
This is going to be the (working) document that describes Fe's formal grammar.
This process is very complex, and will take some time.

## The Grammar
Keep in mind that for the purpose of this project, "::=" reads as "is a", can't
recall if this is completely accurate in the mathematical world, but this will
suffice for now.

```txt
<program> ::= {<statement>}*
<statement> ::= <assignment> | <expression> | <file-dropin>
<assignment> ::= <mutability-specifier> <identifier> = <expression>: <type> <punctuation>
<mutability-specifier> ::= constant | variable
<identifier> ::= <letter> | <letter> <identifier>
<letter> ::= a | b | c | ... | z | A | B | C | ... | Z
<expression> ::= <term> | <unary-operator> <term> | <term> <binary-operator> <term> | <term> <postfix-compound-operator>
<term> ::= <factor> | <term> <binary-operator> <factor> | <factor> <binary-operator> <term>
<factor> ::= (<expression>) | <number> | <identifier>
<number> ::= <integer> | <floating-point>
<integer> ::= <digit> | <digit> <integer>
<digit> ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
<floating-point> ::= <integer> . <integer>
<unary-operator> ::= + | - | !
<binary-operator> ::= + | - | and | or | * | / | ->
<postfix-compound-operator> ::= ++ | --
<type> ::= uint8 | sint8 | uint16 | sint16 | uint32 | sint32 | uint64 | sint64 | boolean | ssize | usize
<punctuation> ::= ;
<file-dropin> ::= dropin "<file-path>"
<file-path> ::> <file-name> .fe | <folder-structure> <file-name> .fe
<file-name> ::> <letter> | <letter> <file-name>
<folder-structure> ::= <file-name> / | <folder-structure> <file-name> /
```
