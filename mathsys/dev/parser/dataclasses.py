#^
#^  HEAD
#^

#> HEAD -> MODULES
from __future__ import annotations
from dataclasses import dataclass
from abc import ABC


#^
#^  START
#^

#> START -> CLASS
@dataclass(kw_only = True, frozen = True)
class Start:
    statements: list[Level1]


#^
#^  1ºLEVEL
#^

#> 1ºLEVEL -> NAMESPACE
class Level1(ABC): pass

#> 1ºLEVEL -> DECLARATION
@dataclass(kw_only = True, frozen = True)
class Declaration(Level1):
    group: str | None
    variable: Variable
    expression: Expression

#> 1ºLEVEL -> DEFINITION
@dataclass(kw_only = True, frozen = True)
class Definition(Level1):
    group: str | None
    variable: Variable
    expression: Expression

#> 1ºLEVEL -> ANNOTATION
@dataclass(kw_only = True, frozen = True)
class Annotation(Level1):
    group: str
    variables: list[Variable]

#> 1ºLEVEL -> NODE
@dataclass(kw_only = True, frozen = True)
class Node(Level1):
    expression: Expression

#> 1ºLEVEL -> EQUATION
@dataclass(kw_only = True, frozen = True)
class Equation(Level1):
    leftexpression: Expression
    rightexpression: Expression

#> 1ºLEVEL -> COMMENT
@dataclass(kw_only = True, frozen = True)
class Comment(Level1):
    text: str

#> 1ºLEVEL -> USE
@dataclass(kw_only = True, frozen = True)
class Use(Level1):
    name: str
    start: Start | None


#^
#^  2ºLEVEL
#^

#> 2ºLEVEL -> NAMESPACE
class Level2(ABC): pass

#> 2ºLEVEL -> EXPRESSION
@dataclass(kw_only = True, frozen = True)
class Expression(Level2):
    signs: list[bool | None]
    terms: list[Level3]


#^
#^  3ºLEVEL
#^

#> 3ºLEVEL -> NAMESPACE
class Level3(ABC): pass

#> 3ºLEVEL -> TERM
@dataclass(kw_only = True, frozen = True)
class Term(Level3):
    numerator: list[Level4]
    denominator: list[Level4]


#^
#^  4ºLEVEL
#^

#> 4ºLEVEL -> NAMESPACE
class Level4(ABC): pass

#> 4ºLEVEL -> FACTOR
@dataclass(kw_only = True, frozen = True)
class Factor(Level4):
    value: Level5
    exponent: Expression | None

#> 4ºLEVEL -> LIMIT
@dataclass(kw_only = True, frozen = True)
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
class Level5(ABC): pass

#> 5ºLEVEL -> INFINITE
@dataclass(kw_only = True, frozen = True)
class Infinite(Level5): pass

#> 5ºLEVEL -> VARIABLE
@dataclass(kw_only = True, frozen = True)
class Variable(Level5):
    representation: str

#> 5ºLEVEL -> NEST
@dataclass(kw_only = True, frozen = True)
class Nest(Level5):
    expression: Expression | None

#> 5ºLEVEL -> TENSOR
@dataclass(kw_only = True, frozen = True)
class Tensor(Level5):
    values: list[Expression]

#> 5ºLEVEL -> WHOLE
@dataclass(kw_only = True, frozen = True)
class Whole(Level5):
    value: int

#> 5ºLEVEL -> ABSOLUTE
@dataclass(kw_only = True, frozen = True)
class Absolute(Level5):
    expression: Expression