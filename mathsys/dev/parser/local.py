#^
#^  HEAD
#^

#> HEAD -> MODULES
import os
from lark import Token


#^
#^  SYNTAX
#^

#> SYNTAX -> VARIABLE
SYNTAX = r"""
start: (_S | _L)* (level1 _S? (_L+ level1 _S?)*)? (_S | _L)*

declaration: (OBJECT _S)? variable _S? _EQUALITY _S?  expression
definition: (OBJECT _S)? variable _S? _BINDING _S? expression
annotation: OBJECT _S variable (_S? _COMMA _S? variable)*
node: expression
equation: expression _S? _EQUALITY _S? expression
comment: QUOTE
use: _USE _S MODULE

expression: (SIGN _S?)? level3 (_S? SIGN _S? level3)*

term: level4 ((_S? OPERATOR)? _S? level4)*

factor: level5 (_EXPONENTIATION _S? expression _S? _EXPONENTIATION)?
limit: _LIM _S variable _S? _TO _S? expression SIGN? _S _OF _S nest (_EXPONENTIATION _S? expression _S? _EXPONENTIATION)?

variable: IDENTIFIER
infinite: _INF
nest: _OPEN _S? expression? _S? _CLOSE
tensor: _ENTER _S? (expression (_S? _COMMA _S? expression)* _S?)? _EXIT
whole: NUMBER
absolute: _PIPE _S? expression _S? _PIPE


level1: (declaration | definition | annotation | node | equation | comment | use)
level2: (expression)
level3: (term)
level4: (factor | limit)
level5: (variable | infinite | nest | tensor | whole | absolute)


_LIM: /\blim\b/
_PIPE: /\|/
_TO: /->/
_OF: /\bof\b/
_USE: /\buse\b/
IDENTIFIER: /(?!\b(?:inf|of|use|lim|Infinite|Integer|Natural|Nexists|Tensor|Undefined|Variable|Whole)\b)[A-Za-zº$%]+/
OBJECT: /\@(?:Infinite|Integer|Natural|Nexists|Tensor|Undefined|Variable|Whole)\b/
_INF: /\binf\b/
_EXPONENTIATION: /\^/
NUMBER: /[0-9]+/
_DOT: /\./
_BINDING: /==/
_EQUALITY: /=/
OPERATOR: /[\*\/]/
SIGN: /[+-]/
_OPEN: /\(/
_CLOSE: /\)/
_ENTER: /\[/
_COMMA: /,/
_EXIT: /\]/
_S: / +/
_L: /\n/
MODULE: /"[a-z]+"/
QUOTE: /\#(?: [^\n]*)?/
"""


#^
#^  LIBRARY
#^

#> LIBRARY -> READ
def read(name):
    path = os.path.join(os.path.join(os.path.dirname(__file__), "..", "common"), f"{name}.msX")
    with open(path) as file:
        return file.read()

#> LIBRARY -> MODULES
MODULES = {
    "standard": read("standard")
}


#^
#^  HELPERS
#^

#> HELPERS -> TOKEN TRIMMER
def ñ(token: Token) -> str: return token.value.replace(" ", "")