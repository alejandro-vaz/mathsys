#
#   SYNTAX
#

# SYNTAX -> VARIABLE
syntax = r"""
sheet: NEWLINE* (level2 (NEWLINE+ level2)*)? NEWLINE*

definition: IDENTIFIER BINDING expression
declaration: IDENTIFIER EQUALITY expression
node: expression
equation: expression EQUALITY expression
comment: QUOTE

expression: term+

term: level5 (OPERATOR level5)*

infinite: SIGNS? INF (EXPONENTIATION expression EXPONENTIATION)?
limit: SIGNS? LIM IDENTIFIER TO expression OF OPEN expression CLOSE
variable: SIGNS? IDENTIFIER (EXPONENTIATION expression EXPONENTIATION)?
nest: SIGNS? OPEN expression CLOSE (EXPONENTIATION expression EXPONENTIATION)?
vector: SIGNS? ENTER (expression (COMMA expression)*)? EXIT (EXPONENTIATION expression EXPONENTIATION)?
number: SIGNS? NUMBER (EXPONENTIATION expression EXPONENTIATION)?


level1: sheet
level2: (definition | declaration | node | equation | comment)
level3: expression
level4: term
level5: (infinite | limit | variable | nest | vector | number)


INF: /inf/
LIM: /lim/
TO: /->/
OF: /of/
QUOTE: /\#( [^\n]*)?/
IDENTIFIER: /[A-Za-z]+/
EXPONENTIATION: /\^/
NUMBER: /[0-9]+(\.[0-9]+)?/
NEWLINE: /\n+/
BINDING: /==/
EQUALITY: /=/
OPERATOR: /[·\*\/]/
SIGNS: /[+-]+(\s*[+-]+)*/
OPEN: /\(/
CLOSE: /\)/
ENTER: /\[/
COMMA: /,/
EXIT: /\]/
SPACE: / +/

%ignore SPACE
"""