#
#   SYNTAX
#

# SYNTAX -> VARIABLE
syntax = r"""
sheet: NEWLINE* (declaration (NEWLINE+ declaration)*)? NEWLINE*

declaration: IDENTIFIER EQUALITY expression

expression: term+

term: factor (OPERATOR factor)*

factor: SIGNS? (NUMBER | IDENTIFIER | (OPEN expression CLOSE) | vector)

vector: ENTER (expression (COMMA expression)*)? EXIT


IDENTIFIER: /[A-Za-z]+/
NUMBER: /[0-9]+(\.[0-9]+)?/
NEWLINE: /\n+/
EQUALITY: /=/
OPERATOR: /[\*\/]/
SIGNS: /[+-]+(\s*[+-]+)*/
OPEN: /\(/
CLOSE: /\)/
ENTER: /\[/
COMMA: /,/
EXIT: /\]/
SPACE: / +/

%ignore SPACE
%ignore NEWLINE
"""