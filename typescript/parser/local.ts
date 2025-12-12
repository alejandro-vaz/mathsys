//^
//^ HEAD
//^

//> HEAD -> LIBRARY


//^
//^ LIBRARY
//^

//> LIBRARY -> MODULES
export const MODULES = {
    standard: ""
}


//^
//^ HELPERS
//^

//> HELPERS -> TOKEN
export type Token = {
    type: string,
    value: string,
    text: string,
    offset: number,
    lineBreaks: number,
    line: number,
    col: number
}

//> HELPERS -> ISTOKEN
export function istoken(item: any): boolean {
    return item 
        && "type" in item
        && "value" in item 
        && "text" in item
        && "offset" in item
        && "lineBreaks" in item
        && "line" in item
        && "col" in item
}

//> HELPERS -> TOKEN TRIMMER
export function ñ(token: Token): string {return token.text.replace(" ", "")}

//> HELPERS -> FILTER
export function del(list: any[]): any[] {
    const stack = [...list];
    const result: any[] = [];
    while (stack.length > 0) {
        const item = stack.shift();
        if (item === null || item === undefined) continue;
        if (Array.isArray(item)) {
            stack.unshift(...item);
        } else if (istoken(item)) {
            if (!item.type.startsWith("_")) result.push(item);
        } else {
            result.push(item);
        }
    }
    return result;
}