//^
//^ HEAD
//^

//> HEAD -> DATA
import {Start} from "./start.js";
import {Declaration, Definition, Annotation, Node, Equation, Use, Level1} from "./level1.js";
import {Expression, Level2} from "./level2.js";
import {Term, Level3} from "./level3.js";
import {Factor, Limit, Level4} from "./level4.js";
import {Infinite, Variable, Nest, Tensor, Whole, Absolute, Undefined, Rational, Casts, Level5} from "./level5.js";


//^
//^ EBNF
//^

//> EBNF -> SORT
function ordering(line: string): [number, number | string] {
    const rule = line.split("->")[0].trim();
    return rule.startsWith("$") ? [1, parseInt(rule.slice(1), 10)] : [0, rule];
}

//> EBNF -> CLASS
class Extensor {
    private counter!: number;
    private parentheses!: {[key: string]: string};
    private more!: {[key: string]: string};
    private multiple!: {[key: string]: string};
    private optional!: {[key: string]: string};
    constructor() {this.reset()}
    reset(): void {
        this.counter = 0;
        this.parentheses = {};
        this.more = {};
        this.multiple = {};
        this.optional = {};
    }
    run(ebnf: string): string {
        this.reset();
        const out = new Set<string>();
        for (const line of ebnf.split("\n").map(each => each.trim())) {
            if (!line || line.startsWith("#") || line.startsWith("//")) {continue}
            const [rule, productions] = line.split("->", 2).map(part => part.trim());
            const [body, rules] = this.expand(productions);
            out.add(`${rule} -> ${body}`);
            rules.forEach(each => out.add(each));
        };
        const ordered = [...out];
        ordered.sort((first, second) => {
            const [from, to] = [ordering(first), ordering(second)];
            if (from[0] !== to[0]) {return from[0] - to[0]}
            return typeof from[1] === "number" && typeof to[1] === "number" ? from[1] - to[1] : String(from[1]).localeCompare(String(to[1]))
        });
        return ordered.join("\n");
    }
    expand(expression: string): [string, Set<string>] {
        const rules = new Set<string>();
        while (expression.includes("(")) {this.collapse(expression, rules)}
        expression = this.postfix(expression, rules);
        return [expression.trim(), rules];
    }
    collapse(expression: string, rules: Set<string>): string {
        const hit = /\(([^()]+)\)/.exec(expression);
        if (!hit) {return expression};
        const inner = hit[1];
        const symbol = inner in this.parentheses ? this.parentheses[inner] : this.next();
        const [expanded, newrules] = this.expand(inner);
        newrules.forEach(each => rules.add(each));
        rules.add(`${symbol} -> ${expanded}`);
        return expression.slice(0, hit.index) + symbol + expression.slice(hit.index + hit[0].length);
    }
    postfix(expression: string, rules: Set<string>): string {
        let hit;
        while ((hit = /(?<atom>\$[0-9]+|[A-Za-z_][A-Za-z_0-9]*|'[^']*')(?<operator>[*+?])/.exec(expression))) {
            const atom = hit.groups!["atom"];
            let symbol;
            let production;
            switch (hit.groups!["operator"]) {
                case "+": {
                    symbol = this.more[atom] ??= this.next();
                    production = `${symbol} -> ${atom} ${symbol} | ${atom}`;
                    break;
                }
                case "*": {
                    symbol = this.multiple[atom] ??= this.next();
                    production = `${symbol} -> ${atom} ${symbol} |`;
                    break;
                }
                case "?": {
                    symbol = this.optional[atom] ??= this.next();
                    production = `${symbol} -> ${atom} |`;
                    break;
                }
                default: throw new Issue
            }
            expression = expression.slice(0, hit.index) + symbol + expression.slice(hit.index + hit[0].length);
            rules.add(production);
        }
        return expression;
    }
    next(): string {this.counter++; return `$${this.counter}`}
}


//^
//^ NONTERMINAL
//^

//> NONTERMINALS -> ALL
export const NONTERMINALS = [
    Start,
    Declaration,
    Definition,
    Annotation,
    Node,
    Equation,
    Use,
    Expression,
    Term,
    Factor,
    Limit,
    Infinite,
    Variable,
    Nest,
    Tensor,
    Whole,
    Absolute,
    Undefined,
    Rational,
    Casts,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5
].reduce((accumulated, current) => {
    accumulated[current.name] = current;
    return accumulated;
}, {})


//^
//^ SYNTAX
//^

export const SYNTAX = new Extensor().run(String.raw`
//> SYNTAX -> START
Start -> (_NEWLINES? Level1 _SPACES? (_NEWLINES Level1 _SPACES?)*)? _NEWLINES? _EOF

//> SYNTAX -> 1ºLEVEL
Declaration -> (TYPE _SPACES)? Variable _SPACES? _EQUALITY _SPACES? Level2
Definition -> (TYPE _SPACES)? Variable _SPACES? _BINDING _SPACES? Level2
Annotation -> TYPE _SPACES Variable (_SPACES? _COMMA _SPACES? Variable)*
Node -> Level2
Equation -> Level2 _SPACES? _EQUALITY _SPACES? Level2
Use -> _USE _SPACES MODULE

//> SYNTAX -> 2ºLEVEL
Expression -> (SIGN _SPACES?)* Level3 ((_SPACES? SIGN)+ _SPACES? Level3)*

//> SYNTAX -> 3ºLEVEL
Term -> Level4 ((_SPACES? OPERATOR)? _SPACES? Level4)*

//> SYNTAX -> 4ºLEVEL
Factor -> Level5 (_EXPONENTIATION _SPACES? Level2 _SPACES? _EXPONENTIATION)?
Limit -> _LIM _SPACES Variable _SPACES? _TO _SPACES? Level2 SIGN? _SPACES _OF _SPACES Nest (_EXPONENTIATION _SPACES? Level2 _SPACES? _EXPONENTIATION)?

//> SYNTAX -> 5ºLEVEL
Infinite -> _INF
Variable -> IDENTIFIER
Nest -> _OPEN _SPACES? Level2? _SPACES? _CLOSE
Tensor -> _ENTER _SPACES? (Level2 (_SPACES? _COMMA _SPACES? Level2)* _SPACES?)? _EXIT
Whole -> NUMBER
Absolute -> _PIPE _SPACES? Level2 _SPACES? _PIPE
Undefined -> _UNDEFINED
Rational -> RATIONAL
Casts -> Level5 TYPE

//> SYNTAX -> LEVELS
Level1 -> Declaration | Definition | Annotation | Node | Equation | Use
Level2 -> Expression
Level3 -> Term
Level4 -> Factor | Limit
Level5 -> Infinite | Variable | Nest | Tensor | Whole | Absolute | Undefined | Rational | Casts
`)