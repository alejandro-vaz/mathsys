//^
//^ HEAD
//^

//> HEAD -> DATA
import {Binary, Pointer} from "./ir";


//^
//^ NONTERMINAL
//^

//> NONTERMINAL -> START
export abstract class NonTerminal {
    private frozen: boolean = false;
    constructor() {return new Proxy(this, {set: (target, property, value) => {
        if (target.frozen) {throw new Error("Instance is frozen")}
        // @ts-ignore
        target[property] = value;
        return true;
    }})}
    abstract latex(types: Map<string, string>): string;
    abstract ir(binary: Binary[], nodes: Binary[]): Pointer;
    freeze(): void {this.frozen = true}
}