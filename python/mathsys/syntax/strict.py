#
#   SYNTAX
#

# SYNTAX -> VARIABLE
syntax = r"""
sheet: (declaration (NEWLINE declaration)*)?

declaration: IDENTIFIER EQUALITY expression

expression: (term | brackets | variable)*

term: SIGNS? NUMBER
variable: SIGNS? IDENTIFIER
brackets: SIGNS? OPEN expression CLOSE


IDENTIFIER: /[A-Za-z]+/
NUMBER: /[0-9]+(\.[0-9]+)?/
NEWLINE: /\n+/
EQUALITY: /=/
SIGNS: /[+-]+(\s*[+-]+)*/
OPEN: /\(/
CLOSE: /\)/
SPACE: / +/

%ignore SPACE
%ignore NEWLINE
"""