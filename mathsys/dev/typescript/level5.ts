//^
//^ HEAD
//^

//> HEAD -> DATA
import {NonTerminal} from "./nonterminal";
import {IDENTIFIER, NUMBER, RATIONAL, TYPE} from "./tokenizer";
import {Opcode, node, Binary, Pointer, String as Str, Option, Vec, BigUint, Group} from "./ir";
import {VARIABLES, CONVERSION} from "./latex";


//^
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> NAMESPACE
export abstract class Level5 extends NonTerminal {}

//> 5ºLEVEL -> INFINITE
export class Infinite extends Level5 {
    private code = new Opcode(0x0C).binary();
    constructor(items: []) {super(); this.freeze(); return this}
    latex(types: Map<string, string>): string {return String.raw`\infty `}
    ir(binary: Binary[], nodes: Binary[]): Pointer {return node(this.code, binary, nodes)}
}

//> 5ºLEVEL -> VARIABLE
export class Variable extends Level5 {
    private code = new Opcode(0x0D).binary();
    readonly representation: string;
    constructor(items: IDENTIFIER[]) {
        super();
        this.representation = items[0].value;
        this.freeze();
        return this;
    }
    latex(types: Map<string, string>): string {
        let curated = this.representation;
        for (const [source, replace] of VARIABLES) {
            curated = curated.replaceAll(source, replace);
        }
        return (CONVERSION.get(types.get(this.representation) ?? "@Undefined") as (name: string) => string)(curated)
    }
    ir(binary: Binary[], nodes: Binary[]): Pointer {
        const representation = new Str(this.representation);
        return node(this.code.add(representation), binary, nodes);
    }
}

//> 5ºLEVEL -> NEST
export class Nest extends Level5 {
    private code = new Opcode(0x0E).binary();
    readonly value: Level2 | null;
    constructor(items: Level2[]) {
        super();
        this.value = items.length === 1 ? items[0] : null;
        this.freeze();
        return this;
    }
    latex(types: Map<string, string>): string {
        const inside = this.value !== null ? this.value.latex(types) : "";
        return String.raw`\left( ${inside}\right) `;
    }
    ir(binary: Binary[], nodes: Binary[]): Pointer {
        const value = new Option(this.value !== null ? this.value.ir(binary, nodes) : null);
        return node(this.code.add(value), binary, nodes);
    }
}

//> 5ºLEVEL -> TENSOR
export class Tensor extends Level5 {
    private code = new Opcode(0x0F).binary();
    readonly values: readonly Level2[];
    constructor(items: Level2[]) {
        super();
        this.values = items;
        this.freeze();
        return this;
    }
    latex(types: Map<string, string>): string {
        const inside = this.values.length === 0 ? String.raw`\; ` : this.values.map(value => value.latex(types)).join(String.raw`\\ `);
        return String.raw`\begin{bmatrix}${inside}\end{bmatrix}`;
    }
    ir(binary: Binary[], nodes: Binary[]): Pointer {
        const values = new Vec(this.values.map(value => value.ir(binary, nodes)));
        return node(this.code.add(values), binary, nodes);
    }
}

//> 5ºLEVEL -> WHOLE
export class Whole extends Level5 {
    private code = new Opcode(0x10).binary();
    readonly value: bigint;
    constructor(items: NUMBER[]) {
        super();
        this.value = BigInt(items[0].value);
        this.freeze();
        return this;
    }
    latex(types: Map<string, string>): string {return this.value.toString()}
    ir(binary: Binary[], nodes: Binary[]): Pointer {
        const value = new BigUint(this.value);
        return node(this.code.add(value), binary, nodes);
    }
}

//> 5ºLEVEL -> ABSOLUTE
export class Absolute extends Level5 {
    private code = new Opcode(0x11).binary();
    readonly value: Level2;
    constructor(items: Level2[]) {
        super();
        this.value = items[0];
        this.freeze();
        return this;
    }
    latex(types: Map<string, string>): string {return String.raw`\left| ${this.value.latex(types)}\right| `}
    ir(binary: Binary[], nodes: Binary[]): Pointer {
        const value = this.value.ir(binary, nodes);
        return node(this.code.add(value), binary, nodes);
    }
}

//> 5ºLEVEL -> UNDEFINED
export class Undefined extends Level5 {
    private code = new Opcode(0x12).binary();
    constructor(items: []) {
        super();
        this.freeze();
        return this;
    }
    latex(types: Map<string, string>): string {return String.raw`\left. ?\right. `}
    ir(binary: Binary[], nodes: Binary[]): Pointer {return node(this.code, binary, nodes)}
}

//> 5ºLEVEL -> RATIONAL
export class Rational extends Level5 {
    private code = new Opcode(0x13).binary();
    readonly whole: bigint;
    readonly decimal: bigint;
    constructor(items: RATIONAL[]) {
        super();
        this.whole = BigInt(items[0].value.split(".", 1)[0]);
        this.decimal = BigInt(items[0].value.split(".", 1)[1].replace(/0*$/, "").split("").reverse().join(""));
        this.freeze();
        return this;
    }
    latex(types: Map<string, string>): string {return `${this.whole}.${this.decimal.toString().split("").reverse().join("")}`}
    ir(binary: Binary[], nodes: Binary[]): Pointer {
        const whole = new BigUint(this.whole);
        const decimal = new BigUint(this.decimal);
        return node(this.code.add(whole, decimal), binary, nodes);
    }
}

//> 5ºLEVEL -> CASTS
export class Casts extends Level5 {
    private code = new Opcode(0x014).binary();
    readonly element: Level5;
    readonly to: string;
    constructor(items: (Level5 | TYPE)[]) {
        super();
        this.element = items[0] as Level5;
        this.to = (items[1] as TYPE).value;
        this.freeze();
        return this;
    }
    latex(types: Map<string, string>): string {return this.element.latex(types)}
    ir(binary: Binary[], nodes: Binary[]): Pointer {
        const element = this.element.ir(binary, nodes);
        const to = new Group(this.to);
        return node(this.code.add(element, to), binary, nodes);
    }
}

//> 5ºLEVEL -> TYPES
import {Level2} from "./level2";