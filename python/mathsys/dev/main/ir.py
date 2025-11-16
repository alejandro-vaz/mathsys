#^
#^  HEAD
#^

#> HEAD -> DATACLASSES
from dataclasses import dataclass
from .parser import (
    #~ DATACLASSES -> START
    Start,
    #~ DATACLASSES -> 1ºLEVEL
    Level1,
    Declaration,
    Definition,
    Node,
    Equation,
    Comment, 
    #~ DATACLASSES -> 2ºLEVEL
    Level2,
    Expression,
    #~ DATACLASSES -> 3ºLEVEL
    Level3,
    Term,
    #~ DATACLASSES -> 4ºLEVEL
    Level4,
    Factor,
    Limit,
    #~ DATACLASSES -> 5ºLEVEL
    Level5,
    Infinite,
    Variable,
    Nest,
    Vector,
    Number
)


#^
#^  TYPES
#^

#> TYPES -> U8 CLASS
class u8:
    def __new__(self, value: int) -> bytes:
        if not 1 <= value <= 2**8 - 1: raise ValueError(f"'{value}' is outside range for u8.")
        return bytes([value])

#> TYPES -> U32 CLASS
class u32:
    def __new__(self, value: int) -> bytes:
        if not 1 <= value <= 2**32 - 1: raise ValueError(f"'{value}' is outside range for u32.")
        return bytes([
            (value) & 0xFF,
            (value >> 8) & 0xFF,
            (value >> 16) & 0xFF,
            (value >> 24) & 0xFF
        ])

#> TYPES -> NULL8 CLASS
class null8:
    def __new__(self) -> bytes: return bytes([0])

#> TYPES -> NULL32 CLASS
class null32:
    def __new__(self) -> bytes: return bytes([0, 0, 0, 0])

#> TYPES -> NAMESPACE
class Sequence:
    code: u8

#> TYPES -> JOIN
def join(binary: list[bytes]) -> bytes:
    return b"".join(binary)


#^
#^  START
#^

#> START -> CLASS
@dataclass
class IRStart(Sequence):
    code = u8(0x01)
    statements: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + (join(self.statements) + null32())


#^
#^  1ºLEVEL
#^

#> 1ºLEVEL -> DECLARATION
@dataclass
class IRDeclaration(Sequence):
    code = u8(0x02)
    variable: u32
    pointer: u32
    def __bytes__(self) -> bytes:
        return self.code + self.variable + self.pointer

#> 1ºLEVEL -> DEFINITION
@dataclass
class IRDefinition(Sequence):
    code = u8(0x03)
    variable: u32
    pointer: u32
    def __bytes__(self) -> bytes:
        return self.code + self.variable + self.pointer

#> 1ºLEVEL -> NODE
@dataclass
class IRNode(Sequence):
    code = u8(0x04)
    pointer: u32
    def __bytes__(self) -> bytes:
        return self.code + self.pointer

#> 1ºLEVEL -> EQUATION
@dataclass
class IREquation(Sequence):
    code = u8(0x05)
    left: u32
    right: u32
    def __bytes__(self) -> bytes:
        return self.code + self.left + self.right

#> 1ºLEVEL -> COMMENT
@dataclass
class IRComment(Sequence):
    code = u8(0x06)
    characters: list[u8]
    def __bytes__(self) -> bytes:
        return self.code + (join(self.characters) + null8())


#^
#^  2ºLEVEL
#^

#> 2ºLEVEL -> EXPRESSION
@dataclass
class IRExpression(Sequence):
    code = u8(0x07)
    terms: list[u32]
    signs: list[u8]
    def __bytes__(self) -> bytes:
        return self.code + (join(self.terms) + null32()) + (join(self.signs) + null8())


#^
#^  3ºLEVEL
#^

#> 3ºLEVEL -> TERM
@dataclass
class IRTerm(Sequence):
    code = u8(0x08)
    numerator: list[u32]
    denominator: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + (join(self.numerator) + null32()) + (join(self.denominator) + null32())


