@{%
//^
//^ SYNTAX
//^

//> SYNTAX -> LEXER
const lexer = require("moo").compile({
    _UNDEFINED: /\?/,
    _LIM: /lim/,
    _PIPE: /\|/,
    _TO: /->/,
    _OF: /of/,
    _INF: /inf/,
    _USE: /use/,
    TYPE: /(?:Infinite|Integer|Natural|Nexists|Rational|Tensor|Type|Undefined|Variable|Whole)/,
    IDENTIFIER: /[A-Za-zº$%]+/,
    _AT: /\@/,
    _EXPONENTIATION: /\^/,
    RATIONAL: /[0-9]*\.[0-9]+/,
    NUMBER: /[0-9]+/,
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
    _L: /\n+/,
    MODULE: /"[a-z]+"/,
    QUOTE: /\#(?:[^\n]*)?/
});

//> SYNTAX -> POSTPROCESSING
const post = require("./post.js");

//> SYNTAX -> DELETER
export function del(list: any[]): any[] {
    const stack = [...list];
    const result: any[] = [];
    while (stack.length > 0) {
        const item = stack.shift();
        if (item === null || item === undefined) continue;
        if (Array.isArray(item)) {
            stack.unshift(...item);
        } else if (istoken(item)) {
            if (!item.type.startsWith("_")) result.push(item);
        } else {
            result.push(item);
        }
    }
    return result;
}
%}

@lexer lexer

start -> ((%_S | %_L):* (level2 | %QUOTE)):* (%_S | %_L):* {% data => post.start(del(data)) %}

declaration -> (grouping %_S):? variable %_S:? %_EQUALITY %_S:? level2 {% data => post.declaration(del(data)) %}
definition -> (grouping %_S):? variable %_S:? %_BINDING %_S:? level2 {% data => post.definition(del(data)) %}
annotation -> grouping %_S variable (%_S:? %_COMMA %_S:? variable):* {% data => post.annotation(del(data)) %}
node -> level2 {% data => post.node(del(data)) %}
equation -> level2 %_S:? %_EQUALITY %_S:? level2 {% data => post.equation(del(data)) %}
use -> %_USE %_S %MODULE  {% data => post.use(del(data)) %}

expression -> (%SIGN %_S:?):? level3 (%_S:? %SIGN %_S:? level3):* {% data => post.expression(del(data)) %}
grouping -> %_AT %TYPE {% data => post.grouping(del(data)) %}

term -> level4 ((%_S:? %OPERATOR):? %_S:? level4):* {% data => post.term(del(data)) %}

factor -> level5 (%_EXPONENTIATION %_S:? level2 %_S:? %_EXPONENTIATION):? {% data => post.factor(del(data)) %}
limit -> %_LIM %_S variable %_S:? %_TO %_S:? level2 %SIGN:? %_S %_OF %_S nest (%_EXPONENTIATION %_S:? level2 %_S:? %_EXPONENTIATION):? {% data => post.limit(del(data)) %}

variable -> %IDENTIFIER {% data => post.variable(del(data)) %}
infinite -> %_INF {% data => post.infinite(del(data)) %}
nest -> %_OPEN %_S:? level2:? %_S:? %_CLOSE {% data => post.nest(del(data)) %}
tensor -> %_ENTER %_S:? (level2 (%_S:? %_COMMA %_S:? level2):* %_S:?):? %_EXIT {% data => post.tensor(del(data)) %}
whole -> %NUMBER {% data => post.whole(del(data)) %}
absolute -> %_PIPE %_S:? level2 %_S:? %_PIPE {% data => post.absolute(del(data)) %}
undefined -> %_UNDEFINED {% data => post.undefined(del(data)) %}
rational -> %RATIONAL {% data => post.rational(del(data)) %}


level1 -> (declaration | definition | annotation | node | equation | use) {% id %}
level2 -> (expression | grouping) {% id %}
level3 -> (term) {% id %}
level4 -> (factor | limit) {% id %}
level5 -> (variable | infinite | nest | tensor | whole | absolute | undefined | rational) {% id %}