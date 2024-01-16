# Formal Grammar
This is going to be the (working) document that describes Fe's formal grammar.
This process is very complex, and will take some time.

## The Grammar
Keep in mind that for the purpose of this project, "::=" reads as "is a", can't
recall if this is completely accurate in the mathematical world, but this will
suffice for now.

```txt
<executable> ::= <statement>
<statement> ::= <assignment> | <expression>
<assignment> ::= <mutability_specifier> <identifier> = <expression>: <type> <punctuation>
<mutability_specifier> ::= uint8 | sint8 | uint16 | sint16 | uint32 | sint32 | uint64 | sint64 | boolean | ssize | usize
<identifier> ::= <letter> | <letter> <identifier>
<letter> ::= a | b | c | ... | z | A | B | C | ... | Z
```