#^
#^  4ºLEVEL
#^

#> 4ºLEVEL -> FACTOR
@dataclass
class IRFactor(Sequence):
    code = u8(0x09)
    pointer: u32
    expression: u32 | null32
    def __bytes__(self) -> bytes:
        return self.code + self.pointer + self.expression

#> 4ºLEVEL -> LIMIT
@dataclass
class IRLimit(Sequence):
    code = u8(0x0A)
    variable: u32
    approach: u32
    direction: u8 | null8
    pointer: u32
    exponent: u32 | null32
    def __bytes__(self) -> bytes:
        return self.code + self.variable + self.approach + self.direction + self.pointer + self.exponent


#^
#^  5ºLEVEL
#^

#> 5ºLEVEL -> INFINITE
@dataclass
class IRInfinite(Sequence):
    code = u8(0x0B)
    def __bytes__(self) -> bytes:
        return self.code

#> 5ºLEVEL -> VARIABLE
@dataclass
class IRVariable(Sequence):
    code = u8(0x0C)
    characters: list[u8]
    def __bytes__(self) -> bytes:
        return self.code + (join(self.characters) + null8())

#> 5ºLEVEL -> NEST
@dataclass
class IRNest(Sequence):
    code = u8(0x0D)
    pointer: u32 | null32
    def __bytes__(self) -> bytes:
        return self.code + self.pointer

#> 5ºLEVEL -> VECTOR
@dataclass
class IRVector(Sequence):
    code = u8(0x0E)
    values: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + (join(self.values) + null32())

#> 5ºLEVEL -> NUMBER
@dataclass
class IRNumber(Sequence):
    code = u8(0x0F)
    value: u32 | null32
    shift: u8 | null8
    def __bytes__(self) -> bytes:
        return self.code + self.value + self.shift


#^
#^  IR
#^

