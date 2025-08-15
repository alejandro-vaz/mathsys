//
//  HEAD
//

// HEAD -> CURL
async function curl(script: string, data: object): Promise<object | boolean | string | number | null> {
    return (await fetch(
        ("https://") +
        ("abscissa.eu") +
        ("/api/") +
        (script.replace(/^\/+/, '')),
        {
            cache: "no-store",
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        }
    )).json();
}

// HEAD -> MATHSYS EXIT
class MathsysExit extends Error {
    constructor(public exitCode: number) {
        super(`Mathsys halted with code ${exitCode}`);
        this.name = "MathsysExit";
    }
}


//
//  API
//

// API -> VALIDATE
export async function validate(code: string): Promise<boolean> {
    return await curl("mathsys/validate", {Mcode: code}) as boolean;
}

// API -> VIEW
export async function view(code: string): Promise<string> {
    return await curl("mathsys/view", {Mcode: code}) as string;
}

// API -> COMPILE
export async function compile(code: string): Promise<WebAssembly.Instance> {
    return await WebAssembly.instantiate(
        await WebAssembly.compile(await curl("mathsys/compile", {Mcode: code}) as ArrayBuffer),
        {
            sys: {
                call1: (pointer, length) => {

                },
                call60: (exitCode) => {
                    throw new MathsysExit(exitCode);
                }
            }
        }
    );
}