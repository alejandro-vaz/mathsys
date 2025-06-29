#
#   HEAD
#

# HEAD -> MODULES
from __future__ import annotations
from dataclasses import dataclass
import sys

# HEAD -> DATACLASSES
from .tokenizer import Token


#
#   DATACLASSES
#

# DATACLASSES -> NAMESPACE
class ASTNode: pass

# DATACLASSES -> VALUE
@dataclass
class Value(ASTNode):
    signs: str | None
    datatype: str
    value: str | Expression

# DATACLASSES -> EXPRESSION
@dataclass
class Expression(ASTNode):
    values: list[Value]

# DATACLASSES -> DECLARATION
@dataclass
class Declaration(ASTNode):
    keyword: str | None
    identifier: str
    value: Expression

# DATACLASSES -> PROGRAM
@dataclass
class Program(ASTNode):
    statements: list[Declaration]


#
#   PARSER
#

# PARSER -> CLASS
class Parser:
    # CLASS -> VARIABLES
    tokens: list[Token]
    strict: bool
    position: int
    # CLASS -> INIT
    def __init__(self: object, tokens: list, strict: bool) -> None:
        self.tokens = tokens
        self.strict = strict
        self.position = 0
    # CLASS -> GET CURRENT TOKEN
    def thisToken(self: object) -> Token | None:
        return self.tokens[self.position] if self.position < len(self.tokens) else None
    # CLASS -> CONSUME TOKEN
    def consume(self: object, *expectedTypes: str) -> Token:
        token = self.thisToken()
        if token is None:
            sys.exit(f"[AST ISSUE] Unexpected end of input, expected {expectedTypes}")
        elif token.datatype not in expectedTypes:
            raise sys.exit(f"[AST ISSUE] Expected token {expectedTypes} but got {token.datatype} at line {token.position[0]}, col {token.position[1]}")
        else: 
            self.position += 1
            return token
    # CLASS -> PARSE
    def parse(self: object) -> Program:
        statements: list[Declaration] = []
        while self.thisToken() is not None:
            if self.thisToken().datatype == "NEWLINE":
                self.consume("NEWLINE")
            else:
                statements.append(self.declaration())
        return Program(statements)
    # CLASS -> DECLARATION PARSING
    def declaration(self: object) -> Declaration:
        data: list[str | Expression | None] = []
        if self.strict or self.thisToken().datatype == "KEYWORD":
            data.append(self.consume("KEYWORD").value)
        else:
            data.append(None)
        data.append(self.consume("IDENTIFIER").value)
        self.consume("EQUALITY")
        data.append(self.expression())
        return Declaration(*data)
    # CLASS -> EXPRESSION PARSING
    def expression(self: object) -> Expression:
        data: list[list[Value]] = []
        data.append([])
        data[0].append(self.value())
        while self.thisToken().datatype in ["SIGNS", "NUMBER", "IDENTIFIER", "LPAREN"]:
            data[0].append(self.value())
        return Expression(data[0])
    # CLASS -> VALUE PARSING
    def value(self: object) -> Value:
        data: list[str | None | Expression] = []
        if self.thisToken().datatype == "SIGNS":
            data.append(self.consume("SIGNS").value)
        else:
            data.append(None)
        token = self.consume("NUMBER", "IDENTIFIER", "LPAREN")
        data.append(token.datatype)
        match token.datatype:
            case "LPAREN":
                data.append(self.expression())
                self.consume("RPAREN")
            case _: 
                data.append(token.value)
        return Value(*data)