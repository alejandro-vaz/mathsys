#
#   SYNTAX
#

# SYNTAX -> VARIABLE
syntax = r"""
#
#   CONSTRUCTS
#

# CONSTRUCTS -> 1ºLEVEL
sheet: declaration*

# CONSTRUCTS -> 2ºLEVEL
declaration: KEYWORD IDENTIFIER EQUALITY expression

# CONSTRUCTS -> 3ºLEVEL
expression: (term | brackets | variable)*

# CONSTRUCTS -> 4ºLEVEL
term: SIGNS? NUMBER
variable: SIGNS? IDENTIFIER
brackets: SIGNS? OPEN expression CLOSE


#
#   TOKENS
#

# TOKENS -> ORDERED DEFINITIONS
KEYWORD: /Num(&[a-z]+)?/
IDENTIFIER: /[A-Za-z]+/
NUMBER: /[0-9]+(\.[0-9]+)?/
NEWLINE: /\n+/
EQUALITY: /=/
SIGNS: /[+-]+(\s*[+-]*)*/
OPEN: /\(/
CLOSE: /\)/
SPACE: / +/

%ignore SPACE
%ignore NEWLINE
"""