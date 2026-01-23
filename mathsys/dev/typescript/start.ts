//^
//^ HEAD
//^

//> HEAD -> DATA
import {NonTerminal} from "./nonterminal";
import {Opcode, Binary, Pointer, Vec, node} from "./ir";


//^
//^ START
//^

//> START -> CLASS
export class Start extends NonTerminal {
    private code = new Opcode(0x01).binary();
    readonly stream: Level1[];
    constructor(items: Level1[]) {
        super();
        this.stream = [...items];
        this.freeze();
        return this;
    }
    latex(types: Map<string, string> = new Map<string, string>()): string {
        let delimiters;
        switch (this.stream.length) {
            case 0: {delimiters = ['', '']; break}
            case 1: {delimiters = [String.raw`\(`, String.raw`\)`]; break}
            default: {delimiters = [String.raw`\[`, String.raw`\]`]}
        }
        let values = this.stream.map(element => element.latex(types)).join(String.raw`\\ `);
        while (values.startsWith(String.raw`\\`)) {values = values.slice(2)}
        return `${delimiters[0]}${values}${delimiters[1]}`;
    }
    ir(binary: Binary[], nodes: Binary[] = []): Pointer {
        const pointers = new Vec(this.stream.map(element => element.ir(binary, nodes)));
        return node(this.code.add(pointers), binary, nodes);
    }
}

//> START -> TYPES
import {Level1} from "./level1";