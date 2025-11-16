#^
#^  SYNTAX
#^

#> SYNTAX -> VARIABLE
syntax = r"""
start: (_S | _L)* (level1 _S? (_L+ level1)*)? (_S | _L)*

declaration: variable _S? _EQUALITY _S?  expression
definition: variable _S? _BINDING _S? expression
node: expression
equation: expression _S? _EQUALITY _S? expression
comment: _COMMAND _S QUOTE?

expression: (SIGNS _S?)? level3 (_S? SIGNS _S? level3)*

term: level4 ((_S? OPERATOR)? _S? level4)*

factor: level5 (_EXPONENTIATION _S? expression _S? _EXPONENTIATION)?
limit: _LIM _S variable _S? _TO _S? expression SIGN? _S _OF _S nest (_EXPONENTIATION _S? expression _S? _EXPONENTIATION)?

variable: IDENTIFIER
infinite: _INF
nest: _OPEN _S? expression? _S? _CLOSE
vector: _ENTER _S? (expression (_S? _COMMA _S? expression)* _S?)? _EXIT
number: NUMBER (_DOT NUMBER)?


level1: (declaration | definition | node | equation | comment)
level2: (expression)
level3: (term)
level4: (factor | limit)
level5: (variable | infinite | nest | vector | number)


_LIM: /lim/
_TO: /->/
_OF: /of/
_COMMAND: /\#/
QUOTE: /[^\n]+/
IDENTIFIER: /(?i)(?!\b(?:inf|of|lim)\b)[A-Za-zº$%]+/
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