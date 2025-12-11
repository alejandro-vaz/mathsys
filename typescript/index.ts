//^
//^ PRELUDE
//^

//> PRELUDE -> FUNCTIONS
function functions(): Function[] {return [
    targets
]}

//> PRELUDE -> TIME WRAPPER
async function timeWrapper<Type>(fn: () => Promise<Type>, name: string): Promise<Type> {
    const start = Date.now();
    const state = await fn();
    console.log(`[INFO] Compiled to ${name} in ${(Date.now() - start) / 1000}s.`);
    return state;
}


//^
//^ MAIN
//^

//> MAIN -> TARGETS
export async function targets(): Promise<string> {
    let list = []
    for (const fn of functions()) {list.push(fn.name.replace("_", "-"))}
    return list.join(", ");
}