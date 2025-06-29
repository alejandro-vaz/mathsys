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

// MAIN -> FUNCTION
export function main(contents: string, strict: boolean): Program {
    return new Parser(new Tokenizer(contents).run(), strict).parse();
}