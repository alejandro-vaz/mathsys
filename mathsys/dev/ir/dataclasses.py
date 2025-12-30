#^
#^  HEAD
#^

#> HEAD -> MODULES
from dataclasses import dataclass

#> HEAD -> DATA
from .local import Opcode, Pointer, Sign, Option, BigUint, String, Group, Vec


#^
#^  START
#^

#> START -> CLASS
@dataclass(kw_only = True, frozen = True)
class Start:
    binary = lambda self: self.code + self.statements
    code = Opcode(0x01).binary()
    statements: Vec[Pointer]
    


#^
#^  1ºLEVEL
#^

#> 1ºLEVEL -> DECLARATION
@dataclass(kw_only = True, frozen = True)
class Declaration:
    binary = lambda self: self.code + self.group + self.variable + self.expression
    code = Opcode(0x02).binary()
    group: Group
    variable: Pointer
    expression: Pointer

#> 1ºLEVEL -> DEFINITION
@dataclass(kw_only = True, frozen = True)
class Definition:
    binary = lambda self: self.code + self.group + self.variable + self.expression
    code = Opcode(0x03).binary()
    group: Group
    variable: Pointer
    expression: Pointer



#> 1ºLEVEL -> ANNOTATION
@dataclass(kw_only = True, frozen = True)
class Annotation:
    binary = lambda self: self.code + self.group + self.variables
    code = Opcode(0x04).binary()
    group: Group
    variables: Vec[Pointer]

#> 1ºLEVEL -> NODE
@dataclass(kw_only = True, frozen = True)
class Node:
    binary = lambda self: self.code + self.expression
    code = Opcode(0x05).binary()
    expression: Pointer

#> 1ºLEVEL -> EQUATION
@dataclass(kw_only = True, frozen = True)
class Equation:
    binary = lambda self: self.code + self.leftexpression + self.rightexpression
    code = Opcode(0x06).binary()
    leftexpression: Pointer
    rightexpression: Pointer

#> 1ºLEVEL -> COMMENT
@dataclass(kw_only = True, frozen = True)
class Comment:
    binary = lambda self: self.code + self.text
    code = Opcode(0x07).binary()
    text: String

#> 1ºLEVEL -> USE
@dataclass(kw_only = True, frozen = True)
class Use:
    binary = lambda self: self.code + self.name + self.start
    code = Opcode(0x08).binary()
    name: String
    start: Option[Pointer]


#^
#^  2ºLEVEL
#^

#> 2ºLEVEL -> EXPRESSION
@dataclass(kw_only = True, frozen = True)
class Expression:
    binary = lambda self: self.code + self.signs + self.terms
    code = Opcode(0x09).binary()
    signs: Vec[Option[Sign]]
    terms: Vec[Pointer]


#^
#^  3ºLEVEL
#^

#> 3ºLEVEL -> TERM
@dataclass(kw_only = True, frozen = True)
class Term:
    binary = lambda self: self.code + self.numerator + self.denominator
    code = Opcode(0x0A).binary()
    numerator: Vec[Pointer]
    denominator: Vec[Pointer]


#^
#^  4ºLEVEL
#^

#> 4ºLEVEL -> FACTOR
@dataclass(kw_only = True, frozen = True)
class Factor:
    binary = lambda self: self.code + self.value + self.exponent
    code = Opcode(0x0B).binary()
    value: Pointer
    exponent: Option[Pointer]

#> 4ºLEVEL -> LIMIT
@dataclass(kw_only = True, frozen = True)
class Limit:
    binary = lambda self: self.code + self.variable + self.approach + self.direction + self.nest + self.exponent
    code = Opcode(0x0C).binary()
    variable: Pointer
    approach: Pointer
    direction: Option[Sign]
    nest: Pointer
    exponent: Option[Pointer]


#^
#^  5ºLEVEL
#^

#> 5ºLEVEL -> INFINITE
@dataclass(kw_only = True, frozen = True)
class Infinite:
    binary = lambda self: self.code
    code = Opcode(0x0D).binary()

#> 5ºLEVEL -> VARIABLE
@dataclass(kw_only = True, frozen = True)
class Variable:
    binary = lambda self: self.code + self.representation
    code = Opcode(0x0E).binary()
    representation: String

#> 5ºLEVEL -> NEST
@dataclass(kw_only = True, frozen = True)
class Nest:
    binary = lambda self: self.code + self.expression
    code = Opcode(0x0F).binary()
    expression: Option[Pointer]

#> 5ºLEVEL -> TENSOR
@dataclass(kw_only = True, frozen = True)
class Tensor:
    binary = lambda self: self.code + self.values
    code = Opcode(0x10).binary()
    values: Vec[Pointer]

#> 5ºLEVEL -> WHOLE
@dataclass(kw_only = True, frozen = True)
class Whole:
    binary = lambda self: self.code + self.value
    code = Opcode(0x11).binary()
    value: BigUint

#> 5ºLEVEL -> ABSOLUTE
@dataclass(kw_only = True, frozen = True)
class Absolute:
    binary = lambda self: self.code + self.expression
    code = Opcode(0x12).binary()
    expression: Pointer