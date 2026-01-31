//^
//^ HEAD
//^

//> HEAD -> COMPILER
import {Parser} from "./parser/code.js";
import {LaTeX} from "./latex/code.js";
import {IR} from "./ir/code.js";


//^
//^ MAIN
//^

//> MAIN -> VALIDATE
export async function validate(content: string): Promise<boolean> {try {
    new Parser().run(content);
    return true;
} catch {return false}}

//> MAIN -> BINARY
export async function binary(content: string): Promise<Uint8Array> {
    return new IR().run(await new Parser().run(content));
}

//> MAIN -> TOKENS
export async function tokens(content: string): Promise<number> {
    return (await new IR().run(await new Parser().run(content))).length;
}

//> MAIN -> LATEX
export async function latex(content: string): Promise<string> {
    return new LaTeX().run(await new Parser().run(content));
}


//^
//^ TARGETS
//^

//> TARGETS -> FUNCTIONS
export const FUNCTIONS = [
    validate,
    binary,
    tokens,
    latex
]

//> TARGETS -> FUNCTIONS
export const HELP = FUNCTIONS.map(fn => "- " + fn.name.replace("_", "-")).join("\n");