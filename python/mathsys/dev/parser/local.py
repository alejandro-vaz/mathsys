#^
#^  HEAD
#^

#> HEAD -> MODULES
from lark import Token

#> HEAD -> LIBRARY
from ..common import (
    standard
)


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
comment: _COMMAND _S QUOTE?
use: _USE _S MODULE

expression: (SIGNS _S?)? level3 (_S? SIGNS _S? level3)*

term: level4 ((_S? OPERATOR)? _S? level4)*

factor: level5 (_EXPONENTIATION _S? expression _S? _EXPONENTIATION)?
limit: _LIM _S variable _S? _TO _S? expression SIGN? _S _OF _S nest (_EXPONENTIATION _S? expression _S? _EXPONENTIATION)?

variable: IDENTIFIER
infinite: _INF
nest: _OPEN _S? expression? _S? _CLOSE
tensor: _ENTER _S? (expression (_S? _COMMA _S? expression)* _S?)? _EXIT
number: NUMBER (_DOT NUMBER)?


level1: (declaration | definition | annotation | node | equation | comment | use)
level2: (expression)
level3: (term)
level4: (factor | limit)
level5: (variable | infinite | nest | tensor | number)


_LIM: /\blim\b/
_TO: /->/
_OF: /\bof\b/
_USE: /\buse\b/
_COMMAND: /\#/
QUOTE: /[^\n]+/
MODULE: /[a-z]+/
OBJECT: /\b(Any|Infinite|Nexists|Number|Tensor|Undefined|Variable)\b/
IDENTIFIER: /(?i)(?!\b(?:inf|of|use|lim|Any|Infinite|Nexists|Number|Tensor|Undefined|Variable)\b)[A-Za-zº$%]+/
_INF: /\binf\b/
_EXPONENTIATION: /\^/
NUMBER: /[0-9]+/
_DOT: /\./
_BINDING: /==/
_EQUALITY: /=/
OPERATOR: /[\*\/]/
SIGNS: /[+-]+(\s*[+-]+)*/
SIGN: /[+-]/
_OPEN: /\(/
_CLOSE: /\)/
_ENTER: /\[/
_COMMA: /,/
_EXIT: /\]/
_S: / +/
_L: /\n/
"""


#^
#^  LIBRARY
#^

#> LIBRARY -> MODULES
MODULES = {
    "standard": standard.data
}


#^
#^  HELPERS
#^

#> HELPERS -> TOKEN TRIMMER
def ñ(token: Token) -> str: return token.value.replace(" ", "")