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
        readonly statements: string[]
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
        readonly group: string,
        readonly variable: string,
        readonly expression: string
    ) {}
    toString(): string {
        return `${this.variable}=${this.expression}`;
    }
}

//> 1ºLEVEL -> DEFINITION
export class Definition {
    constructor(
        readonly group: string,
        readonly variable: string,
        readonly expression: string
    ) {}
    toString(): string {
        return String.raw`${this.variable}\equiv ${this.expression}`;
    }
}

//> 1ºLEVEL -> ANNOTATION
export class Annotation {
    constructor(
        readonly group: string,
        readonly variables: string[]
    ) {}
    toString(): string {
        const variables = this.variables.join(",");
        switch (this.group) {
            case "@Integer": {return String.raw`${variables}\in\mathbb{Z}`}
            case "@Natural": {return String.raw`${variables}\in\mathbb{N}`}
            case "@Rational": {return String.raw`${variables}\in\mathbb{Q}`}
            case "@Whole": {return String.raw`${variables}\in\mathbb{W}`}
            default: {return ""}
        }
    }
}

//> 1ºLEVEL -> NODE
export class Node {
    constructor(
        readonly expression: string
    ) {}
    toString(): string {
        return this.expression;
    }
}

//> 1ºLEVEL -> EQUATION
export class Equation {
    constructor(
        readonly leftexpression: string,
        readonly rightexpression: string
    ) {}
    toString(): string {
        return `${this.leftexpression}=${this.rightexpression}`;
    }
}

//> 1ºLEVEL -> COMMENT
export class Comment {
    constructor(
        readonly text: string
    ) {}
    toString(): string {
        const curated = this.text.split("").map(character => character in SPECIAL ? SPECIAL[character] : character).join("");
        return String.raw`\\\text{${curated}}`;
    }
}

//> 1ºLEVEL -> USE
export class Use {
    constructor(
        readonly name: string,
        readonly start: boolean
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
        readonly signs: (boolean | null)[],
        readonly terms: string[]
    ) {}
    toString(): string {
        let string: string[] = [];
        this.terms.forEach((value: string, index: number) => {
            const sign = this.signs[index] !== null ? (this.signs[index] ? "+" : "-") : "";
            string.push(`${sign}${this.terms[index]}`);
        })
        return string.join("");
    }
}


//^
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> TERM
export class Term {
    constructor(
        readonly numerator: string[],
        readonly denominator: string[]
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
        readonly value: string,
        readonly exponent: string | null
    ) {}
    toString(): string {
        const exponent = this.exponent !== null ? `^{${this.exponent}}` : "";
        return `${this.value}${exponent}`;
    }
}

//> 4ºLEVEL -> LIMIT
export class Limit {
    constructor(
        readonly variable: string,
        readonly approach: string,
        readonly direction: boolean | null,
        readonly nest: string,
        readonly exponent: string | null
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
        readonly representation: string
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
        readonly expression: string
    ) {}
    toString(): string {
        const inside = this.expression ? this.expression : String.raw`\, `;
        return String.raw`\left( ${inside}\right) `;
    }
}

//> 5ºLEVEL -> TENSOR
export class Tensor {
    constructor(
        readonly values: string[]
    ) {}
    toString(): string {
        const inside = this.values.length === 0 ? String.raw`\; ` : this.values.join(String.raw`\\ `);
        return String.raw`\begin{bmatrix}${inside}\end{bmatrix}`;
    }
}

//> 5ºLEVEL -> WHOLE
export class Whole {
    constructor(
        readonly value: bigint
    ) {}
    toString(): string {
        return `${this.value}`;
    }
}

//> 5ºLEVEL -> ABSOLUTE
export class Absolute {
    constructor(
        readonly expression: string
    ) {}
    toString(): string {
        return String.raw`\left| ${this.expression}\right| `
    }
}