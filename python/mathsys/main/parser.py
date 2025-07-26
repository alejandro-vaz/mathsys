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

# 1ºLEVEL -> NAMESPACE
class Level1: pass

# 1ºLEVEL -> SHEET
@dataclass
class Sheet(Level1):
    statements: list[Level2]


#
#   2ºLEVEL
#

# 2ºLEVEL -> NAMESPACE
class Level2: pass

# 2ºLEVEL -> DECLARATION
@dataclass
class Declaration(Level2):
    identifier: str
    expression: Expression

# 2ºLEVEL -> NODE
@dataclass
class Node(Level2):
    expression: Expression

# 2ºLEVEL -> EQUATION
@dataclass
class Equation(Level2):
    leftSide: Expression
    rightSide: Expression

# 2ºLEVEL -> COMMENT
@dataclass
class Comment(Level2):
    text: str


#
#   3ºLEVEL
#

# 3ºLEVEL -> NAMESPACE
class Level3: pass

# 3ºLEVEL -> EXPRESSION
@dataclass
class Expression(Level3):
    terms: list[Term]


#
#   4ºLEVEL
#

# 4ºLEVEL -> NAMESPACE
class Level4: pass

# 4ºLEVEL -> TERM
@dataclass
class Term(Level4):
    factors: list[Factor]
    operators: list[str]


#
#   5ºLEVEL
#

# 5ºLEVEL -> NAMESPACE
class Level5: pass

# 5ºLEVEL -> FACTOR
@dataclass
class Factor(Level5):
    type: [
        "unsigned" | "signed", 
        "bare" | "exponentiated",
        "number" | "identifier" | "expression" | "vector"
    ]
    signs: str | None
    value: str | Expression | Vector
    exponent: Expression | None


#
#   6ºLEVEL
#

# 6ºLEVEL -> NAMESPACE
class Level6: pass

# 6ºLEVEL -> VECTOR
@dataclass
class Vector(Level6):
    expressions: list[Expression]


#
#   PARSER
#

# PARSER -> TOKEN TRIMMER
def ñ(token: Token) -> str: return token.value.replace(" ", "")

# PARSER -> LIST ACCESS
def º(array: list, number: int) -> any:
    if number < len(array):
        return array[number]

