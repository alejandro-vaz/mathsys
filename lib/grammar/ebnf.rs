//^
//^ EBNF
//^

//> EBNF -> DEFINITION
pub static EBNF: &'static str = "
Start -> (NEWLINES? Level1 (NEWLINES Level1)*)? NEWLINES? ENDOFFILE

Definition -> Variable DEFINITION Level2
Function -> Variable OPEN (Variable (COMMA Variable)*)? CLOSE DEFINITION Level2
Node -> Level2
Equation -> Level2 EQUALITY Level2
Use -> USE MODULE

Expression -> SIGN* Level3 (SIGN+ Level3)*

Term -> Level4 (OPERATOR? Level4)*

Factor -> Level5 (EXPONENTIATION Level2 EXPONENTIATION)?
Limit -> LIMIT Variable TO Level2 SIGN? OF Nest (EXPONENTIATION Level2 EXPONENTIATION)?

Infinite -> INFINITE
Variable -> IDENTIFIER
Nest -> OPEN Level2? CLOSE
Vector -> ENTER (Level2 (COMMA Level2)*)? EXIT
Whole -> NUMBER
Absolute -> PIPE Level2 PIPE
Undefined -> UNDEFINED
Rational -> RATIONAL
Call -> Variable OPEN (Level2 (COMMA Level2)*)? CLOSE

Level1 -> Definition | Function | Node | Equation | Use
Level2 -> Expression
Level3 -> Term
Level4 -> Factor | Limit
Level5 -> Infinite | Variable | Nest | Vector | Whole | Absolute | Undefined | Rational | Call
";