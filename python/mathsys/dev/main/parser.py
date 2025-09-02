#
#   HEAD
#

# HEAD -> MODULES
from __future__ import annotations
from dataclasses import dataclass
from lark import Lark, Transformer, Token


#
#   START
#

# START -> CLASS
@dataclass
class Start():
    statements: list[Level1]


#
#   1ºLEVEL
#

# 1ºLEVEL -> NAMESPACE
class Level1: pass

# 1ºLEVEL -> DEBUG
@dataclass
class Debug(Level1): pass

# 1ºLEVEL -> DECLARATION
@dataclass
class Declaration(Level1):
    identifier: Variable
    expression: Expression

# 1ºLEVEL -> DEFINITION
@dataclass
class Definition(Level1):
    identifier: Variable
    expression: Expression

# 1ºLEVEL -> NODE
@dataclass
class Node(Level1):
    value: Expression

# 1ºLEVEL -> EQUATION
@dataclass
class Equation(Level1):
    left: Expression
    right: Expression

# 1ºLEVEL -> COMMENT
@dataclass
class Comment(Level1):
    content: str


#
#   2ºLEVEL
#

# 2ºLEVEL -> NAMESPACE
class Level2: pass

# 2ºLEVEL -> EXPRESSION
@dataclass
class Expression(Level2):
    signs: list[str | None]
    terms: list[Level3]


#
#   3ºLEVEL
#

# 3ºLEVEL -> NAMESPACE
class Level3: pass

# 3ºLEVEL -> TERM
@dataclass
class Term(Level3):
    numerator: list[Level4]
    denominator: list[Level4]


#
#   4ºLEVEL
#

# 4ºLEVEL -> NAMESPACE
class Level4: pass

# 4ºLEVEL -> FACTOR
@dataclass
class Factor(Level4):
    value: Level5
    exponent: Expression | None


#
#   5ºLEVEL
#

# 5ºLEVEL -> NAMESPACE
class Level5: pass

# 5ºLEVEL -> INFINITE
@dataclass
class Infinite(Level5): pass

# 5ºLEVEL -> LIMIT
@dataclass 
class Limit(Level5):
    variable: Variable
    approach: Expression
    direction: bool | None
    of: Nest

# 5ºLEVEL -> VARIABLE
@dataclass
class Variable(Level5):
    representation: str

# 5ºLEVEL -> NEST
@dataclass
class Nest(Level5):
    expression: Expression

# 5ºLEVEL -> VECTOR
@dataclass
class Vector(Level5):
    values: list[Expression]

# 5ºLEVEL -> NUMBER
@dataclass
class Number(Level5):
    whole: int
    decimal: int | None


#
#   PARSER
#

# PARSER -> TOKEN TRIMMER
def ñ(token: Token) -> str: return token.value.replace(" ", "")

# PARSER -> LIST ACCESS
def º(array: list, number: int) -> any:
    if number < len(array): return array[number]

# PARSER -> CLASS
class Parser(Transformer):
    # CLASS -> VARIABLES
    parser: Lark
    # CLASS -> INIT
    def __init__(self, syntax: str) -> None: super(); self.parser = Lark(syntax, parser="earley")
    # CLASS -> RUN
    def run(self, content: str) -> Level1: return self.transform(self.parser.parse(content))
    # CLASS -> LEVEL 1
    def level1(self, items: list[Level1]) -> Level1: return items[0]
    # CLASS -> LEVEL 2
    def level2(self, items: list[Level2]) -> Level2: return items[0]
    # CLASS -> LEVEL 3
    def level3(self, items: list[Level3]) -> Level3: return items[0]
    # CLASS -> LEVEL 4
    def level4(self, items: list[Level4]) -> Level4: return items[0]
    # CLASS -> LEVEL 5
    def level5(self, items: list[Level5]) -> Level5: return items[0]
    # CLASS -> START CONSTRUCT
    def start(self, items: list[Level1]) -> Start: 
        return Start(
            items
        )
    # CLASS -> 1 DEBUG CONSTRUCT
    def debug(self, items: list) -> Debug: 
        return Debug()
    # CLASS -> 1 DECLARATION CONSTRUCT
    def declaration(self, items: list[Token, Expression]) -> Declaration: 
        return Declaration(
            *items
        )
    # CLASS -> 1 DEFINITION CONSTRUCT
    def definition(self, items: list[Token, Expression]) -> Definition: 
        return Definition(
            *items
        )
    # CLASS -> 1 NODE CONSTRUCT
    def node(self, items: list[Expression]) -> Node: 
        return Node(
            *items
        )
    # CLASS -> 1 EQUATION CONSTRUCT
    def equation(self, items: list[Expression]) -> Equation: 
        return Equation(
            *items
        )
    # CLASS -> 1 COMMENT CONSTRUCT
    def comment(self, items: list[Token]) -> Comment:
        return Comment(
            ñ(*items)
        )
    # CLASS -> 2 EXPRESSION CONSTRUCT
    def expression(self, items: list[Token | Level3]) -> Expression:
        return Expression(
            [None] + [ñ(item) for item in items if isinstance(item, Token)],
            [item for item in items if isinstance(item, Level3)]
        )
    # CLASS -> 3 TERM CONSTRUCT
    def term(self, items: list[Token | Level4]) -> Term:
        numerator = []
        denominator = []
        location = True
        for item in items:
            if isinstance(item, Token):
                match ñ(item):
                    case "*": location = True
                    case "/": location = False
            else:
                if location: numerator.append(item)
                else: denominator.append(item)
        return Term(
            numerator,
            denominator
        )
    # CLASS -> 4 FACTOR CONSTRUCT
    def factor(self, items: list[Level5 | Expression]) -> Factor:
        return Factor(
            items[0],
            items[1] if len(items) == 2 else None
        )
    # CLASS -> 5 INFINITE CONSTRUCT
    def infinite(self, items: list) -> Infinite:
        return Infinite()
    # CLASS -> 5 LIMIT CONSTRUCT
    def limit(self, items: list[Token | Expression | Nest]) -> Limit:
        return Limit(
            items[0],
            items[1],
            items[2] == "+" if isinstance(items[2], Token) else None,
            items[-1]
        )
    # CLASS -> 5 VARIABLE CONSTRUCT
    def variable(self, items: list[Token]) -> Variable:
        return Variable(
            ñ(*items)
        )
    # CLASS -> 5 NEST CONSTRUCT
    def nest(self, items: list[Expression]) -> Nest:
        return Nest(
            *items
        )
    # CLASS -> 5 VECTOR CONSTRUCT
    def vector(self, items: list[Expression]) -> Vector:
        return Vector(
            items
        )
    # CLASS -> 5 NUMBER CONSTRUCT
    def number(self, items: list[Token]) -> Number:
        return Number(
            int(ñ(items[0])),
            int(ñ(items[-1])) if len(items) == 2 else None
        )