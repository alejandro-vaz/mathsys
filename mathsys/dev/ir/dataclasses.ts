//^
//^ HEAD
//^

//> HEAD -> DATA
import {clamp, join, u8, null8, u32, null32} from "./local.js";


//^
//^ START
//^

//> START -> CLASS
export class Start {
    static code = u8(0x01);
    constructor(
        readonly statements: u32[]
    ) {}
    bytes(): Uint8Array {return clamp(
        Start.code, join(this.statements, null32())
    )}
}


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> DECLARATION
export class Declaration {
    static code = u8(0x02);
    constructor(
        readonly group: u8 | null8,
        readonly variable: u32,
        readonly expression: u32
    ) {}
    bytes(): Uint8Array {return clamp(
        Declaration.code, this.group, this.variable, this.expression
    )}
}

//> 1ºLEVEL -> DEFINITION
export class Definition {
    static code = u8(0x03);
    constructor(
        readonly group: u8 | null8,
        readonly variable: u32,
        readonly expression: u32
    ) {}
    bytes(): Uint8Array {return clamp(
        Definition.code, this.group, this.variable, this.expression
    )}
}

//> 1ºLEVEL -> ANNOTATION
export class Annotation {
    static code = u8(0x04);
    constructor(
        readonly group: u8 | null8,
        readonly variables: u32[]
    ) {}
    bytes(): Uint8Array {return clamp(
        Annotation.code, this.group, join(this.variables, null32())
    )}
}

//> 1ºLEVEL -> NODE
export class Node {
    static code = u8(0x05);
    constructor(
        readonly expression: u32
    ) {}
    bytes(): Uint8Array {return clamp(
        Node.code, this.expression
    )}
}

//> 1ºLEVEL -> EQUATION
export class Equation {
    static code = u8(0x06);
    constructor(
        readonly leftexpression: u32,
        readonly rightexpression: u32
    ) {}
    bytes(): Uint8Array {return clamp(
        Equation.code, this.leftexpression, this.rightexpression
    )}
}

//> 1ºLEVEL -> COMMENT
export class Comment {
    static code = u8(0x07);
    constructor(
        readonly text: u8[]
    ) {}
    bytes(): Uint8Array {return clamp(
        Comment.code, join(this.text, null8())
    )}
}

//> 1ºLEVEL -> USE
export class Use {
    static code = u8(0x08);
    constructor(
        readonly name: u8[],
        readonly start: u32 | null32
    ) {}
    bytes(): Uint8Array {return clamp(
        Use.code, join(this.name, null8()), this.start
    )}
}


//^
//^ 2ºLEVEL
//^

//> 2ºLEVEL -> EXPRESSION
export class Expression {
    static code = u8(0x09);
    constructor(
        readonly signs: u8[],
        readonly terms: u32[]
    ) {}
    bytes(): Uint8Array {return clamp(
        Expression.code, join(this.signs, null8()), join(this.terms, null32())
    )}
}


//^
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> TERM
export class Term {
    static code = u8(0x0A);
    constructor(
        readonly numerator: u32[],
        readonly denominator: u32[]
    ) {}
    bytes(): Uint8Array {return clamp(
        Term.code, join(this.numerator, null32()), join(this.denominator, null32())
    )}
}


//^
//^ 4ºLEVEL
//^

//> 4ºLEVEL -> FACTOR
export class Factor {
    static code = u8(0x0B);
    constructor(
        readonly value: u32,
        readonly exponent: u32 | null32
    ) {}
    bytes(): Uint8Array {return clamp(
        Factor.code, this.value, this.exponent
    )}
}

//> 4ºLEVEL -> LIMIT
export class Limit {
    static code = u8(0x0C);
    constructor(
        readonly variable: u32,
        readonly approach: u32,
        readonly direction: u8 | null8,
        readonly nest: u32,
        readonly exponent: u32 | null32
    ) {}
    bytes(): Uint8Array {return clamp(
        Limit.code, this.variable, this.approach, this.direction, this.nest, this.exponent
    )}
}


//^
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> INFINITE
export class Infinite {
    static code = u8(0x0D);
    constructor() {}
    bytes(): Uint8Array {return clamp(
        Infinite.code
    )}
}

//> 5ºLEVEL -> VARIABLE
export class Variable {
    static code = u8(0x0E);
    constructor(
        readonly representation: u8[]
    ) {}
    bytes(): Uint8Array {return clamp(
        Variable.code, join(this.representation, null8())
    )}
}

//> 5ºLEVEL -> NEST
export class Nest {
    static code = u8(0x0F);
    constructor(
        readonly expression: u32 | null32
    ) {}
    bytes(): Uint8Array {return clamp(
        Nest.code, this.expression
    )}
}

//> 5ºLEVEL -> TENSOR
export class Tensor {
    static code = u8(0x10);
    constructor(
        readonly values: u32[]
    ) {}
    bytes(): Uint8Array {return clamp(
        Tensor.code, join(this.values, null32())
    )}
}

//> 5ºLEVEL -> WHOLE
export class Whole {
    static code = u8(0x11);
    constructor(
        readonly value: u32 | null32
    ) {}
    bytes() {return clamp(
        Whole.code, this.value
    )}
}

//> 5ºLEVEL -> ABSOLUTE
export class Absolute {
    static code = u8(0x12);
    constructor(
        readonly expression: u32
    ) {}
    bytes() {return clamp(
        Absolute.code, this.expression
    )}
}