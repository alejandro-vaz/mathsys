#^
#^  HEAD
#^

#> HEAD -> MODULES
from __future__ import annotations
from dataclasses import dataclass


#^
#^  START
#^

#> START -> CLASS
@dataclass
class Start:
    statements: list[Level1]


#^
#^  1ºLEVEL
#^

#> 1ºLEVEL -> NAMESPACE
class Level1: pass

#> 1ºLEVEL -> DECLARATION
@dataclass
class Declaration(Level1):
    group: str | None
    variable: Variable
    expression: Expression

#> 1ºLEVEL -> DEFINITION
@dataclass
class Definition(Level1):
    group: str | None
    variable: Variable
    expression: Expression

#> 1ºLEVEL -> ANNOTATION
@dataclass
class Annotation(Level1):
    group: str
    variables: list[Variable]

#> 1ºLEVEL -> NODE
@dataclass
class Node(Level1):
    expression: Expression

#> 1ºLEVEL -> EQUATION
@dataclass
class Equation(Level1):
    leftexpression: Expression
    rightexpression: Expression

#> 1ºLEVEL -> COMMENT
@dataclass
class Comment(Level1):
    text: str

#> 1ºLEVEL -> USE
@dataclass
class Use(Level1):
    name: str
    start: Start | None


#^
#^  2ºLEVEL
#^

#> 2ºLEVEL -> NAMESPACE
class Level2: pass

#> 2ºLEVEL -> EXPRESSION
@dataclass
class Expression(Level2):
    signs: list[str | None]
    terms: list[Level3]


#^
#^  3ºLEVEL
#^

#> 3ºLEVEL -> NAMESPACE
class Level3: pass

#> 3ºLEVEL -> TERM
@dataclass
class Term(Level3):
    numerator: list[Level4]
    denominator: list[Level4]


#^
#^  4ºLEVEL
#^

#> 4ºLEVEL -> NAMESPACE
class Level4: pass

#> 4ºLEVEL -> FACTOR
@dataclass
class Factor(Level4):
    value: Level5
    exponent: Expression | None

#> 4ºLEVEL -> LIMIT
@dataclass
class Limit(Level4):
    variable: Variable
    approach: Expression
    direction: bool | None
    nest: Nest
    exponent: Expression | None


#^
#^  5ºLEVEL
#^

#> 5ºLEVEL -> NAMESPACE
class Level5: pass

#> 5ºLEVEL -> INFINITE
@dataclass
class Infinite(Level5): pass

#> 5ºLEVEL -> VARIABLE
@dataclass
class Variable(Level5):
    representation: str

#> 5ºLEVEL -> NEST
@dataclass
class Nest(Level5):
    expression: Expression | None

#> 5ºLEVEL -> TENSOR
@dataclass
class Tensor(Level5):
    values: list[Expression]

#> 5ºLEVEL -> NUMBER
@dataclass
class Number(Level5):
    value: int
    shift: int