//^
//^ HEAD
//^

//> HEAD -> MODULES
import nearley from "nearley";
import util from "util";

//> HEAD -> DAT
import grammar from "./syntax.cjs";
import * as parser from "./dataclasses.js";


//^
//^ PARSER
//^

//> PARSER -> CLASS
export class Parser {
    //~ CLASS -> VARIABLES
    parser: nearley.Parser
    //~ CLASS -> CONSTRUCTOR
    constructor() {this.parser = new nearley.Parser(nearley.Grammar.fromCompiled(grammar))}
    //~ CLASS -> RUN
    run(content: string): parser.Start {
        this.parser.feed(content);
        return this.parser.results[0];
    }
}