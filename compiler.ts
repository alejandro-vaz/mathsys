//
//  TOKENIZER
//

// TOKENIZER -> TOKEN
class Token {
    datatype: string;
    value: any;
    position: number[];
    constructor(datatype: string, value: any, position: number[]) {
        this.datatype = datatype;
        this.value = value;
        this.position = position;
    }
}

// TOKENIZER -> CLASS
class Tokenizer {
    // CLASS -> VARIABLES
    code: string;
    tokens: (Token | null)[];
    position: number[];
    lineStart: number;
    index: number;
    regex: RegExp;
    // CLASS -> TOKENS
    spec = {
        KEYWORD: "[A-Z][a-z]{2}(&[a-z]+)?",
        SPACE: " +",
        IDENTIFIER: "[A-Za-z]{1,}",
        EQUALITY: "=",
        NUMBER: "[-+]?[0-9]+(\\.[0-9]+)?",
        NEWLINE:"\\n+"
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
            this.regex.lastIndex = this.index;
            const pseudoToken = this.regex.exec(this.code);
            if (!pseudoToken || !pseudoToken.groups) {
                throw new Error(`[TOKENIZER ISSUE] Unexpected character '${this.code[this.index]}' on line ${this.position[0]}`);
            }
            const token = this.tokenMatch(pseudoToken);
            if (token !== null) {
                this.tokens.push(token)
            }
        }
        return this.tokens.filter((element): element is Token => element !== null)
    }
    // CLASS -> TOKEN MATCHER
    tokenMatch(pseudoToken: RegExpExecArray): Token | null {
        const groups = pseudoToken.groups!;
        const kind = Object.keys(groups).find(k => groups[k] !== undefined)!;
        const value = groups[kind]!;
        const startIndex = pseudoToken.index;
        this.index = this.regex.lastIndex;
        switch (kind) {
            case "SPACE":
                return null
            case "NEWLINE":
                this.position[0] += 1;
                this.lineStart = this.index;
                return new Token(kind, value.length, [...this.position])
            case "EQUALITY":
                return new Token(kind, null, [this.position[0], startIndex - this.lineStart + 1]);
            default:
                return new Token(kind, value, [this.position[0], startIndex - this.lineStart + 1]);
        }
    }
}


//
//  AST
//

// AST -> NODE
class ASTNode {}

// AST -> PROGRAM
class Program extends ASTNode {
    statements: ASTNode[];
    constructor(statements: ASTNode[]) {
        super();
        this.statements = statements;
    }
}

// AST -> DECLARATION
class Declaration extends ASTNode {
    keyword: string | null
    identifier: string
    value: string
    constructor(keyword: string | null, identifier: string, value: string) {
        super();
        this.keyword = keyword;
        this.identifier = identifier;
        this.value = value;
    }
}

// AST -> PARSER
class Parser {
    // PARSER -> VARIABES
    tokens: Token[];
    strict: boolean;
    position: number;
    // PARSER -> INIT
    constructor(tokens: Token[], strict: boolean) {
        this.tokens = tokens;
        this.strict = strict;
        this.position = 0;
    }
    // PARSER -> GET CURRENT TOKEN
    thisToken(): Token | null {
        return this.position < this.tokens.length ? this.tokens[this.position] : null;
    }
    // PARSER -> CONSUME TOKEN
    consume(expectedType: string): Token {
        const token = this.thisToken();
        if (token === null) {
            throw new Error(`[AST ISSUE] Unexpected end of input, expected ${expectedType}`)
        } else if (token.datatype !== expectedType) {
            throw new Error(`[AST ISSUE] Expected token ${expectedType} but got ${token.datatype} at line ${token.position[0]}, col ${token.position[1]}`)
        } else {
            this.position++;
            return token;
        }
    }
    // PARSER -> PARSE
    parse(): Program {
        const statements: Declaration[] = []
        while (this.thisToken() !== null) {
            statements.push(this.declaration());
        }
        return new Program(statements)
    }
    // PARSER -> DECLARATION PARSING
    declaration(): Declaration {
        const data: (string | null)[] = [];
        if (this.strict || this.thisToken()?.datatype === "KEYWORD") {
            data.push(this.consume("KEYWORD").value);
        } else {
            data.push(null);
        }
        data.push(this.consume("IDENTIFIER").value);
        this.consume("EQUALITY");
        data.push(this.consume("NUMBER").value);
        return new Declaration(data[0], data[1]!, data[2]!)
    }
}


//
//  SEMANTIC
//

// SEMANTIC -> ANALYZER
class Analyzer {
    // ANALYZER -> VARIABLES
    strict: boolean;
    symbols: object;
    // ANALYZER -> INIT
    constructor(strict: boolean) {
        this.strict = strict;
        this.symbols = {};
    }
    // ANALYZER -> VISIT
    visit(node: ASTNode): null {
        return (this as any)[node.constructor.name].call(this, node)
    }
    // ANALYZER -> INJECT PROGRAM
    Program(program: Program): object {
        for (const statement of program.statements) {
            this.visit(statement);
        }
        return this.symbols;
    }
    // ANALYZER -> INJECT DECLARATION
    Declaration(declaration: Declaration): void {
        if (![null, "Num", "Num&int", "Num&float"].includes(declaration.keyword)) {
            throw new Error(`[SEMANTIC ISSUE] Unknown keyword '${declaration.keyword}' for '${declaration.identifier}'`);
        };
        this.symbols[declaration.identifier] = {
            keyword: declaration.keyword,
            value: declaration.value
        }
    }
}


//
//  MAIN
//

// MAIN -> FUNCTION
export function main(contents: string, strict: boolean): object {
    return new Analyzer(strict).Program(new Parser(new Tokenizer(contents).run(), strict).parse());
}