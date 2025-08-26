#
#   HEAD
#

# HEAD -> DATACLASSES
from dataclasses import dataclass
from .parser import (
    # DATACLASSES -> 1ºLEVEL
    Level1,
    Sheet, 
    # DATACLASSES -> 2ºLEVEL
    Level2,
    Definition,
    Declaration,
    Node,
    Equation,
    Comment,
    # DATACLASSES -> 3ºLEVEL
    Level3,
    Expression, 
    # DATACLASSES -> 4ºLEVEL
    Level4,
    Term, 
    # DATACLASSES -> 5ºLEVEL
    Level5,
    Infinite,
    Limit,
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

# TYPES -> JOIN
def join(binary: list[bytes]) -> bytes:
    result = b""
    for data in binary:
        result += data
    return result


#
#   1ºLEVEL
#

# 1ºLEVEL -> SHEET
@dataclass
class IRSheet(Sequence):
    code = u8(0x01)
    location: u32
    statements: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + self.location + (join(self.statements) + null32())


#
#   2ºLEVEL
#

# 2ºLEVEL -> DEFINITION
@dataclass
class IRDefinition(Sequence):
    code = u8(0x02)
    location: u32
    pointer: u32
    characters: list[u8]
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.pointer + (join(self.characters) + null8())

# 2ºLEVEL -> DECLARATION
@dataclass
class IRDeclaration(Sequence):
    code = u8(0x03)
    location: u32
    pointer: u32
    characters: list[u8]
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.pointer + (join(self.characters) + null8())

# 2ºLEVEL -> NODE
@dataclass
class IRNode(Sequence):
    code = u8(0x04)
    location: u32
    pointer: u32
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.pointer

# 2ºLEVEL -> EQUATION
@dataclass
class IREquation(Sequence):
    code = u8(0x05)
    location: u32
    left: u32
    right: u32
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.left + self.right

# 2ºLEVEL -> COMMENT
@dataclass
class IRComment(Sequence):
    code = u8(0x06)
    location: u32
    characters: list[u8]
    def __bytes__(self) -> bytes:
        return self.code + self.location + (join(self.characters) + null8())


#
#   3ºLEVEL
#

# 3ºLEVEL -> EXPRESSION
@dataclass
class IRExpression(Sequence):
    code = u8(0x07)
    location: u32
    terms: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + self.location + (join(self.terms) + null32())


#
#   4ºLEVEL
#

# 4ºLEVEL -> TERM
@dataclass
class IRTerm(Sequence):
    code = u8(0x08)
    location: u32
    numerator: list[u32]
    denominator: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + self.location + (join(self.numerator) + null32()) + (join(self.denominator) + null32())


#
#   5ºLEVEL
#

# 5ºLEVEL -> INFINITE
@dataclass
class IRInfinite(Sequence):
    code = u8(0x09)
    location: u32
    sign: u8
    exponent: u32 | null32
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.sign + self.exponent

# 5ºLEVEL -> LIMIT
@dataclass
class IRLimit(Sequence):
    code = u8(0x0A)
    location: u32
    sign: u8
    variable: list[u8]
    to: u32
    at: u32
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.sign + (join(self.characters) + null8()) + self.to + self.at

# 5ºLEVEL -> VARIABLE
@dataclass
class IRVariable(Sequence):
    code = u8(0x0B)
    location: u32
    sign: u8
    exponent: u32 | null32
    characters: list[u8]
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.sign + self.exponent + (join(self.characters) + null8())

# 5ºLEVEL -> NEST
@dataclass
class IRNest(Sequence):
    code = u8(0x0C)
    location: u32
    sign: u8
    exponent: u32 | null32
    pointer: u32
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.sign + self.exponent + self.pointer

# 5ºLEVEL -> VECTOR
@dataclass
class IRVector(Sequence):
    code = u8(0x0D)
    location: u32
    sign: u8
    exponent: u32 | null32
    pointers: list[u32]
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.sign + self.exponent + (join(self.pointers) + null32())

# 5ºLEVEL -> NUMBER
@dataclass
class IRNumber(Sequence):
    code = u8(0x0E)
    location: u32
    sign: u8
    exponent: u32 | null32
    value: u32 | null32
    shift: u8 | null8
    def __bytes__(self) -> bytes:
        return self.code + self.location + self.sign + self.exponent + self.value + self.shift


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
    def run(self, sheet: Sheet) -> bytes:
        self.ir = []
        self.counter = 0
        self.sheet(sheet)
        return join([bytes(sequence) for sequence in self.ir])
    # IR -> 1 SHEET GENERATION
    def sheet(self, sheet: Sheet) -> u32:
        statements = []
        for statement in sheet.statements:
            match statement:
                case Definition(): statements.append(self.definition(statement))
                case Declaration(): statements.append(self.declaration(statement))
                case Node(): statements.append(self.node(statement))
                case Equation(): statements.append(self.equation(statement))
                case Comment(): statements.append(self.comment(statement))
        register = self.new()
        self.ir.append(IRSheet(
            register,
            statements    
        ))
        return register
    # GENERATOR -> 2 DEFINITION GENERATION
    def definition(self, definition: Definition) -> u32:
        pointer = self.expression(definition.expression)
        register = self.new()
        self.ir.append(IRDefinition(
            register,
            pointer,
            [definition.identifier.encode()]
        ))
        return register
    # GENERATOR -> 2 DECLARATION GENERATION
    def declaration(self, declaration: Declaration) -> u32:
        pointer = self.expression(declaration.expression)
        register = self.new()
        self.ir.append(IRDeclaration(
            register,
            pointer,
            [declaration.identifier.encode()]
        ))
        return register
    # GENERATOR -> 2 NODE GENERATION
    def node(self, node: Node) -> u32:
        pointer = self.expression(node.expression)
        register = self.new()
        self.ir.append(IRNode(
            register,
            pointer
        ))
        return register
    # GENERATOR -> 2 EQUATION GENERATION
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
    # GENERATOR -> 2 COMMENT GENERATION
    def comment(self, comment: Comment) -> u32:
        register = self.new()
        self.ir.append(IRComment(
            register,
            [comment.text.encode()]
        ))
        return register
    # GENERATOR -> 3 EXPRESSION GENERATION
    def expression(self, expression: Expression) -> u32:
        terms = []
        for term in expression.terms: terms.append(self.term(term))
        register = self.new()
        self.ir.append(IRExpression(
            register,
            terms
        ))
        return register
    # GENERATOR -> 4 TERM GENERATION
    def term(self, term: Term) -> u32:
        numerator = []
        denominator = []
        for index in range(len(term.factors)):
            if index == 0: 
                dump = numerator
            else:
                match term.operators[index - 1]:
                    case "*" | "·": dump = numerator
                    case "/": dump = denominator
            match term.factors[index]:
                case Infinite(): dump.append(self.infinite(term.factors[index]))
                case Limit(): dump.append(self.limit(term.factors[index]))
                case Variable(): dump.append(self.variable(term.factors[index]))
                case Nest(): dump.append(self.nest(term.factors[index]))
                case Vector(): dump.append(self.vector(term.factors[index]))
                case Number(): dump.append(self.number(term.factors[index]))
        register = self.new()
        self.ir.append(IRTerm(
            register,
            numerator,
            denominator
        ))
        return register
    # GENERATOR -> 5 INFINITE GENERATION
    def infinite(self, infinite: Infinite) -> u32:
        exponent = self.expression(infinite.exponent) if infinite.exponent is not None else null32()
        register = self.new()
        self.ir.append(IRInfinite(
            register,
            u8(infinite.signs.count("-")) if infinite.signs is not None and infinite.signs.count("-") != 0 else null8(),
            exponent
        ))
        return register
    # GENERATOR -> 5 LIMIT GENERATION
    def limit(self, limit: Limit) -> u32:
        to = self.expression(limit.to)
        at = self.expression(limit.at)
        register = self.new()
        self.ir.append(IRLimit(
            register,
            u8(limit.signs.count("-")) if limit.signs is not None and limit.signs.count("-") != 0 else null8(),
            [limit.variable.encode()],
            to,
            at
        ))
    # GENERATOR -> 5 VARIABLE GENERATION
    def variable(self, variable: Variable) -> u32:
        exponent = self.expression(variable.exponent) if variable.exponent is not None else null32()
        register = self.new()
        self.ir.append(IRVariable(
            register,
            u8(variable.signs.count("-")) if variable.signs is not None and variable.signs.count("-") != 0 else null8(),
            exponent,
            [variable.representation.encode()]
        ))
        return register
    # GENERATOR -> 5 NEST GENERATION
    def nest(self, nest: Nest) -> u32:
        exponent = self.expression(nest.exponent) if nest.exponent is not None else null32()
        pointer = self.expression(nest.expression)
        register = self.new()
        self.ir.append(IRNest(
            register,
            u8(nest.signs.count("-")) if nest.signs is not None and nest.signs.count("-") != 0 else null8(),
            exponent,
            pointer
        ))
        return register
    # GENERATOR -> 5 VECTOR GENERATION
    def vector(self, vector: Vector) -> u32:
        exponent = self.expression(vector.exponent) if vector.exponent is not None else null32()
        pointers = []
        for value in vector.values: pointers.append(self.expression(value))
        register = self.new()
        self.ir.append(IRVector(
            register,
            u8(vector.signs.count("-")) if vector.signs is not None and vector.signs.count("-") != 0 else null8(),
            exponent,
            pointers
        ))
        return register
    # GENERATOR -> 5 NUMBER GENERATION
    def number(self, number: Number) -> u32:
        exponent = self.expression(number.exponent) if number.exponent is not None else null32()
        register = self.new()
        self.ir.append(IRNumber(
            register,
            u8(number.signs.count("-")) if number.signs is not None and number.signs.count("-") != 0 else null8(),
            exponent,
            u32(int(number.representation.replace(".", ""))) if float(number.representation) != 0 else null32(),
            u32(len(number.representation.split(".")[1])) if "." in number.representation else null8()
        ))
        return register