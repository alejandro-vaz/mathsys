//^
//^ HEAD
//^

//> HEAD -> COMPILER
import {Parser} from "./parser/code.js";
import {LaTeX} from "./latex/code.js";
import {IR} from "./ir/code.js";


//^
//^ PRELUDE
//^

//> PRELUDE -> CLASSES
export const _parser = new Parser();
export const _latex = new LaTeX();
export const _ir = new IR();

//> PRELUDE -> FUNCTIONS
export async function functions(): Promise<Function[]> {return [
    help,
    validate,
    binary,
    tokens,
    latex
]}

//> PRELUDE -> TIME WRAPPER
async function timeWrapper<Type>(fn: () => Promise<Type>, name: string): Promise<Type> {
    const start = Date.now();
    const state = await fn();
    console.log(`[INFO] Compiled to ${name} in ${(Date.now() - start) / 1000}s.`);
    return state;
}

//> PRELUDE -> STATISTICS
export async function statistics(): Promise<[]> {return []}

//> PRELUDE -> CLEAR
export async function clear(): Promise<void> {}


//^
//^ MAIN
//^

//> MAIN -> HELP
export async function help(): Promise<string> {
    return (await functions()).map(fn => "- " + fn.name.replace("_", "-")).join("\n");
}

//> MAIN -> VALIDATE
export async function validate(content: string): Promise<boolean> {try {
    _parser.run(content);
    return true;
} catch {return false}}

//> MAIN -> BINARY
export async function binary(content: string): Promise<Uint8Array> {
    return _ir.run(_parser.run(content));
}

//> MAIN -> TOKENS
export async function tokens(content: string): Promise<number> {
    return _ir.run(_parser.run(content)).length;
}

//> MAIN -> LATEX
export async function latex(content: string): Promise<string> {
    return _latex.run(_parser.run(content));
}

console.log(await latex(`use standard
u*v`))