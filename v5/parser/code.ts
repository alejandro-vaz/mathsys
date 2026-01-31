//^
//^ HEAD
//^

//> HEAD -> MODULES
import nearley from "nearley";
import fs from "fs/promises";
import {fileURLToPath} from "url";
import {dirname, join} from "path";

//> HEAD -> DATA
import * as parser from "./dataclasses.js";
import syntax from "./syntax.cjs";


//^
//^ LIBRARY
//^

//> LIBRARY -> READ
async function read(name: string): Promise<string> {
    const filePath = join(dirname(fileURLToPath(import.meta.url)), "../common", `${name}.msX`);
    return await fs.readFile(filePath, "utf-8");
}

//> LIBRARY -> MODULES
const MODULES: Record<string, string> = {
    standard: await read("standard")
}


//^
//^ PARSER
//^

//> PARSER -> CLASS
export class Parser {
    //~ CLASS -> VARIABLES
    parser: nearley.Parser
    //~ CLASS -> CONSTRUCTOR
    constructor() {this.parser = new nearley.Parser(nearley.Grammar.fromCompiled(syntax))}
    //~ CLASS -> RUN
    run(content: string): parser.Start {
        this.parser.feed(content);
        let result = this.parser.results[0] as parser.Start;
        for (const level1 of result.statements) {if (level1 instanceof parser.Use) {
            level1.start = MODULES[level1.name] !== undefined ? (new Parser()).run(MODULES[level1.name]) : null
        }}
        return result;
    }
}