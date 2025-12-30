//^
//^ HEAD
//^

//> HEAD -> DATA
import {join, Binary, Opcode, Pointer, Sign, Option, BigUint, String, Group, Vec} from "./local.js";


//^
//^ START
//^

//> START -> CLASS
export class Start {
    binary = (): Binary => join(Start.code, this.statements);
    static code = new Opcode(0x01);
    constructor(
        readonly statements: Vec<Pointer>
    ) {}
}


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> DECLARATION
export class Declaration {
    binary = (): Binary => join(Declaration.code, this.group, this.variable, this.expression);
    static code = new Opcode(0x02);
    constructor(
        readonly group: Group,
        readonly variable: Pointer,
        readonly expression: Pointer
    ) {}
}

//> 1ºLEVEL -> DEFINITION
export class Definition {
    binary = (): Binary => join(Definition.code, this.group, this.variable, this.expression);
    static code = new Opcode(0x03);
    constructor(
        readonly group: Group,
        readonly variable: Pointer,
        readonly expression: Pointer
    ) {}
}

//> 1ºLEVEL -> ANNOTATION
export class Annotation {
    binary = (): Binary => join(Annotation.code, this.group, this.variables);
    static code = new Opcode(0x04);
    constructor(
        readonly group: Group,
        readonly variables: Vec<Pointer>
    ) {}
}

//> 1ºLEVEL -> NODE
export class Node {
    binary = (): Binary => join(Node.code, this.expression);
    static code = new Opcode(0x05);
    constructor(
        readonly expression: Pointer
    ) {}
}

//> 1ºLEVEL -> EQUATION
export class Equation {
    binary = (): Binary => join(Equation.code, this.leftexpression, this.rightexpression);
    static code = new Opcode(0x06);
    constructor(
        readonly leftexpression: Pointer,
        readonly rightexpression: Pointer
    ) {}
}

//> 1ºLEVEL -> COMMENT
export class Comment {
    binary = (): Binary => join(Comment.code, this.text);
    static code = new Opcode(0x07);
    constructor(
        readonly text: String
    ) {}
}

//> 1ºLEVEL -> USE
export class Use {
    binary = (): Binary => join(Use.code, this.name, this.start);
    static code = new Opcode(0x08);
    constructor(
        readonly name: String,
        readonly start: Option<Pointer>
    ) {}
}


//^
//^ 2ºLEVEL
//^

//> 2ºLEVEL -> EXPRESSION
export class Expression {
    binary = (): Binary => join(Expression.code, this.signs, this.terms);
    static code = new Opcode(0x09);
    constructor(
        readonly signs: Vec<Option<Sign>>,
        readonly terms: Vec<Pointer>
    ) {}
}


//^
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> TERM
export class Term {
    binary = (): Binary => join(Term.code, this.numerator, this.denominator);
    static code = new Opcode(0x0A);
    constructor(
        readonly numerator: Vec<Pointer>,
        readonly denominator: Vec<Pointer>
    ) {}
}


//^
//^ 4ºLEVEL
//^

//> 4ºLEVEL -> FACTOR
export class Factor {
    binary = (): Binary => join(Factor.code, this.value, this.exponent);
    static code = new Opcode(0x0B);
    constructor(
        readonly value: Pointer,
        readonly exponent: Option<Pointer>
    ) {}
}

//> 4ºLEVEL -> LIMIT
export class Limit {
    binary = (): Binary => join(Limit.code, this.variable, this.approach, this.direction, this.nest, this.exponent);
    static code = new Opcode(0x0C);
    constructor(
        readonly variable: Pointer,
        readonly approach: Pointer,
        readonly direction: Option<Sign>,
        readonly nest: Pointer,
        readonly exponent: Option<Pointer>
    ) {}
}


//^
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> INFINITE
export class Infinite {
    binary = (): Binary => join(Infinite.code);
    static code = new Opcode(0x0D);
    constructor() {}
}

//> 5ºLEVEL -> VARIABLE
export class Variable {
    binary = (): Binary => join(Variable.code, this.representation);
    static code = new Opcode(0x0E);
    constructor(
        readonly representation: String
    ) {}
}

//> 5ºLEVEL -> NEST
export class Nest {
    binary = (): Binary => join(Nest.code, this.expression);
    static code = new Opcode(0x0F);
    constructor(
        readonly expression: Option<Pointer>
    ) {}
}

//> 5ºLEVEL -> TENSOR
export class Tensor {
    binary = (): Binary => join(Tensor.code, this.values);
    static code = new Opcode(0x10);
    constructor(
        readonly values: Vec<Pointer>
    ) {}
}

//> 5ºLEVEL -> WHOLE
export class Whole {
    binary = (): Binary => join(Whole.code, this.value);
    static code = new Opcode(0x11);
    constructor(
        readonly value: BigUint
    ) {}
}

//> 5ºLEVEL -> ABSOLUTE
export class Absolute {
    binary = (): Binary => join(Absolute.code, this.expression);
    static code = new Opcode(0x12);
    constructor(
        readonly expression: Pointer
    ) {}
}