#^
#^  HEAD
#^

#> HEAD -> MODULES
from __future__ import annotations
from dataclasses import dataclass
from lark import Lark, Transformer, Token


#^
#^  START
#^

#> START -> CLASS
@dataclass(frozen = True)
class Start:
    statements: list[Level1]


#^
#^  1ºLEVEL
#^

#> 1ºLEVEL -> NAMESPACE
class Level1: pass

#> 1ºLEVEL -> DECLARATION
@dataclass(frozen = True)
class Declaration(Level1):
    identifier: Variable
    expression: Expression

#> 1ºLEVEL -> DEFINITION
@dataclass(frozen = True)
class Definition(Level1):
    identifier: Variable
    expression: Expression

#> 1ºLEVEL -> NODE
@dataclass(frozen = True)
class Node(Level1):
    value: Expression

#> 1ºLEVEL -> EQUATION
@dataclass(frozen = True)
class Equation(Level1):
    left: Expression
    right: Expression

#> 1ºLEVEL -> COMMENT
@dataclass(frozen = True)
class Comment(Level1):
    content: str


#^
#^  2ºLEVEL
#^

#> 2ºLEVEL -> NAMESPACE
class Level2: pass

#> 2ºLEVEL -> EXPRESSION
@dataclass(frozen = True)
class Expression(Level2):
    signs: list[str | None]
    terms: list[Level3]


#^
#^  3ºLEVEL
#^

#> 3ºLEVEL -> NAMESPACE
class Level3: pass

#> 3ºLEVEL -> TERM
@dataclass(frozen = True)
class Term(Level3):
    numerator: list[Level4]
    denominator: list[Level4]


#^
#^  4ºLEVEL
#^

#> 4ºLEVEL -> NAMESPACE
class Level4: pass

#> 4ºLEVEL -> FACTOR
@dataclass(frozen = True)
class Factor(Level4):
    value: Level5
    exponent: Expression | None

#> 4ºLEVEL -> LIMIT
@dataclass(frozen = True)
class Limit(Level4):
    variable: Variable
    approach: Expression
    direction: bool | None
    of: Nest
    exponent: Expression | None


#^
#^  5ºLEVEL
#^

#> 5ºLEVEL -> NAMESPACE
class Level5: pass

#> 5ºLEVEL -> INFINITE
@dataclass(frozen = True)
class Infinite(Level5): pass

#> 5ºLEVEL -> VARIABLE
@dataclass(frozen = True)
class Variable(Level5):
    representation: str

#> 5ºLEVEL -> NEST
@dataclass(frozen = True)
class Nest(Level5):
    expression: Expression | None

#> 5ºLEVEL -> VECTOR
@dataclass(frozen = True)
class Vector(Level5):
    values: list[Expression]

#> 5ºLEVEL -> NUMBER
@dataclass(frozen = True)
class Number(Level5):
    whole: str
    decimal: str | None


#^
#^  PARSER
#^

#> PARSER -> TOKEN TRIMMER
def ñ(token: Token) -> str: return token.value.replace(" ", "")

#> PARSER -> CLASS
class Parser(Transformer):
    #~ CLASS -> VARIABLES
    parser: Lark
    #~ CLASS -> INIT
    def __init__(self, syntax: str) -> None: super(); self.parser = Lark(syntax, parser="earley")
    #~ CLASS -> RUN
    def run(self, content: str) -> Start: 
        return self.transform(self.parser.parse(content))
    #~ CLASS -> LEVEL 1
    def level1(self, items: list[Level1]) -> Level1: return items[0]
    #~ CLASS -> LEVEL 2
    def level2(self, items: list[Level2]) -> Level2: return items[0]
    #~ CLASS -> LEVEL 3
    def level3(self, items: list[Level3]) -> Level3: return items[0]
    #~ CLASS -> LEVEL 4
    def level4(self, items: list[Level4]) -> Level4: return items[0]
    #~ CLASS -> LEVEL 5
    def level5(self, items: list[Level5]) -> Level5: return items[0]
    #~ CLASS -> START CONSTRUCT
    def start(self, items: list[Level1]) -> Start: 
        return Start(
            statements = items
        )
    #~ CLASS -> 1 DECLARATION CONSTRUCT
    def declaration(self, items: list[Variable | Expression]) -> Declaration: 
        return Declaration(
            identifier = items[0],
            expression = items[1]
        )
    #~ CLASS -> 1 DEFINITION CONSTRUCT
    def definition(self, items: list[Variable | Expression]) -> Definition: 
        return Definition(
            identifier = items[0],
            expression = items[1]
        )
    #~ CLASS -> 1 NODE CONSTRUCT
    def node(self, items: list[Expression]) -> Node: 
        return Node(
            value = items[0]
        )
    #~ CLASS -> 1 EQUATION CONSTRUCT
    def equation(self, items: list[Expression]) -> Equation: 
        return Equation(
            left = items[0],
            right = items[1]
        )
    #~ CLASS -> 1 COMMENT CONSTRUCT
    def comment(self, items: list[Token]) -> Comment:
        return Comment(
            content = items[0].value if items else ""
        )
    #~ CLASS -> 2 EXPRESSION CONSTRUCT
    def expression(self, items: list[Token | Level3]) -> Expression:
        return Expression(
            signs = ([] if isinstance(items[0], Token) else [None]) + [ñ(item) for item in items if isinstance(item, Token)],
            terms = [item for item in items if isinstance(item, Level3)]
        )
    #~ CLASS -> 3 TERM CONSTRUCT
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
            numerator = numerator,
            denominator = denominator
        )
    #~ CLASS -> 4 FACTOR CONSTRUCT
    def factor(self, items: list[Level5 | Expression]) -> Factor:
        return Factor(
            value = items[0],
            exponent = items[1] if len(items) == 2 else None
        )
    #~ CLASS -> 4 LIMIT CONSTRUCT
    def limit(self, items: list[Variable | Expression | Token | Nest]) -> Limit:
        return Limit(
            variable = items[0],
            approach = items[1],
            direction = ñ(items[2]) == "+" if isinstance(items[2], Token) else None,
            of = items[-2] if isinstance(items[-2], Nest) else items[-1],
            exponent = items[-1] if isinstance(items[-1], Expression) else None
        )
    #~ CLASS -> 5 INFINITE CONSTRUCT
    def infinite(self, items: list) -> Infinite:
        return Infinite()
    #~ CLASS -> 5 VARIABLE CONSTRUCT
    def variable(self, items: list[Token]) -> Variable:
        return Variable(
            representation = ñ(items[0])
        )
    #~ CLASS -> 5 NEST CONSTRUCT
    def nest(self, items: list[Expression]) -> Nest:
        return Nest(
            expression = items[0] if len(items) == 1 else None
        )
    #~ CLASS -> 5 VECTOR CONSTRUCT
    def vector(self, items: list[Expression]) -> Vector:
        return Vector(
            values = items
        )
    #~ CLASS -> 5 NUMBER CONSTRUCT
    def number(self, items: list[Token]) -> Number:
        return Number(
            whole = ñ(items[0]),
            decimal = ñ(items[-1]) if len(items) == 2 else None
        )