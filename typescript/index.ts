//
//  HEAD
//

// HEAD -> COMPILER
import { Tokenizer } from "./main/tokenizer.js";
import { Parser } from "./main/parser.js";

// HEAD -> DATACLASSES
import { Program } from "./main/parser.js";


//
//  MAIN
//

// MAIN -> CONTENT
export function compile(content: string, strict: boolean): Program {
    return new Parser(new Tokenizer(content).run(), strict).parse();
}