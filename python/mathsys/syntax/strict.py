#
#   SYNTAX
#

# SYNTAX -> VARIABLE
syntax = r"""
sheet: NEWLINE* (level2 (NEWLINE+ level2)*)? NEWLINE*

declaration: IDENTIFIER EQUALITY expression
node: expression
equation: expression EQUALITY expression
comment: QUOTE

expression: term+

term: factor (OPERATOR factor)*

factor: SIGNS? (NUMBER | IDENTIFIER | (OPEN expression CLOSE) | vector) (EXPONENT expression EXPONENT)?

vector: ENTER (expression (COMMA expression)*)? EXIT

level1: sheet
level2: (declaration | node | equation | comment)
level3: expression
level4: term
level5: factor
level6: vector


QUOTE: /\#[^\n]*/
IDENTIFIER: /[A-Za-z]+/
EXPONENT: /\^/
NUMBER: /[0-9]+(\.[0-9]+)?/
NEWLINE: /\n+/
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