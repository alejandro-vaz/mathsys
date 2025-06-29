//
//  DATACLASSES
//

// DATACLASSES -> TOKEN
export class Token {
    datatype: string;
    value: any;
    position: number[];
    constructor(datatype: string, value: any, position: number[]) {
        this.datatype = datatype;
        this.value = value;
        this.position = position;
    }
}


//
//  TOKENIZER
//

// TOKENIZER -> CLASS
export class Tokenizer {
    // CLASS -> VARIABLES
    code: string;
    tokens: (Token | null)[];
    position: number[];
    lineStart: number;
    index: number;
    regex: RegExp;
    // CLASS -> TOKENS
    spec = {
        KEYWORD: "Num(?:&int|&float)?",
        SPACE: " +",
        IDENTIFIER: "[A-Za-z]+",
        EQUALITY: "=",
        NUMBER: "[0-9]+(\\.[0-9]+)?",
        SIGNS: "[+-](\\s*[+-])*",
        LPAREN: "\\(",
        RPAREN: "\\)",
        NEWLINE: "\\n+",
        MISMATCH: "."
    }
    // CLASS -> NEW ITEM
    constructor(code: string) {
        this.code = code;
        this.tokens = [];
        this.position = [1, 0];
        this.lineStart = 0;
        this.index = 0;
        this.regex = new RegExp(Object.entries(this.spec).map(([datatype, pattern]) => `(?<${datatype}>${pattern})`).join("|"), "y");
    }
    // CLASS -> TOKENIZER
    run(): Token[] {
        while (this.index < this.code.length) {
            this.tokens.push(this.tokenMatch(this.regex.exec(this.code)))
        }
        return this.tokens.filter((element): element is Token => element !== null)
    }
    // CLASS -> TOKEN MATCHER
    tokenMatch(pseudoToken: RegExpExecArray): Token | null {
        const kind = Object.keys(pseudoToken.groups!).find(k => pseudoToken.groups![k] !== undefined)!;
        const value = pseudoToken.groups![kind]!;
        this.index = this.regex.lastIndex;
        switch (kind) {
            case "MISMATCH":
                throw new Error(`[TOKENIZER ISSUE] Unexpected character '${this.code[this.index]}' on line ${this.position[0]}`);
            case "SPACE":
                return null
            case "NEWLINE":
                this.position[0] += 1;
                this.lineStart = this.index;
                return new Token(kind, value.length, [...this.position])
            case "EQUALITY":
            case "LPAREN":
            case "RPAREN":
                return new Token(kind, null, [this.position[0], pseudoToken.index - this.lineStart + 1]);
            default:
                return new Token(kind, value, [this.position[0], pseudoToken.index - this.lineStart + 1]);
        }
    }
}