#
#   HEAD
#

# HEAD -> MODULES
from __future__ import annotations
from dataclasses import dataclass
from lark import Lark, Transformer, Token


#
#   1ºLEVEL
#

# 1ºLEVEL -> SHEET
@dataclass
class Sheet:
    type: [
        "empty" | "inline" | "normal"
    ]
    statements: list[Declaration]


#
#   2ºLEVEL
#

# 2ºLEVEL -> DECLARATION
@dataclass
class Declaration:
    identifier: str
    expression: Expression


#
#   3ºLEVEL
#

# 3ºLEVEL -> EXPRESSION
@dataclass
class Expression:
    terms: list[Term]


#
#   4ºLEVEL
#

# 4ºLEVEL -> TERM
@dataclass
class Term:
    type: [
        "unsigned" | "signed", 
        "number" | "identifier" | "expression"
    ]
    signs: str
    value: str | Expression


#
#   PARSER
#

# PARSER -> TOKEN TRIMMER
def ñ(token: Token) -> str: return token.value.replace(" ", "")

# PARSER -> CLASS
class Parser(Transformer):
    syntax: str
    # CLASS -> INIT
    def __init__(self, syntax: str) -> None:
        self.syntax = syntax
        super()
    # CLASS -> RUN
    def run(self, content: str) -> Sheet:
        return self.transform(Lark(self.syntax, parser="earley", start="sheet").parse(content))
    # CLASS -> SHEET CONSTRUCT
    def sheet(self, items: list[Token | Declaration]) -> Sheet: 
        type = self._sheet(items)
        return Sheet(type, [item for item in items if isinstance(item, Declaration)])
    # CLASS -> DECLARATION CONSTRUCT
    def declaration(self, items: list[Token | Expression]) -> Declaration: 
        return Declaration(ñ(items[0]), items[2])
    # CLASS -> EXPRESSION CONSTRUCT
    def expression(self, items: list[Term]) -> Expression: 
        return Expression(items)
    # CLASS -> TERM CONSTRUCT
    def term(self, items: list[Token]) -> Term:
        type = self._term(items)
        match type:
            case ["unsigned", "number"]: return Term(type, "", ñ(items[0]))
            case ["unsigned", "identifier"]: return Term(type, "", ñ(items[0]))
            case ["unsigned", "expression"]: return Term(type, "", items[1])
            case ["signed", "number"]: return Term(type, ñ(items[0]), ñ(items[1]))
            case ["signed", "identifier"]: return Term(type, ñ(items[0]), ñ(items[1]))
            case ["signed", "expression"]: return Term(type, ñ(items[0]), items[2])
    # CLASS -> SHEET TYPE
    def _sheet(self, items: list[Token | Declaration]) -> list[str]:
        match len([declaration for declaration in items if isinstance(declaration, Declaration)]):
            case 0: return ["empty"]
            case 1: return ["inline"]
            case _: return ["normal"]
    # CLASS -> TERM TYPE
    def _term(self, items: list[Token]) -> list[str]:
        match items[0].type:
            case "SIGNS": return ["signed", self._term(items[1:])[1]]
            case "NUMBER": return ["unsigned", "number"]
            case "IDENTIFIER": return ["unsigned", "identifier"]
            case "OPEN": return ["unsigned", "expression"]