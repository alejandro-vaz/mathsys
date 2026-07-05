//^
//^ GRAMMr
//^

//> GRAMMAR -> ARRAY
pub static GRAMMAR: [&'static str; 53] = [
    "Start: $6 $4 ENDOFFILE",

    "Definition: Variable DEFINITION Level2",
    "Function: Variable OPEN $10 CLOSE DEFINITION Level2",
    "Node: Level2",
    "Equation: Level2 EQUALITY Level2",
    "Use: USE MODULE",

    "Expression: $15 Level3 $16",

    "Term: Level4 $20",

    "Factor: Level5 $22",
    "Limit: LIMIT Variable TO Level2 $23 OF Nest $22",

    "Infinite: INFINITE",
    "Variable: IDENTIFIER",
    "Nest: OPEN $24 CLOSE",
    "Vector: ENTER $28 EXIT",
    "Whole: NUMBER",
    "Absolute: PIPE Level2 PIPE",
    "Undefined: UNDEFINED",
    "Rational: RATIONAL",
    "Call: Variable OPEN $28 CLOSE",

    "Level1: Definition | Function | Node | Equation | Use",
    "Level2: Expression",
    "Level3: Term",
    "Level4: Factor | Limit",
    "Level5: Infinite | Variable | Nest | Vector | Whole | Absolute | Undefined | Rational | Call",
    
    "$0: Start",
    "$1: NEWLINES Level1 $2",
    "$2: SPACES |",
    "$3: $4 Level1 $5",
    "$4: NEWLINES |",
    "$5: $1 $5 |",
    "$6: $3 |",
    "$7: COMMA Variable",
    "$8: Variable $9 $2",
    "$9: $7 $9 |",
    "$10: $8 |",
    "$11: SIGN $2",
    "$12: SIGN",
    "$13: $14 Level3",
    "$14: $12 $14 | $12",
    "$15: $11 $15 |",
    "$16: $13 $16 |",
    "$17: OPERATOR",
    "$18: $19 Level4",
    "$19: $17 |",
    "$20: $18 $20 |",
    "$21: EXPONENTIATION Level2 EXPONENTIATION",
    "$22: $21 |",
    "$23: SIGN |",
    "$24: Level2 |",
    "$25: COMMA Level2",
    "$26: Level2 $27 $2",
    "$27: $25 $27 |",
    "$28: $26 |"
];