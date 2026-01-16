//^
//^ DEFAULTS
//^

//> DEFAULTS -> CODE
function Code(code: string): string {return code}

//> DEFAULTS -> PLACE
function Place(place: string): string {return place}

//> DEFAULTS -> ITEM
function Item(item: string): string {return item}


//^
//^ ISSUES
//^

//> ISSUES -> ISSUE
export class Issue extends Error {
    private content: string;
    constructor(message: string) {
        const formatted = Issue.prototype.format.call(
            {constructor: {name: "Issue"}},
            message
        );
        super(formatted);
        this.content = formatted;
        this.name = this.constructor.name;
    }
    consume(): never {
        console.error(this.content);
        process.exit(1);
    }
    format(base: string): string {
        return `Raised ${Item(this.constructor.name)} issue:\n>\n> ${base.replaceAll("\n", "\n> ")}\n>\n`;
    }
}

//> ISSUES -> UNKNOWN TOKEN
export class UnknownToken extends Issue {constructor(line: number, column: number, code: string) {
    const pointer = " ".repeat(column - 1) + Place("^");
    super(`Unknown token at line ${line}:\n\n    ${Code(code)}\n    ${pointer}`);
}}

//> ISSUES -> UNKNOWN TARGET
export class UnknownTarget extends Issue {constructor(unknown: string, available: Function[]) {
    const helpers = available.map(each => "- " + each.name.replaceAll("_", "-")).join("\n");
    super(`Unknown target ${Item(unknown)}.\n\nAvailable targets:\n${helpers}`);
}}

//> ISSUES -> NO FILE PROVIDED
export class NoFileProvided extends Issue {constructor() {super("No input file provided.")}}

//> ISSUES -> NO TARGET PROVIDED
export class NoTargetProvided extends Issue {constructor(available: Function[]) {
    const helpers = available.map(each => "- " + each.name.replaceAll("_", "-")).join("\n");
    super(`Available targets:\n${helpers}`);
}}

//> ISSUES -> BROKEN SYNTAX
export class BrokenSyntax extends Issue {constructor() {super("Syntax error.")}}

//> ISSUES -> FAILED COMPILATION
export class FailedCompilation extends Issue {constructor() {super("Compilation failed.")}}