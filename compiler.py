#
#   HEAD
#

# HEAD -> MODULES
import re
import sys
from dataclasses import dataclass


#
#   TOKENIZER
#

# TOKENIZER -> TOKEN
@dataclass
class Token:
    datatype: str
    value: any
    position: list[int]

# TOKENIZER -> CLASS
class Tokenizer:
    # CLASS -> VARIABLES
    code: str
    tokens: list[Token | None]
    position: list[int]
    lineStart: int
    index: int
    regex: re.Pattern
    # CLASS -> TOKENS
    spec = {
        "KEYWORD": r"[A-Z][a-z]{2}(&[a-z]+)?",
        "SPACE": r" +",
        "IDENTIFIER": r"[A-Za-z]{1,}",
        "EQUALITY": r"=",
        "NUMBER": r"[-+]?[0-9]+(\.[0-9]+)?",
        "NEWLINE": r"\n+"
    }
    # CLASS -> NEW ITEM
    def __init__(self: object, code: str) -> None:
        self.code = code
        self.tokens = []
        self.position = [1, 0]
        self.lineStart = 0
        self.index = 0
        self.regex = re.compile("|".join(f"(?P<{name}>{pattern})" for name, pattern in self.spec.items()))
    # CLASS -> TOKENIZER
    def run(self: object) -> list[Token]:
        while self.index < len(self.code):
            pseudoToken = self.regex.match(self.code, self.index)
            if not pseudoToken:
                sys.exit(f"[TOKENIZER ISSUE] Unexpected character {self.code[self.index]!r} on line {self.position[0]}")
            self.tokens.append(self.tokenMatch(pseudoToken))
        return [token for token in self.tokens if token is not None]
    # CLASS -> TOKEN MATCHER
    def tokenMatch(self: object, pseudoToken: re.Match) -> Token | None:
        kind = pseudoToken.lastgroup
        value = pseudoToken.group(kind)
        self.index = pseudoToken.end()
        match kind:
            case "SPACE":
                return None
            case "NEWLINE":
                self.position[0] += 1
                self.lineStart = pseudoToken.end()
                return Token(kind, len(value), self.position)
            case "EQUALITY":
                return Token(kind, None, [self.position[0], pseudoToken.start() - self.lineStart + 1])
            case _:
                return Token(kind, value, [self.position[0], pseudoToken.start() - self.lineStart + 1])


#
#   AST
#

# AST -> NODE
class ASTNode: pass

# AST -> PROGRAM
@dataclass
class Program(ASTNode):
    statements: list[ASTNode]

# AST -> DECLARATION
@dataclass
class Declaration(ASTNode):
    keyword: str | None
    identifier: str
    value: str

# AST -> PARSER
class Parser:
    # PARSER -> VARIABLES
    tokens: list[Token]
    strict: bool
    position: int
    # PARSER -> INIT
    def __init__(self: object, tokens: list, strict: bool) -> None:
        self.tokens = tokens
        self.strict = strict
        self.position = 0
    # PARSER -> GET CURRENT TOKEN
    def thisToken(self: object) -> Token | None:
        return self.tokens[self.position] if self.position < len(self.tokens) else None
    # PARSER -> CONSUME TOKEN
    def consume(self: object, expectedType: str) -> Token:
        token = self.thisToken()
        if token is None:
            sys.exit(f"[AST ISSUE] Unexpected end of input, expected {expectedType}")
        elif token.datatype != expectedType:
            raise sys.exit(f"[AST ISSUE] Expected token {expectedType} but got {token.datatype} at line {token.position[0]}, col {token.position[1]}")
        else:
            self.position += 1
            return token
    # PARSER -> PARSE
    def parse(self: object) -> Program:
        statements: list[Declaration] = []
        while self.thisToken() is not None:
            statements.append(self.declaration())
        return Program(statements)
    # PARSER -> DECLARATION PARSING
    def declaration(self: object) -> Declaration:
        data: list[str | None] = []
        if self.strict or self.thisToken().datatype == "KEYWORD":
            data.append(self.consume("KEYWORD").value)
        else:
            data.append(None)
        data.append(self.consume("IDENTIFIER").value)
        self.consume("EQUALITY")
        data.append(self.consume("NUMBER").value)
        return Declaration(*data)


#
#   SEMANTIC
#

# SEMANTIC -> ANALYZER
class Analyzer:
    # ANALYZER -> VARIABLES
    strict: bool
    symbols: dict
    # ANALYZER -> INIT
    def __init__(self: object, strict: bool) -> None:
        self.strict = strict
        self.symbols = {}
    # ANALYZER -> VISIT
    def visit(self: object, node: ASTNode) -> None:
        return getattr(self, node.__class__.__name__)(node)
    # ANALYZER -> INJECT PROGRAM
    def Program(self: object, program: Program) -> dict:
        for statement in program.statements:
            self.visit(statement)
        return self.symbols
    # ANALYZER -> INJECT DECLARATION
    def Declaration(self: object, declaration: Declaration) -> None:
        if declaration.keyword not in [None, "Num", "Num&int", "Num&float"]: 
            sys.exit(f"[SEMATIC ISSUE] Unknown keyword '{declaration.keyword}' for '{declaration.identifier}'")
        self.symbols[declaration.identifier] = {
            "keyword": declaration.keyword,
            "value": declaration.value
        }


#
#   MAIN
#

# MAIN -> FUNCTION
def main() -> None:
    if len(sys.argv) != 2:
        sys.exit("[ENTRY ISSUE] Usage: python compiler.py <filename>")
    strict = sys.argv[1].endswith(".calc")
    with open(sys.argv[1], "r") as file:
        print(Analyzer(strict).Program(Parser(Tokenizer(file.read()).run(), strict).parse()))

# MAIN -> ENTRY POINT
if __name__ == "__main__":
    main()