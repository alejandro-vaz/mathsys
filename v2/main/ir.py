#
#   HEAD
#

# HEAD -> DATACLASSES
from dataclasses import dataclass
from .parser import (
    # DATACLASSES -> START
    Start,
    # DATACLASSES -> 1ºLEVEL
    Level1,
    Debug,
    Declaration,
    Definition,
    Node,
    Equation,
    Comment, 
    # DATACLASSES -> 2ºLEVEL
    Level2,
    Expression,
    # DATACLASSES -> 3ºLEVEL
    Level3,
    Term,
    # DATACLASSES -> 4ºLEVEL
    Level4,
    Factor,
    Limit,
    # DATACLASSES -> 5ºLEVEL
    Level5,
    Infinite,
    Variable,
    Nest,
    Vector,
    Number
)


#
#   TYPES
#

# TYPES -> U8 CLASS
class u8:
    def __new__(self, value: int) -> bytes:
        if not 1 <= value <= 2**8 - 1: raise ValueError(f"'{value}' is outside range for u8.")
        return bytes([value])

# TYPES -> U32 CLASS
class u32:
    def __new__(self, value: int) -> bytes:
        if not 1 <= value <= 2**32 - 1: raise ValueError(f"'{value}' is outside range for u32.")
        return bytes([
            (value) & 0xFF,
            (value >> 8) & 0xFF,
            (value >> 16) & 0xFF,
            (value >> 24) & 0xFF
        ])

# TYPES -> NULL8 CLASS
class null8:
    def __new__(self) -> bytes: return bytes([0])

# TYPES -> NULL32 CLASS
class null32:
    def __new__(cls) -> bytes: return bytes([0, 0, 0, 0])

# TYPES -> NAMESPACE
class Sequence:
    code: u8
    location: u32

# TYPES -> JOIN
def join(binary: list[bytes]) -> bytes:
    result = b""
    for data in binary:
        result += data
    return result


#
#   START
#

# START -> CLASS
@dataclass
class IRStart(Sequence):
    code = u8(0x01)
    location: u32
    statements: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + self.location + (join(self.statements) + null32())


#
#   1ºLEVEL
#

# 1ºLEVEL -> DEBUG
@dataclass
class IRDebug(Sequence):
    code = u8(0x02)
    location: u32
    def __bytes__(self) -> bytes:
        return self.code + self.location

# 1ºLEVEL -> DECLARATION
@dataclass
class IRDeclaration(Sequence):
    code = u8(0x03)
    location: u32
    variable: u32
    pointer: u32
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.variable + self.pointer

# 1ºLEVEL -> DEFINITION
@dataclass
class IRDefinition(Sequence):
    code = u8(0x04)
    location: u32
    variable: u32
    pointer: u32
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.variable + self.pointer

# 1ºLEVEL -> NODE
@dataclass
class IRNode(Sequence):
    code = u8(0x05)
    location: u32
    pointer: u32
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.pointer

# 1ºLEVEL -> EQUATION
@dataclass
class IREquation(Sequence):
    code = u8(0x06)
    location: u32
    left: u32
    right: u32
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.left + self.right

# 1ºLEVEL -> COMMENT
@dataclass
class IRComment(Sequence):
    code = u8(0x07)
    location: u32
    characters: list[u8]
    def __bytes__(self) -> bytes:
        return self.code + self.location + (join(self.characters) + null8())


#
#   2ºLEVEL
#

# 2ºLEVEL -> EXPRESSION
@dataclass
class IRExpression(Sequence):
    code = u8(0x08)
    location: u32
    terms: list[u32]
    signs: list[u8 | null8]
    def __bytes__(self) -> bytes:
        return self.code + self.location + (join(self.terms) + null32()) + join(self.signs)


#
#   3ºLEVEL
#

# 3ºLEVEL -> TERM
@dataclass
class IRTerm(Sequence):
    code = u8(0x09)
    location: u32
    numerator: list[u32]
    denominator: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + self.location + (join(self.numerator) + null32()) + (join(self.denominator) + null32())


