//^
//^ HEAD
//^

//> HEAD -> COMPILER
import {Parser} from "./parser/code.js";


//^
//^ PRELUDE
//^

//> PRELUDE -> CLASSES
export const _parser = new Parser();

//> PRELUDE -> FUNCTIONS
function functions(): Function[] {return [
    targets,
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
function statistics(): [] {return []}

//> PRELUDE -> CLEAR
function clear(): void {}


//^
//^ MAIN
//^

//> MAIN -> TARGETS
export async function targets(): Promise<string> {
    let list = []
    for (const fn of functions()) {list.push(fn.name.replace("_", "-"))}
    return list.join(", ");
}

//> MAIN -> LATEX
export async function latex(content: string): Promise<any> {return _parser.run(content)}


await latex("use standard");