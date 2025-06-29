//
//  HEAD
//

// HEAD -> DATACLASSES
import { Token } from "./tokenizer.js";


//
//  DATACLASSES
//

// DATACLASSES -> NAMESPACE
class ASTNode {}

// DATACLASSES -> VALUE
class Value extends ASTNode {
    signs: string | null
    datatype: string
    value: string | Expression
    constructor(signs: string | null, datatype: string, value: string | Expression) {
        super();
        this.signs = signs;
        this.datatype = datatype;
        this.value = value;
    }
}

// DATACLASSES -> EXPRESSION
class Expression extends ASTNode {
    values: Value[]
    constructor(values: Value[]) {
        super();
        this.values = values;
    }
}

// DATACLASSES -> DECLARATION
class Declaration extends ASTNode {
    keyword: string | null
    identifier: string
    value: Expression
    constructor(keyword: string | null, identifier: string, value: Expression) {
        super();
        this.keyword = keyword;
        this.identifier = identifier;
        this.value = value;
    }
}

// DATACLASSES -> PROGRAM
export class Program extends ASTNode {
    statements: Declaration[];
    constructor(statements: Declaration[]) {
        super();
        this.statements = statements;
    }
}


//
//  PARSER
//

// PARSER -> CLASS
export class Parser {
    // CLASS -> VARIABES
    tokens: Token[];
    strict: boolean;
    position: number;
    // CLASS -> INIT
    constructor(tokens: Token[], strict: boolean) {
        this.tokens = tokens;
        this.strict = strict;
        this.position = 0;
    }
    // CLASS -> GET CURRENT TOKEN
    thisToken(): Token | null {
        return this.position < this.tokens.length ? this.tokens[this.position] : null;
    }
    // CLASS -> CONSUME TOKEN
    consume(...expectedTypes: string[]): Token {
        const token = this.thisToken();
        if (token === null) {
            throw new Error(`[AST ISSUE] Unexpected end of input, expected ${expectedTypes}`)
        } else if (!expectedTypes.includes(token.datatype)) {
            throw new Error(`[AST ISSUE] Expected token ${expectedTypes} but got ${token.datatype} at line ${token.position[0]}, col ${token.position[1]}`)
        } else {
            this.position++;
            return token;
        }
    }
    // CLASS -> PARSE
    parse(): Program {
        const statements: Declaration[] = []
        while (this.thisToken() !== null) {
            if (this.thisToken().datatype === "NEWLINE") {
                this.consume("NEWLINE");
            } else {
                statements.push(this.declaration());
            }
        }
        return new Program(statements)
    }
    // CLASS -> DECLARATION PARSING
    declaration(): Declaration {
        const data: (string | null | Expression)[] = [];
        if (this.strict || this.thisToken().datatype === "KEYWORD") {
            data.push(this.consume("KEYWORD").value);
        } else {
            data.push(null);
        }
        data.push(this.consume("IDENTIFIER").value);
        this.consume("EQUALITY");
        data.push(this.expression());
        return new Declaration(...data as [string | null, string, Expression]);
    }
    // CLASS -> EXPRESSION PARSING
    expression(): Expression {
        const data: Value[][] = [];
        data.push([]);
        data[0].push(this.value());
        while (["SIGNS", "NUMBER", "IDENTIFIER", "LPAREN"].includes(this.thisToken().datatype)) {
            data[0].push(this.value());
        }
        return new Expression(data[0]);
    }
    // CLASS -> VALUE PARSING
    value(): Value {
        const data: (string | null | Expression)[] = [];
        if (this.thisToken().datatype === "SIGNS") {
            data.push(this.consume("SIGNS").value);
        } else {
            data.push(null);
        }
        const token = this.consume("NUMBER", "IDENTIFIER", "LPAREN");
        data.push(token.datatype);
        switch (token.datatype) {
            case "LPAREN":
                data.push(this.expression());
                this.consume("RPAREN");
            default:
                data.push(token.value);
        }
        return new Value(...data as [string | null, string, string | Expression]);
    }
}