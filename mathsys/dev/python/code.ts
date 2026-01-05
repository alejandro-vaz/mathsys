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
async function read(name: string): Promise<string> {return await fs.readFile(join(
    dirname(fileURLToPath(import.meta.url)), 
    "../common", 
    `${name}.msX`
), "utf-8")}


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
    async run(content: string): Promise<parser.Start> {return new parser.Start(
        await Promise.all((this.parser.feed(content).results.pop() as parser.Start).stream.map(async item => item instanceof parser.Use ? new parser.Use(item.name, await this.run(await read(item.name))) : item))
    )}
}