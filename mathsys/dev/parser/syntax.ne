@{%
const lexer = require("moo").compile({
    _LIM: /\blim\b/,
    _TO: /->/,
    _OF: /\bof\b/,
    _USE: /\buse\b/,
    IDENTIFIER: /(?!\b(?:inf|of|use|lim|Infinite|Natural|Nexists|Tensor|Undefined|Variable)\b)[A-Za-zº$%]+/,
    OBJECT: /\@(?:Infinite|Natural|Nexists|Tensor|Undefined|Variable)\b/,
    _INF: /\binf\b/,
    _EXPONENTIATION: /\^/,
    NUMBER: /[0-9]+/,
    _DOT: /\./,
    _BINDING: /==/,
    _EQUALITY: /=/,
    OPERATOR: /[\*\/]/,
    SIGN: /[+-]/,
    _OPEN: /\(/,
    _CLOSE: /\)/,
    _ENTER: /\[/,
    _COMMA: /,/,
    _EXIT: /\]/,
    _S: / +/,
    _L: {match: /\n/, lineBreaks: true},
    MODULE: /"[a-z]+"/,
    QUOTE: /#(?: [^\n]*)?/
});
const del = require("./local.js").del;
const post = require("./post.js");
%}

@lexer lexer

start -> (%_S | %_L):* (level1 %_S:? (%_L:+ level1 %_S:?):*):? (%_S | %_L):* {% data => post.start(del(data)) %}

declaration -> (%OBJECT %_S):? variable %_S:? %_EQUALITY %_S:?  expression {% data => post.declaration(del(data)) %}
definition -> (%OBJECT %_S):? variable %_S:? %_BINDING %_S:? expression {% data => post.definition(del(data)) %}
annotation -> %OBJECT %_S variable (%_S:? %_COMMA %_S:? variable):* {% data => post.annotation(del(data)) %}
node -> expression {% data => post.node(del(data)) %}
equation -> expression %_S:? %_EQUALITY %_S:? expression {% data => post.equation(del(data)) %}
comment -> %QUOTE {% data => post.comment(del(data)) %}
use -> %_USE %_S %MODULE  {% data => post.use(del(data)) %}

expression -> (%SIGN %_S:?):? level3 (%_S:? %SIGN %_S:? level3):* {% data => post.expression(del(data)) %}

term -> level4 ((%_S:? %OPERATOR):? %_S:? level4):* {% data => post.term(del(data)) %}

factor -> level5 (%_EXPONENTIATION %_S:? expression %_S:? %_EXPONENTIATION):? {% data => post.factor(del(data)) %}
limit -> %_LIM %_S variable %_S:? %_TO %_S:? expression %SIGN:? %_S %_OF %_S nest (%_EXPONENTIATION %_S:? expression %_S:? %_EXPONENTIATION):? {% data => post.limit(del(data)) %}

variable -> %IDENTIFIER {% data => post.variable(del(data)) %}
infinite -> %_INF {% data => post.infinite(del(data)) %}
nest -> %_OPEN %_S:? expression:? %_S:? %_CLOSE {% data => post.nest(del(data)) %}
tensor -> %_ENTER %_S:? (expression (%_S:? %_COMMA %_S:? expression):* %_S:?):? %_EXIT {% data => post.tensor(del(data)) %}
natural -> %NUMBER {% data => post.natural(del(data)) %}


level1 -> (declaration | definition | annotation | node | equation | comment | use) {% id %}
level2 -> (expression) {% id %}
level3 -> (term) {% id %}
level4 -> (factor | limit) {% id %}
level5 -> (variable | infinite | nest | tensor | natural) {% id %}