# PARSER -> CLASS
class Parser(Transformer):
    syntax: str
    # CLASS -> INIT
    def __init__(self, syntax: str) -> None:
        self.syntax = syntax
        super()
    # CLASS -> RUN
    def run(self, content: str) -> Level1:
        return self.transform(Lark(self.syntax, parser="earley", start="level1").parse(content))
    # CLASS -> LEVEL 1
    def level1(self, items: list[Level1]) -> Level1:
        return items[0]
    # CLASS -> LEVEL 2
    def level2(self, items: list[Level2]) -> Level2:
        return items[0]
    # CLASS -> LEVEL 3
    def level3(self, items: list[Level3]) -> Level3:
        return items[0]
    # CLASS -> LEVEL 4
    def level4(self, items: list[Level4]) -> Level4:
        return items[0]
    # CLASS -> LEVEL 5
    def level5(self, items: list[Level5]) -> Level5:
        return items[0]
    # CLASS -> LEVEL 6
    def level6(self, items: list[Level6]) -> Level6:
        return items[0]
    # CLASS -> 1 SHEET CONSTRUCT
    def sheet(self, items: list[Token | Level2]) -> Sheet: 
        return Sheet([item for item in items if isinstance(item, Level2)])
    # CLASS -> 2 DECLARATION CONSTRUCT
    def declaration(self, items: list[Token | Expression]) -> Declaration: 
        return Declaration(ñ(items[0]), items[2])
    # CLASS -> 2 NODE CONSTRUCT
    def node(self, items: list[Expression]) -> Node:
        return Node(items[0])
    # CLASS -> 2 EQUATION CONSTRUCT
    def equation(self, items: list[Token | Expression]) -> Equation:
        return Equation(items[0], items[2])
    # CLASS -> 2 COMMENT CONSTRUCT
    def comment(self, items: list[Token]) -> Comment:
        return Comment(items[0].value[1:].strip())
    # CLASS -> 3 EXPRESSION CONSTRUCT
    def expression(self, items: list[Term]) -> Expression: 
        return Expression(items)
    # CLASS -> 4 TERM CONSTRUCT
    def term(self, items: list[Factor | Token]) -> Term:
        return Term(
            [factor for factor in items if isinstance(factor, Factor)],
            [ñ(operator) for operator in items if isinstance(operator, Token)]
        )
    # CLASS -> 5 FACTOR CONSTRUCT
    def factor(self, items: list[Token | Expression | Vector]) -> Factor:
        type = self._factor(items)
        match type:
            case ["unsigned", "bare", "number"]: return Factor(type, None, ñ(items[0]), None)
            case ["unsigned", "bare", "identifier"]: return Factor(type, None, ñ(items[0]), None)
            case ["unsigned", "bare", "expression"]: return Factor(type, None, items[1], None)
            case ["unsigned", "bare", "vector"]: return Factor(type, None, items[0], None)
            case ["signed", "bare", "number"]: return Factor(type, ñ(items[0]), ñ(items[1]), None)
            case ["signed", "bare", "identifier"]: return Factor(type, ñ(items[0]), ñ(items[1]), None)
            case ["signed", "bare", "expression"]: return Factor(type, ñ(items[0]), items[2], None)
            case ["signed", "bare", "vector"]: return Factor(type, ñ(items[0]), items[1], None)
            case ["unsigned", "exponentiated", "number"]: return Factor(type, None, ñ(items[0]), items[2])
            case ["unsigned", "exponentiated", "identifier"]: return Factor(type, None, ñ(items[0]), items[2])
            case ["unsigned", "exponentiated", "expression"]: return Factor(type, None, items[1], items[4])
            case ["unsigned", "exponentiated", "vector"]: return Factor(type, None, items[0], items[2])
            case ["signed", "exponentiated", "number"]: return Factor(type, ñ(items[0]), ñ(items[1]), items[3])
            case ["signed", "exponentiated", "identifier"]: return Factor(type, ñ(items[0]), ñ(items[1]), items[3])
            case ["signed", "exponentiated", "expression"]: return Factor(type, ñ(items[0]), items[2], items[5])
            case ["signed", "exponentiated", "vector"]: return Factor(type, ñ(items[0]), items[1], items[3])
    # CLASS -> 6 VECTOR CONSTRUCT
    def vector(self, items: list[Token | Expression]) -> Vector:
        return Vector([expression for expression in items if isinstance(expression, Expression)])
    # CLASS -> 4 TERM TYPE
    def _factor(self, items: list[Token | Expression | Vector]) -> list[str]:
        match items[0]:
            case item if getattr(item, "type", None) == "SIGNS": 
                return ["signed", self._factor(items[1:])[1], self._factor(items[1:])[2]]
            case item if getattr(item, "type", None) == "NUMBER": 
                if getattr(º(items, 1), "type", None) == "EXPONENT": 
                    return ["unsigned", "exponentiated", "number"]
                else:
                    return ["unsigned", "bare", "number"]
            case item if getattr(item, "type", None) == "IDENTIFIER": 
                if getattr(º(items, 1), "type", None) == "EXPONENT":
                    return ["unsigned", "exponentiated", "identifier"]
                else:
                    return ["unsigned", "bare", "identifier"]
            case item if getattr(item, "type", None) == "OPEN": 
                if getattr(º(items, 3), "type", None) == "EXPONENT":
                    return ["unsigned", "exponentiated", "expression"]
                else:
                    return ["unsigned", "bare", "expression"]
            case item if isinstance(item, Vector): 
                if getattr(º(items, 1), "type", None) == "EXPONENT":
                    return ["unsigned", "exponentiated", "vector"]
                else:
                    return ["unsigned", "bare", "vector"]