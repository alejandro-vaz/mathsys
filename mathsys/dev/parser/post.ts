//^
//^ HEAD
//^

//> HEAD -> DATA
import * as parser from "./dataclasses.js";
import {ñ, Token, istoken} from "./local.js";


//^
//^ START
//^

//> START -> POSTPROCESS
export function start(items: parser.Level1[]): parser.Start {
    return new parser.Start(
        items
    );
}


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> DECLARATION
export function declaration(items: (Token | parser.Variable | parser.Expression)[]): parser.Declaration {
    return new parser.Declaration(
        items.length == 3 ? ñ(items[0] as Token) : null,
        items.length == 3 ? items[1] as parser.Variable : items[0] as parser.Variable,
        items.length == 3 ? items[2] as parser.Expression : items[1] as parser.Expression
    );
}

//> 1ºLEVEL -> DEFINITION
export function definition(items: (Token | parser.Variable | parser.Expression)[]): parser.Definition {
    return new parser.Definition(
        items.length == 3 ? ñ(items[0] as Token) : null,
        items.length == 3 ? items[1] as parser.Variable : items[0] as parser.Variable,
        items.length == 3 ? items[2] as parser.Expression : items[1] as parser.Expression
    );
}

//> 1ºLEVEL -> ANNOTATION
export function annotation(items: (Token | parser.Variable)[]): parser.Annotation {
    return new parser.Annotation(
        ñ(items[0] as Token),
        items.filter(item => item instanceof parser.Variable)
    );
}

//> 1ºLEVEL -> NODE
export function node(items: parser.Expression[]): parser.Node {
    return new parser.Node(
        items[0]
    );
}

//> 1ºLEVEL -> EQUATION
export function equation(items: parser.Expression[]): parser.Equation {
    return new parser.Equation(
        items[0],
        items[1]
    );
}

//> 1ºLEVEL -> COMMENT
export function comment(items: Token[]): parser.Comment {
    return new parser.Comment(
        items[0].value.slice(1).trim()
    );
}

//> 1ºLEVEL -> USE
export function use(items: Token[]): parser.Use {
    return new parser.Use(
        ñ(items[0]).slice(1, -1),
        null
    );
}


//^
//^ 2ºLEVEL
//^

//> 2ºLEVEL -> EXPRESSION
export function expression(items: (Token | parser.Level3)[]): parser.Expression {
    return new parser.Expression(
        [...(istoken(items[0]) ? [] : [null]), ...items.filter(item => istoken(item)).map(item => ñ(item as Token) === "+")],
        items.filter(item => item instanceof parser.Level3)
    );
}


//^
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> TERM
export function term(items: (Token | parser.Level4)[]): parser.Term {
    const numerator = [];
    const denominator = [];
    let location = true;
    for (const item of items) {if (istoken(item)) {switch (ñ(item as Token)) {
        case "*": {location = true; break};
        case "/": {location = false; break};
    }} else {location ? numerator.push(item) : denominator.push(item)}}
    return new parser.Term(
        numerator,
        denominator
    );
}


//^
//^ 4ºLEVEL
//^

//> 4ºLEVEL -> FACTOR
export function factor(items: (parser.Level5 | parser.Expression)[]): parser.Factor {
    return new parser.Factor(
        items[0],
        items.length == 2 ? items[1] as parser.Expression : null
    );
}

//> 4ºLEVEL -> LIMIT
export function limit(items: (Token | parser.Variable | parser.Expression | parser.Nest)[]): parser.Limit {
    return new parser.Limit(
        items[0] as parser.Variable,
        items[1] as parser.Expression,
        istoken(items[2]) ? ñ(items[2] as Token) == "+" : null,
        items.at(-2) instanceof parser.Nest ? items.at(-2) as parser.Nest : items.at(-1) as parser.Nest,
        items.at(-1) instanceof parser.Expression ? items.at(-1) as parser.Expression : null
    );
}


//^
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> INFINITE
export function infinite(items: []): parser.Infinite {
    return new parser.Infinite();
}

//> 5ºLEVEL -> VARIABLE
export function variable(items: Token[]): parser.Variable {
    return new parser.Variable(
        ñ(items[0] as Token)
    );
}

//> 5ºLEVEL -> NEST
export function nest(items: parser.Expression[]): parser.Nest {
    return new parser.Nest(
        items.length == 1 ? items[0] : null
    );
}

//> 5ºLEVEL -> TENSOR
export function tensor(items: parser.Expression[]): parser.Tensor {
    return new parser.Tensor(
        items
    );
}

//> 5ºLEVEL -> NATURAL
export function natural(items: Token[]): parser.Natural {
    return new parser.Natural(
        +ñ(items[0])
    );
}