#
#   4ºLEVEL
#

# 4ºLEVEL -> FACTOR
@dataclass
class IRFactor(Sequence):
    code = u8(0x0A)
    location: u32
    pointer: u32
    expression: u32 | null32
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.pointer + self.expression

# 4ºLEVEL -> LIMIT
@dataclass
class IRLimit(Sequence):
    code = u8(0x0B)
    location: u32
    variable: u32
    approach: u32
    direction: u8 | null8
    pointer: u32
    exponent: u32 | null32
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.variable + self.approach + self.direction + self.pointer + self.exponent


#
#   5ºLEVEL
#

# 5ºLEVEL -> INFINITE
@dataclass
class IRInfinite(Sequence):
    code = u8(0x0C)
    location: u32
    def __bytes__(self) -> bytes:
        return self.code + self.location

# 5ºLEVEL -> VARIABLE
@dataclass
class IRVariable(Sequence):
    code = u8(0x0D)
    location: u32
    characters: list[u8]
    def __bytes__(self) -> bytes:
        return self.code + self.location + (join(self.characters) + null8())

# 5ºLEVEL -> NEST
@dataclass
class IRNest(Sequence):
    code = u8(0x0E)
    location: u32
    pointer: u32
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.pointer

# 5ºLEVEL -> VECTOR
@dataclass
class IRVector(Sequence):
    code = u8(0x0F)
    location: u32
    values: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + self.location + (join(self.values) + null32())

# 5ºLEVEL -> 
@dataclass
class IRNumber(Sequence):
    code = u8(0x10)
    location: u32
    value: u32
    shift: u8 | null8
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.value + self.shift


#
#   IR
#

