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

# 2ºLEVEL -> DEFINITION
@dataclass
class Definition(Level2):
    identifier: str
    expression: Expression

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
    left: Expression
    right: Expression

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
    factors: list[Level5]
    operators: list[str]


#
#   5ºLEVEL
#

# 5ºLEVEL -> NAMESPACE
class Level5: pass

# 5ºLEVEL -> INFINITE
@dataclass
class Infinite(Level5):
    signs: str | None
    exponent: Expression | None

# 5ºLEVEL -> LIMIT
@dataclass
class Limit(Level5):
    signs: str | None
    variable: str
    to: Expression
    at: Expression

# 5ºLEVEL -> VARIABLE
@dataclass
class Variable(Level5):
    signs: str | None
    representation: str
    exponent: Expression | None

# 5ºLEVEL -> NEST
@dataclass
class Nest(Level5):
    signs: str | None
    expression: Expression
    exponent: Expression | None

# 5ºLEVEL -> VECTOR
@dataclass
class Vector(Level5):
    signs: str | None
    values: list[Expression]
    exponent: Expression | None

# 5ºLEVEL -> NUMBER
@dataclass
class Number(Level5):
    signs: str | None
    representation: str
    exponent: Expression | None


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
    # CLASS -> VARIABLES
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
    # CLASS -> 1 SHEET CONSTRUCT
    def sheet(self, items: list[Token | Level2]) -> Sheet: 
        return Sheet([item for item in items if isinstance(item, Level2)])
    # CLASS -> 2 DEFINITION CONSTRUCT
    def definition(self, items: list[Token | Expression]) -> Definition:
        return Definition(ñ(items[0]), items[2])
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
    def term(self, items: list[Level5 | Token]) -> Term:
        return Term(
            [factor for factor in items if isinstance(factor, Level5)],
            [ñ(operator) for operator in items if isinstance(operator, Token)]
        )
    # CLASS -> 5 INFINITE CONSTRUCT
    def infinite(self, items: list[Token | Expression]) -> Infinite:
        return Infinite(
            ñ(items[0]) if items[0].type == "SIGNS" else None,
            items[-2] if items[-1].type == "EXPONENTIATION" else None
        )
    # CLASS -> 5 LIMIT CONSTRUCT
    def limit(self, items: list[Token | Expression]) -> Limit:
        return Limit(
            ñ(items[0]) if items[0].type == "SIGNS" else None,
            ñ(items[2]) if items[0].type == "SIGNS" else ñ(items[1]),
            items[4] if items[0].type == "SIGNS" else items[3],
            items[7] if items[0].type == "SIGNS" else items[6]
        )
    # CLASS -> 5 VARIABLE CONSTRUCT
    def variable(self, items: list[Token | Expression]) -> Variable:
        return Variable(
            ñ(items[0]) if items[0].type == "SIGNS" else None,
            ñ(items[1]) if items[0].type == "SIGNS" else ñ(items[0]),
            items[-2] if items[-1].type == "EXPONENTIATION" else None
        )
    # CLASS -> 5 NEST CONSTRUCT
    def nest(self, items: list[Token | Expression]) -> Nest:
        return Nest(
            ñ(items[0]) if items[0].type == "SIGNS" else None,
            items[2] if items[0].type == "SIGNS" else items[1],
            items[-2] if items[-1].type == "EXPONENTIATION" else None
        )
    # CLASS -> 5 VECTOR CONSTRUCT
    def vector(self, items: list[Token | Expression]) -> Vector:
        return Vector(
            ñ(items[0]) if items[0].type == "SIGNS" else None,
            [
                expression for expression in items[:-2] if isinstance(expression, Expression)
            ] if items[-1].type == "EXPONENTIATION" else [
                expression for expression in items if isinstance(expression, Expression)
            ],
            items[-2] if items[-1].type == "EXPONENTIATION" else None
        )
    # CLASS -> 5 NUMBER CONSTRUCT
    def number(self, items: list[Token | Expression]) -> Number:
        return Number(
            ñ(items[0]) if items[0].type == "SIGNS" else None,
            ñ(items[1]) if items[0].type == "SIGNS" else ñ(items[0]),
            items[-2] if items[-1].type == "EXPONENTIATION" else None
        )