#> IR -> GENERATOR
class IR:
    #~ IR -> VARIABLES
    ir: list[Sequence]
    counter: int
    #~ IR -> INIT
    def __init__(self) -> None:
        self.ir = []
        self.counter = 0
    #~ GENERATOR -> VARIABLE GENERATOR
    def new(self) -> u32:
        self.counter += 1
        return u32(self.counter)
    #~ IR -> RUN
    def run(self, start: Start) -> bytes:
        self.ir = []
        self.counter = 0
        self.start(start)
        return join([bytes(sequence) for sequence in self.ir])
    #~ IR -> START GENERATION
    def start(self, start: Start) -> u32:
        self.ir.append(IRStart(
            statements = [self.level1(statement) for statement in start.statements]
        ))
        return self.new()
    #~ IR -> 1 LEVEL GENERATION
    def level1(self, level1: Level1) -> u32:
        match level1:
            case Declaration(): return self.declaration(level1)
            case Definition(): return self.definition(level1)
            case Node(): return self.node(level1)
            case Equation(): return self.equation(level1)
            case Comment(): return self.comment(level1)
    #~ IR -> 1 DECLARATION GENERATION
    def declaration(self, declaration: Declaration) -> u32:
        self.ir.append(IRDeclaration(
            variable = self.variable(declaration.identifier),
            pointer = self.expression(declaration.expression)
        ))
        return self.new()
    #~ IR -> 1 DEFINITION GENERATION
    def definition(self, definition: Definition) -> u32:
        self.ir.append(IRDefinition(
            variable = self.variable(definition.identifier),
            pointer = self.expression(definition.expression)
        ))
        return self.new()
    #~ IR -> 1 NODE GENERATION
    def node(self, node: Node) -> u32:
        self.ir.append(IRNode(
            pointer = self.expression(node.value)
        ))
        return self.new()
    #~ IR -> 1 EQUATION GENERATION
    def equation(self, equation: Equation) -> u32:
        self.ir.append(IREquation(
            left = self.expression(equation.left),
            right = self.expression(equation.right)
        ))
        return self.new()
    #~ IR -> 1 COMMENT GENERATION
    def comment(self, comment: Comment) -> u32:
        self.ir.append(IRComment(
            characters = [comment.content.encode()]
        ))
        return self.new()
    #~ IR -> 2 LEVEL GENERATION
    def level2(self, level2: Level2) -> u32:
        match level2:
            case Expression(): return self.expression(level2)
    #~ IR -> 2 EXPRESSION GENERATION
    def expression(self, expression: Expression) -> u32:
        self.ir.append(IRExpression(
            terms = [self.level3(item) for item in expression.terms],
            signs = [u8(1) if sign is None or sign.count("-") == 0 else u8(sign.count("-") + 1) for sign in expression.signs]
        ))
        return self.new()
    #~ IR -> 3 LEVEL GENERATION
    def level3(self, level3: Level3) -> u32:
        match level3:
            case Term(): return self.term(level3)
    #~ IR -> 3 TERM GENERATION
    def term(self, term: Term) -> u32:
        self.ir.append(IRTerm(
            numerator = [self.level4(item) for item in term.numerator],
            denominator = [self.level4(item) for item in term.denominator]
        ))
        return self.new()
    #~ IR -> 4 LEVEL GENERATION
    def level4(self, level4: Level4) -> u32:
        match level4:
            case Factor(): return self.factor(level4)
            case Limit(): return self.limit(level4)
    #~ IR -> 4 FACTOR GENERATION
    def factor(self, factor: Factor) -> u32:
        self.ir.append(IRFactor(
            pointer = self.level5(factor.value),
            expression = self.expression(factor.exponent) if factor.exponent is not None else null32()
        ))
        return self.new()
    #~ IR -> 4 LIMIT GENERATION
    def limit(self, limit: Limit) -> u32:
        self.ir.append(IRLimit(
            variable = self.variable(limit.variable),
            approach = self.expression(limit.approach),
            direction = u8(int(limit.direction) + 1) if limit.direction is not None else null8(),
            pointer = self.nest(limit.of),
            exponent = self.expression(limit.exponent) if limit.exponent is not None else null32()
        ))
        return self.new()
    #~ IR -> 5 LEVEL GENERATION
    def level5(self, level5: Level5) -> u32:
        match level5:
            case Infinite(): return self.infinite(level5)
            case Variable(): return self.variable(level5)
            case Nest(): return self.nest(level5)
            case Vector(): return self.vector(level5)
            case Number(): return self.number(level5)
    #~ IR -> 5 INFINITE GENERATION
    def infinite(self, infinite: Infinite) -> u32:
        self.ir.append(IRInfinite())
        return self.new()
    #~ IR -> 5 VARIABLE GENERATION
    def variable(self, variable: Variable) -> u32: 
        self.ir.append(IRVariable(
            characters = [variable.representation.encode()]
        ))
        return self.new()
    #~ IR -> 5 NEST GENERATION
    def nest(self, nest: Nest) -> u32:
        self.ir.append(IRNest(
            pointer = self.expression(nest.expression) if nest.expression is not None else null32()
        ))
        return self.new()
    #~ IR -> 5 VECTOR GENERATION
    def vector(self, vector: Vector) -> u32:
        self.ir.append(IRVector(
            values = [self.expression(value) for value in vector.values]
        ))
        return self.new()
    #~ IR -> 5 NUMBER GENERATION
    def number(self, number: Number) -> u32:
        self.ir.append(IRNumber(
            value = u32(int(number.whole + (number.decimal if number.decimal is not None else ""))) if int(number.whole) != 0 or int(number.decimal if number.decimal is not None else "0") != 0 else null32(),
            shift = u8(len(number.decimal)) if number.decimal is not None else null8()
        ))
        return self.new()