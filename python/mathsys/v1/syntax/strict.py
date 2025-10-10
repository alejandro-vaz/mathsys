#
#   SYNTAX
#

# SYNTAX -> VARIABLE
syntax = r"""
start: (_S | _L)* (level1 _S? (_L+ level1)*)? (_S | _L)*

debug: _COMMAND _DEBUG
declaration: variable _S? _EQUALITY _S?  expression
definition: variable _S? _BINDING _S? expression
node: expression
equation: expression _S? _EQUALITY _S? expression
comment: _COMMAND QUOTE

expression: (SIGNS _S?)? level3 (_S? SIGNS _S? level3)*

term: level4 ((_S? OPERATOR)? _S? level4)*

factor: level5 (_EXPONENTIATION _S? expression _S? _EXPONENTIATION)?

infinite: _INF
limit: _LIM _S variable _S? _TO _S? expression SIGN? _S _OF _S nest
variable: IDENTIFIER
nest: _OPEN _S? expression _S? _CLOSE
vector: _ENTER _S? (expression (_S? _COMMA _S? expression)* _S?)? _EXIT
number: NUMBER (_DOT NUMBER)?


level1: (debug | declaration | definition | node | equation | comment)
level2: (expression)
level3: (term)
level4: (factor)
level5: (infinite | limit | variable | nest | vector | number)


_INF: /inf/
_LIM: /lim/
_TO: /->/
_OF: /of/
_DEBUG: /debug/
_COMMAND: /\#/
QUOTE: / [^\n]+/
IDENTIFIER: /[A-Za-z]+/
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