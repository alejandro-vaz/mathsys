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
export function istoken(item: any): item is Token {
    return item 
        && typeof item === 'object'
        && "type" in item
        && "value" in item 
        && "text" in item
        && "offset" in item
        && "lineBreaks" in item
        && "line" in item
        && "col" in item
}

//> HELPERS -> TOKEN TRIMMER
export function ñ(token: Token): string {return token.text.replaceAll(" ", "")}