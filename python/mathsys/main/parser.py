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
    factors: list[Factor]
    operators: list[str]


#
#   5ºLEVEL
#

# 5ºLEVEL -> FACTOR
@dataclass
class Factor:
    type: [
        "unsigned" | "signed", 
        "number" | "identifier" | "expression", "vector"
    ]
    signs: str
    value: str | Expression | Vector


#
#   6ºLEVEL
#

# 6ºLEVEL -> VECTOR
@dataclass
class Vector:
    expressions: list[Expression]


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
    def term(self, items: list[Factor | Token]) -> Term:
        return Term(
            [factor for factor in items if isinstance(factor, Factor)],
            [ñ(operator) for operator in items if isinstance(operator, Token)]
        )
    # CLASS -> FACTOR CONSTRUCT
    def factor(self, items: list[Token | Expression | Vector]) -> Factor:
        type = self._factor(items)
        match type:
            case ["unsigned", "number"]: return Factor(type, "", ñ(items[0]))
            case ["unsigned", "identifier"]: return Factor(type, "", ñ(items[0]))
            case ["unsigned", "expression"]: return Factor(type, "", items[1])
            case ["unsigned", "vector"]: return Factor(type, "", items[0])
            case ["signed", "number"]: return Factor(type, ñ(items[0]), ñ(items[1]))
            case ["signed", "identifier"]: return Factor(type, ñ(items[0]), ñ(items[1]))
            case ["signed", "expression"]: return Factor(type, ñ(items[0]), items[2])
            case ["signed", "vector"]: return Factor(type, ñ(items[0]), items[1])
    # CLASS -> VECTOR CONSTRUCT
    def vector(self, items: list[Token | Expression]) -> Vector:
        return Vector([expression for expression in items if isinstance(expression, Expression)])
    # CLASS -> SHEET TYPE
    def _sheet(self, items: list[Token | Declaration]) -> list[str]:
        match len([declaration for declaration in items if isinstance(declaration, Declaration)]):
            case 0: return ["empty"]
            case 1: return ["inline"]
            case _: return ["normal"]
    # CLASS -> TERM TYPE
    def _factor(self, items: list[Token | Expression | Vector]) -> list[str]:
        match items[0]:
            case item if getattr(item, "type", None) == "SIGNS": return ["signed", self._factor(items[1:])[1]]
            case item if getattr(item, "type", None) == "NUMBER": return ["unsigned", "number"]
            case item if getattr(item, "type", None) == "IDENTIFIER": return ["unsigned", "identifier"]
            case item if getattr(item, "type", None) == "OPEN": return ["unsigned", "expression"]
            case item if isinstance(item, Vector): return ["unsigned", "vector"]