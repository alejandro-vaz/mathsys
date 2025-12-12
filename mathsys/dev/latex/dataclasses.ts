//^
//^ HEAD
//^

//> HEAD -> DATA
import {SPECIAL, VARIABLES, CONVERSION, types} from "./local.js";


//^
//^ START
//^

//> START -> CLASS
export class Start {
    constructor(
        public statements: string[]
    ) {} 
    toString(): string {
        let delimiters;
        switch (this.statements.length) {
            case 0: {delimiters = ["", ""]; break};
            case 1: {delimiters = ["\\(", "\\)"]; break};
            default: delimiters = ["\\[", "\\]"];
        }
        let values = this.statements.join(String.raw`\\ `);
        while (values.startsWith(String.raw`\\`)) {values = values.slice(2)}
        return `${delimiters[0]}${values}${delimiters[1]}`;
    }
}


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> DECLARATION
export class Declaration {
    constructor(
        public group: string,
        public variable: string,
        public expression: string
    ) {}
    toString(): string {
        return `${this.variable}=${this.expression}`;
    }
}

//> 1ºLEVEL -> DEFINITION
export class Definition {
    constructor(
        public group: string,
        public variable: string,
        public expression: string
    ) {}
    toString(): string {
        return String.raw`${this.variable}\equiv ${this.expression}`;
    }
}

//> 1ºLEVEL -> ANNOTATION
export class Annotation {
    constructor(
        public group: string,
        public variables: string[]
    ) {}
    toString(): string {
        return "";
    }
}

//> 1ºLEVEL -> NODE
export class Node {
    constructor(
        public expression: string
    ) {}
    toString(): string {
        return this.expression;
    }
}

//> 1ºLEVEL -> EQUATION
export class Equation {
    constructor(
        public leftexpression: string,
        public rightexpression: string
    ) {}
    toString(): string {
        return `${this.leftexpression}=${this.rightexpression}`;
    }
}

//> 1ºLEVEL -> COMMENT
export class Comment {
    constructor(
        public text: string
    ) {}
    toString(): string {
        const curated = this.text.split("").map(character => character in SPECIAL ? SPECIAL[character] : character).join("");
        return String.raw`\\\text{${curated}}`;
    }
}

//> 1ºLEVEL -> USE
export class Use {
    constructor(
        public name: string,
        public start: boolean
    ) {}
    toString(): string {
        const delimiters = this.start ? ["", ""] : [String.raw`\color{brown}`, String.raw`\color{black}`];
        return String.raw`${delimiters[0]}\textbf{use ${this.name}}${delimiters[1]}`;
    }
}


//^
//^ 2ºLEVEL
//^

//> 2ºLEVEL -> EXPRESSION
export class Expression {
    constructor(
        public signs: string[],
        public terms: string[]
    ) {}
    toString(): string {
        const string = [...this.terms.keys()].map(index => `${this.signs[index]}${this.terms[index]}`).join("");
        return string;
    }
}


//^
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> TERM
export class Term {
    constructor(
        public numerator: string[],
        public denominator: string[]
    ) {}
    toString(): string {
        const numerator = this.numerator.join("");
        const denominator = this.denominator.join("");
        const assembly = this.denominator.length !== 0 ? String.raw`\frac{${numerator}}{${denominator}}` : numerator;
        return assembly;
    }
}


//^
//^ 4ºLEVEL
//^

//> 4ºLEVEL -> FACTOR
export class Factor {
    constructor(
        public value: string,
        public exponent: string | null
    ) {}
    toString(): string {
        const exponent = this.exponent !== null ? `^{${this.exponent}}` : "";
        return `${this.value}${exponent}`;
    }
}

//> 4ºLEVEL -> LIMIT
export class Limit {
    constructor(
        public variable: string,
        public approach: string,
        public direction: boolean | null,
        public nest: string,
        public exponent: string | null
    ) {}
    toString(): string {
        const direction = this.direction !== null ? `^{${this.direction ? '+' : '-'}}` : "";
        const exponent = this.exponent !== null ? `^{${this.exponent}}` : "";
        return String.raw`\lim_{\substack{${this.variable}\to ${this.approach}${direction}}}${this.nest}${exponent}`;
    }
}


//^
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> INFINITE
export class Infinite {
    constructor() {}
    toString(): string {
        return String.raw`\infty `;
    }
}

//> 5ºLEVEL -> VARIABLE
export class Variable {
    constructor(
        public representation: string
    ) {}
    toString(): string {
        let curated = this.representation;
        for (const [source, replace] of Object.entries(VARIABLES)) {curated = curated.replace(source, replace)};
        const identifier = CONVERSION[this.representation in types ? String(types[this.representation]) : "null"](curated);
        return identifier;
    }
}

//> 5ºLEVEL -> NEST
export class Nest {
    constructor(
        public expression: string
    ) {}
    toString(): string {
        const inside = this.expression ? this.expression : String.raw`\, `;
        return String.raw`\left( ${inside}\right) `;
    }
}

//> 5ºLEVEL -> TENSOR
export class Tensor {
    constructor(
        public values: string[]
    ) {}
    toString(): string {
        const inside = this.values.length === 0 ? String.raw`\; ` : this.values.join(String.raw`\\ `);
        return String.raw`\begin{bmatrix}${inside}\end{bmatrix}`;
    }
}

//> 5ºLEVEL -> NUMBER
export class Number {
    constructor(
        public value: number,
        public shift: number
    ) {}
    toString(): string {
        const each = this.value.toString();
        return `${each.slice(0, each.length - this.shift)}.${each.slice(each.length - this.shift)}`;
    }
}