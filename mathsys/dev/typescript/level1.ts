//^
//^ HEAD
//^

//> HEAD -> DATA
import {TYPE, MODULE} from "./tokenizer";
import {NonTerminal} from "./nonterminal";
import {Opcode, Option, Group, Pointer, Binary, node, String, Vec} from "./ir";


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> NAMESPACE
export abstract class Level1 extends NonTerminal {}

//> 1ºLEVEL -> DECLARATION
export class Declaration extends Level1 {
    private code = new Opcode(0x02).binary();
    readonly group: string | null;
    readonly variable: Variable;
    readonly value: Level2;
    constructor(items: (TYPE | Variable | Level2)[]) {
        super();
        this.group = items.length === 3 ? (items[0] as TYPE).value : null;
        this.variable = (items.length === 3 ? items[1] : items[0]) as Variable;
        this.value = (items.length === 3 ? items[2] : items[1]) as Level2;
        this.freeze();
        return this;
    }
}

//> 1ºLEVEL -> TYPES
import {Start} from "./start";
import {Level2} from "./level2";
import {Variable} from "./level5";