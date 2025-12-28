#^
#^  HEAD
#^

#> HEAD -> MODULES
from dataclasses import dataclass

#> HEAD -> DATA
from .local import join, u8, null8, u32, null32


#^
#^  START
#^

#> START -> CLASS
@dataclass(kw_only = True, frozen = True)
class Start:
    code = u8(0x01)
    statements: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + join(self.statements, null32())


#^
#^  1ºLEVEL
#^

#> 1ºLEVEL -> DECLARATION
@dataclass(kw_only = True, frozen = True)
class Declaration:
    code = u8(0x02)
    group: u8 | null8
    variable: u32
    expression: u32
    def __bytes__(self) -> bytes:
        return self.code + self.group + self.variable + self.expression

#> 1ºLEVEL -> DEFINITION
@dataclass(kw_only = True, frozen = True)
class Definition:
    code = u8(0x03)
    group: u8 | null8
    variable: u32
    expression: u32
    def __bytes__(self) -> bytes:
        return self.code + self.group + self.variable + self.expression

#> 1ºLEVEL -> ANNOTATION
@dataclass(kw_only = True, frozen = True)
class Annotation:
    code = u8(0x04)
    group: u8 | null8
    variables: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + self.group + join(self.variables, null32())

#> 1ºLEVEL -> NODE
@dataclass(kw_only = True, frozen = True)
class Node:
    code = u8(0x05)
    expression: u32
    def __bytes__(self) -> bytes:
        return self.code + self.expression

#> 1ºLEVEL -> EQUATION
@dataclass(kw_only = True, frozen = True)
class Equation:
    code = u8(0x06)
    leftexpression: u32
    rightexpression: u32
    def __bytes__(self) -> bytes:
        return self.code + self.leftexpression + self.rightexpression

#> 1ºLEVEL -> COMMENT
@dataclass(kw_only = True, frozen = True)
class Comment:
    code = u8(0x07)
    text: list[u8]
    def __bytes__(self) -> bytes:
        return self.code + join(self.text, null8())

#> 1ºLEVEL -> USE
@dataclass(kw_only = True, frozen = True)
class Use:
    code = u8(0x08)
    name: list[u8]
    start: u32 | null32
    def __bytes__(self) -> bytes:
        return self.code + join(self.name, null8()) + self.start


#^
#^  2ºLEVEL
#^

#> 2ºLEVEL -> EXPRESSION
@dataclass(kw_only = True, frozen = True)
class Expression:
    code = u8(0x09)
    signs: list[u8]
    terms: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + join(self.signs, null8()) + join(self.terms, null32())


#^
#^  3ºLEVEL
#^

#> 3ºLEVEL -> TERM
@dataclass(kw_only = True, frozen = True)
class Term:
    code = u8(0x0A)
    numerator: list[u32]
    denominator: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + join(self.numerator, null32()) + join(self.denominator, null32())


#^
#^  4ºLEVEL
#^

#> 4ºLEVEL -> FACTOR
@dataclass(kw_only = True, frozen = True)
class Factor:
    code = u8(0x0B)
    value: u32
    exponent: u32 | null32
    def __bytes__(self) -> bytes:
        return self.code + self.value + self.exponent

#> 4ºLEVEL -> LIMIT
@dataclass(kw_only = True, frozen = True)
class Limit:
    code = u8(0x0C)
    variable: u32
    approach: u32
    direction: u8 | null8
    nest: u32
    exponent: u32 | null32
    def __bytes__(self) -> bytes:
        return self.code + self.variable + self.approach + self.direction + self.nest + self.exponent


#^
#^  5ºLEVEL
#^

#> 5ºLEVEL -> INFINITE
@dataclass(kw_only = True, frozen = True)
class Infinite:
    code = u8(0x0D)
    def __bytes__(self) -> bytes:
        return self.code

#> 5ºLEVEL -> VARIABLE
@dataclass(kw_only = True, frozen = True)
class Variable:
    code = u8(0x0E)
    representation: list[u8]
    def __bytes__(self) -> bytes:
        return self.code + join(self.representation, null8())

#> 5ºLEVEL -> NEST
@dataclass(kw_only = True, frozen = True)
class Nest:
    code = u8(0x0F)
    expression: u32 | null32
    def __bytes__(self) -> bytes:
        return self.code + self.expression

#> 5ºLEVEL -> TENSOR
@dataclass(kw_only = True, frozen = True)
class Tensor:
    code = u8(0x10)
    values: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + join(self.values, null32())

#> 5ºLEVEL -> WHOLE
@dataclass(kw_only = True, frozen = True)
class Whole:
    code = u8(0x11)
    value: u32 | null32
    def __bytes__(self) -> bytes:
        return self.code + self.value

#> 5ºLEVEL -> ABSOLUTE
@dataclass(kw_only = True, frozen = True)
class Absolute:
    code = u8(0x12)
    expression: u32
    def __bytes__(self) -> bytes:
        return self.code + self.expression