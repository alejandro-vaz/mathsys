//^
//^ HEAD
//^

//> HEAD -> DATA
import {UnknownToken} from "./issues.js";


//^
//^ TOKENS
//^

//> TOKENS -> PROTOTYPE
export abstract class Token {
    public static readonly pattern: RegExp;
    constructor(
        private column: number,
        private line: number,
        readonly value: string
    ) {}
    important(): boolean {return !this.constructor.name.startsWith("_")}
    compilable(): boolean {return this.constructor.name !== "QUOTE"}
}

//> TOKENS -> IMPORTANT
export class IDENTIFIER extends Token {static pattern = /^[A-Za-zº$%]+/}
export class MODULE extends Token {static pattern = /^\"[a-z]+\"/}
export class NUMBER extends Token {static pattern = /^[0-9]+/}
export class OPERATOR extends Token {static pattern = /^[\*\/]/}
export class QUOTE extends Token {static pattern = /^\#[^\n]*/}
export class RATIONAL extends Token {static pattern = /^[0-9]*\.[0-9]+/}
export class SIGN extends Token {static pattern = /^[+-]/}
export class TYPE extends Token {static pattern = /^\@(Infinite|Integer|Natural|Nexists|Rational|Tensor|Undefined|Variable|Whole)/}

//> TOKENS -> UNIMPORTANT
export class _BINDING extends Token {static pattern = /^==/}
export class _CLOSE extends Token {static pattern = /^\)/}
export class _COMMA extends Token {static pattern = /^,/}
export class _ENTER extends Token {static pattern = /^\[/}
export class _EQUALITY extends Token {static pattern = /^=/}
export class _EXIT extends Token {static pattern = /^\]/}
export class _EXPONENTIATION extends Token {static pattern = /^\^/}
export class _INF extends Token {static pattern = /^inf/}
export class _LIM extends Token {static pattern = /^lim/}
export class _NEWLINES extends Token {static pattern = /^\n+/}
export class _OF extends Token {static pattern = /^of/}
export class _OPEN extends Token {static pattern = /^\(/}
export class _PIPE extends Token {static pattern = /^\|/}
export class _SPACES extends Token {static pattern = /^ +/}
export class _TO extends Token {static pattern = /^->/}
export class _UNDEFINED extends Token {static pattern = /^\?/}
export class _USE extends Token {static pattern = /^use/}

//> TOKENS -> EOF
export class _EOF extends Token {static pattern = /(?!)/}

//> TOKENS ->_ORDER
export const ORDER = [
    _UNDEFINED,
    _LIM,
    _PIPE,
    _TO,
    _OF,
    _INF,
    _USE,
    TYPE,
    IDENTIFIER,
    _EXPONENTIATION,
    RATIONAL,
    NUMBER,
    _BINDING,
    _EQUALITY,
    OPERATOR,
    SIGN,
    _OPEN,
    _CLOSE,
    _ENTER,
    _COMMA,
    _EXIT,
    _SPACES,
    _NEWLINES,
    MODULE,
    QUOTE,
    _EOF
]


//^
//^ TOKENIZER
//^

//> TOKENIZER -> CLASS
export class Tokenizer {
    private content!: string;
    private column!: number;
    private line!: number;
    private left!: string;
    private tokens!: Token[];
    run(content: string): Token[] {
        this.content = content;
        this.column = 1;
        this.left = content;
        this.tokens = [];
        while (this.left.length > 0) {
            const token = this.next();
            this.tokens.push(token);
            if (token instanceof _NEWLINES) {this.line += token.value.length; this.column = 0}
            else {this.column += token.value.length}
            this.left = this.left.slice(token.value.length);
        }
        this.tokens.push(new _EOF(this.column, this.line, ""));
        return this.tokens;
    }
    next(): Token {
        const status = new Map<typeof Token, string>();
        for (const token of ORDER) {
            const match = token.pattern.exec(this.left);
            if (!match) {continue}
            status.set(token, match[0]);
        }
        try {
            const at = Math.max(...[...status.values()].map(each => each.length));
            for (const [token, match] of status.entries()) {if (match.length === at) {return new (token as any)(
                this.column,
                this.line,
                match
            )}}
            throw new Error("Unreachable");
        } catch {throw new UnknownToken(this.line, this.column, this.content.split("\n")[this.line - 1])}
    }
}