#^
#^  HEAD
#^

#> HEAD -> MODULES
import os
from lark import Token
import aiofiles

#> HEAD -> VERSION
from .. import __version__


#^
#^  SYNTAX
#^

#> SYNTAX -> VARIABLE
SYNTAX = r"""
start: _NEWLINES? (level1 _SPACES? (_NEWLINES level1 _SPACES?)* _NEWLINES?)?

declaration: (TYPE _S)? variable _S? _EQUALITY _S? level2
definition: (TYPE _S)? variable _S? _BINDING _S? level2
annotation: TYPE _S variable (_S? _COMMA _S? variable)*
node: level2
equation: level2 _S? _EQUALITY _S? level2
use: _USE _S MODULE

expression: (SIGN _S?)* level3 ((_S? SIGN)+ _S? level3)*

term: level4 ((_S? OPERATOR)? _S? level4)*

factor: level5 (_EXPONENTIATION _S? level2 _S? _EXPONENTIATION)?
limit: _LIM _S variable _S? _TO _S? level2 SIGN? _S _OF _S nest (_EXPONENTIATION _S? level2 _S? _EXPONENTIATION)?

infinite: _INF
variable: IDENTIFIER
nest: _OPEN _S? level2? _S? _CLOSE
tensor: _ENTER _S? (level2 (_S? _COMMA _S? level2)* _S?)? _EXIT
whole: NUMBER
absolute: _PIPE _S? level2 _S? _PIPE
undefined: _UNDEFINED
rational: RATIONAL
casts: level5 TYPE


level1: (declaration | definition | annotation | node | equation | use)
level2: (expression)
level3: (term)
level4: (factor | limit)
level5: (infinite | variable | nest | tensor | whole | absolute | undefined | rational | casts)


_UNDEFINED: /\?/ 
_LIM: /lim/
_PIPE: /\|/
_TO: /->/
_OF: /of/
_INF: /inf/
_USE: /use/
TYPE: /\@(?:Infinite|Integer|Natural|Nexists|Rational|Tensor|Undefined|Variable|Whole)/
IDENTIFIER: /[A-Za-zº$%]+/
_EXPONENTIATION: /\^/
RATIONAL: /[0-9]*\.[0-9]+/
NUMBER: /[0-9]+/
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
_L: /\n+/
MODULE: /"[a-z]+"/
QUOTE: /\#(?:[^\n]*)?/
"""


#^
#^  LIBRARY
#^

#> LIBRARY -> READ
async def read(module: str) -> str:
    async with aiofiles.open(os.path.join(os.path.join(os.path.dirname(__file__), "..", "common"), f"{module}.msX")) as file: return await file.read()


#^
#^  HELPERS
#^

#> HELPERS -> TOKEN TRIMMER
def ñ(token: Token) -> str: return token.value.replace(" ", "")