# IR -> GENERATOR
class IR:
    # IR -> VARIABLES
    ir: list[Sequence]
    counter: int
    # IR -> INIT
    def __init__(self) -> None:
        self.ir = []
        self.counter = 0
    # GENERATOR -> VARIABLE GENERATOR
    def new(self) -> u32:
        self.counter += 1
        return u32(self.counter)
    # IR -> RUN
    def run(self, start: Start) -> bytes:
        self.ir = []
        self.counter = 0
        self.start(start)
        return join([bytes(sequence) for sequence in self.ir])
    # IR -> START GENERATION
    def start(self, start: Start) -> u32:
        statements = [self.level1(statement) for statement in start.statements]
        register = self.new()
        self.ir.append(IRStart(
            register,
            statements    
        ))
        return register
    # IR -> 1 LEVEL GENERATION
    def level1(self, level1: Level1) -> u32:
        match level1:
            case Debug(): return self.debug(level1)
            case Declaration(): return self.declaration(level1)
            case Definition(): return self.definition(level1)
            case Node(): return self.node(level1)
            case Equation(): return self.equation(level1)
            case Comment(): return self.comment(level1)
    # IR -> 1 DEBUG GENERATION
    def debug(self, debug: Debug) -> u32:
        register = self.new()
        self.ir.append(IRDebug(
            register
        ))
        return register
    # IR -> 1 DECLARATION GENERATION
    def declaration(self, declaration: Declaration) -> u32:
        variable = self.variable(declaration.identifier)
        pointer = self.expression(declaration.expression)
        register = self.new()
        self.ir.append(IRDeclaration(
            register,
            variable,
            pointer
        ))
        return register
    # IR -> 1 DEFINITION GENERATION
    def definition(self, definition: Definition) -> u32:
        variable = self.variable(definition.identifier)
        pointer = self.expression(definition.expression)
        register = self.new()
        self.ir.append(IRDefinition(
            register,
            variable,
            pointer
        ))
        return register
    # IR -> 1 NODE GENERATION
    def node(self, node: Node) -> u32:
        pointer = self.expression(node.value)
        register = self.new()
        self.ir.append(IRNode(
            register,
            pointer
        ))
        return register
    # IR -> 1 EQUATION GENERATION
    def equation(self, equation: Equation) -> u32:
        left = self.expression(equation.left)
        right = self.expression(equation.right)
        register = self.new()
        self.ir.append(IREquation(
            register,
            left,
            right
        ))
        return register
    # IR -> 1 COMMENT GENERATION
    def comment(self, comment: Comment) -> u32:
        register = self.new()
        self.ir.append(IRComment(
            register,
            [comment.content.encode()]
        ))
        return register
    # IR -> 2 LEVEL GENERATION
    def level2(self, level2: Level2) -> u32:
        match level2:
            case Expression(): return self.expression(level2)
    # IR -> 2 EXPRESSION GENERATION
    def expression(self, expression: Expression) -> u32:
        terms = [self.level3(item) for item in expression.terms]
        signs = [null8() if sign is None or sign.count("-") == 0 else u8(sign.count("-")) for sign in expression.signs]
        register = self.new()
        self.ir.append(IRExpression(
            register,
            terms,
            signs
        ))
        return register
    # IR -> 3 LEVEL GENERATION
    def level3(self, level3: Level3) -> u32:
        match level3:
            case Term(): return self.term(level3)
    # IR -> 3 TERM GENERATION
    def term(self, term: Term) -> u32:
        numerator = [self.level4(item) for item in term.numerator]
        denominator = [self.level4(item) for item in term.denominator]
        register = self.new()
        self.ir.append(IRTerm(
            register,
            numerator,
            denominator
        ))
        return register
    # IR -> 4 LEVEL GENERATION
    def level4(self, level4: Level4) -> u32:
        match level4:
            case Factor(): return self.factor(level4)
            case Limit(): return self.limit(level4)
    # IR -> 4 FACTOR GENERATION
    def factor(self, factor: Factor) -> u32:
        expression = self.expression(factor.exponent) if factor.exponent is not None else null32()
        pointer = self.level5(factor.value)
        register = self.new()
        self.ir.append(IRFactor(
            register,
            pointer,
            expression
        ))
        return register
    # IR -> 4 LIMIT GENERATION
    def limit(self, limit: Limit) -> u32:
        variable = self.variable(limit.variable)
        approach = self.expression(limit.approach)
        pointer = self.nest(limit.of)
        exponent = self.expression(limit.exponent) if limit.exponent is not None else null32()
        register = self.new()
        self.ir.append(IRLimit(
            register,
            variable,
            approach,
            u8(int(limit.direction) + 1) if limit.direction is not None else null8(),
            pointer,
            exponent
        ))
        return register
    # IR -> 5 LEVEL GENERATION
    def level5(self, level5: Level5) -> u32:
        match level5:
            case Infinite(): return self.infinite(level5)
            case Variable(): return self.variable(level5)
            case Nest(): return self.nest(level5)
            case Vector(): return self.vector(level5)
            case Number(): return self.number(level5)
    # IR -> 5 INFINITE GENERATION
    def infinite(self, infinite: Infinite) -> u32:
        register = self.new()
        self.ir.append(IRInfinite(
            register
        ))
        return register
    # IR -> 5 VARIABLE GENERATION
    def variable(self, variable: Variable) -> u32:
        register = self.new()
        self.ir.append(IRVariable(
            register,
            [variable.representation.encode()]
        ))
        return register
    # IR -> 5 NEST GENERATION
    def nest(self, nest: Nest) -> u32:
        pointer = self.expression(nest.expression)
        register = self.new()
        self.ir.append(IRNest(
            register,
            pointer
        ))
        return register
    # IR -> 5 VECTOR GENERATION
    def vector(self, vector: Vector) -> u32:
        values = [self.expression(value) for value in vector.values]
        register = self.new()
        self.ir.append(IRVector(
            register,
            values
        ))
        return register
    # IR -> 5 NUMBER GENERATION
    def number(self, number: Number) -> u32:
        register = self.new()
        self.ir.append(IRNumber(
            register,
            u32(int(number.whole + (number.decimal if number.decimal is not None else "")) + 1),
            u8(len(number.decimal)) if number.decimal is not None else null8()
        ))
        return register