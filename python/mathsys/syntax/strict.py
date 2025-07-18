#
#   SYNTAX
#

# SYNTAX -> VARIABLE
syntax = r"""
sheet: NEWLINE* (declaration (NEWLINE+ declaration)*)? NEWLINE*

declaration: IDENTIFIER EQUALITY expression

expression: term*

term: SIGNS? (NUMBER | IDENTIFIER | (OPEN expression CLOSE))


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