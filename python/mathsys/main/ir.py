#
#   HEAD
#

# HEAD -> DATACLASSES
from atexit import register
from dataclasses import dataclass
from .parser import (
    # DATACLASSES -> 1ºLEVEL
    Level1,
    Sheet, 
    # DATACLASSES -> 2ºLEVEL
    Level2,
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
    Variable,
    Nest,
    Vector,
    Number
)


#
#   NODES
#

# NODES -> NAMESPACE
class Sequence: pass


#
#   1ºLEVEL
#

# 1ºLEVEL -> SHEET
@dataclass
class IRSheet(Sequence):
    code = b"\x00"
    adr: bytes
    statementCount: bytes
    statements: bytes
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.statementCount + self.statements


#
#   2ºLEVEL
#

# 2ºLEVEL -> DECLARATION
@dataclass
class IRDeclaration(Sequence):
    code = b"\x01"
    adr: bytes
    charAmount: bytes
    chars: bytes
    pointer: bytes
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.charAmount + self.chars + self.pointer

# 2ºLEVEL -> NODE
@dataclass
class IRNode(Sequence):
    code = b"\x02"
    adr: bytes
    pointer: bytes
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.pointer

# 2ºLEVEL -> EQUATION
@dataclass
class IREquation(Sequence):
    code = b"\x03"
    adr: bytes
    left: bytes
    right: bytes
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.left + self.right

# 2ºLEVEL -> COMMENT
@dataclass
class IRComment(Sequence):
    code = b"\x04"
    adr: bytes
    amount: bytes
    chars: bytes
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.amount + self.chars


#
#   3ºLEVEL
#

# 3ºLEVEL -> EXPRESSION
@dataclass
class IRExpression(Sequence):
    code = b"\x05"
    adr: bytes
    amount: bytes
    terms: bytes
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.amount + self.terms


#
#   4ºLEVEL
#

# 4ºLEVEL -> TERM
@dataclass
class IRTerm(Sequence):
    code = b"\x06"
    adr: bytes
    amount: bytes
    factors: bytes
    operators: bytes
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.amount + self.factors + self.operators


#
#   5ºLEVEL
#

# 5ºLEVEL -> VARIABLE
@dataclass
class IRVariable(Sequence):
    code = b"\x07"
    adr: bytes
    signs: bytes
    varLength: bytes
    varChars: bytes
    exponent: bytes
    exponentAdr: bytes
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.signs + self.varLength + self.varChars + self.exponent + self.exponentAdr

# 5ºLEVEL -> NEST
@dataclass
class IRNest(Sequence):
    code = b"\x08"
    adr: bytes
    signs: bytes
    pointer: bytes
    exponent: bytes
    exponentAdr: bytes
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.signs + self.pointer + self.exponent + self.exponentAdr

# 5ºLEVEL -> VECTOR
@dataclass
class IRVector(Sequence):
    code = b"\x09"
    adr: bytes
    signs: bytes
    variableAmount: bytes
    variablesAdr: bytes
    exponent: bytes 
    exponentAdr: bytes
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.signs + self.variableAmount + self.variablesAdr + self.exponent + self.exponentAdr

# 5ºLEVEL -> NUMBER
@dataclass
class IRNumber(Sequence):
    code = b"\x0A"
    adr: bytes
    signs: bytes
    length: bytes
    value: bytes
    exponent: bytes
    exponentAdr: bytes
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.signs + self.length + self.value + self.exponent + self.exponentAdr



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
    # IR -> RUN
    def run(self, sheet: Sheet) -> bytes:
        self.ir = []
        self.counter = 0
        self.sheet(sheet)
        representation = b""
        for node in [bytes(node) for node in self.ir]:
            representation += node
        return representation
    # IR -> 1 SHEET GENERATION
    def sheet(self, sheet: Sheet) -> bytes:
        variables = b""
        for statement in sheet.statements:
            match statement:
                case Declaration(): variables += self.declaration(statement)
                case Node(): variables += self.node(statement)
                case Equation(): variables += self.equation(statement)
                case Comment(): variables += self.comment(statement)
        register = self.new()
        self.ir.append(IRSheet(
            register,
            bytes([len(variables) & 0xFF]),
            variables
        ))
        return register
    # GENERATOR -> 2 DECLARATION GENERATION
    def declaration(self, declaration: Declaration) -> bytes:
        variable = self.expression(declaration.expression)
        register = self.new()
        self.ir.append(IRDeclaration(
            register,
            bytes([len(declaration.identifier) & 0xFF]),
            declaration.identifier.encode(),
            variable
        ))
        return register
    # GENERATOR -> 2 NODE GENERATION
    def node(self, node: Node) -> bytes:
        variable = self.expression(node.expression)
        register = self.new()
        self.ir.append(IRNode(
            register,
            variable
        ))
        return register
    # GENERATOR -> 2 EQUATION GENERATION
    def equation(self, equation: Equation) -> bytes:
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
    def comment(self, comment: Comment) -> bytes:
        register = self.new()
        self.ir.append(IRComment(
            register,
            bytes([len(comment.text) & 0xFF]),
            comment.text.encode()
        ))
        return register
    # GENERATOR -> 3 EXPRESSION GENERATION
    def expression(self, expression: Expression) -> bytes:
        variables = b""
        for term in expression.terms:
            variables += self.term(term)
        register = self.new()
        self.ir.append(IRExpression(
            register,
            bytes([len(variables) & 0xFF]),
            variables
        ))
        return register
    # GENERATOR -> 4 TERM GENERATION
    def term(self, term: Term) -> bytes:
        variables = b""
        operators = b""
        for factor in term.factors:
            match factor:
                case Variable(): variables += self.variable(factor)
                case Nest(): variables += self.nest(factor)
                case Vector(): variables += self.vector(factor)
                case Number(): variables += self.number(factor)
        for operator in term.operators:
            match operator:
                case "·" | "*": operators += b"\x00"
                case "/": operators += b"\x01"
        register = self.new()
        self.ir.append(IRTerm(
            register,
            bytes([len(variables) & 0xFF]),
            variables,
            operators
        ))
        return register
    # GENERATOR -> 5 VARIABLE GENERATION
    def variable(self, variable: Variable) -> bytes:
        register = self.new()
        self.ir.append(IRVariable(
            register,
            bytes([(variable.signs.count("-") if variable.signs is not None else 0x00) & 0xFF]),
            bytes([len(variable.representation) & 0xFF]),
            variable.representation.encode(),
            b"\x01" if variable.exponent is not None else b"\x00",
            self.expression(variable.exponent) if variable.exponent is not None else b""
        ))
        return register
    # GENERATOR -> 5 NEST GENERATION
    def nest(self, nest: Nest) -> bytes:
        register = self.new()
        self.ir.append(IRNest(
            register,
            bytes([(nest.signs.count("-") if nest.signs is not None else 0x00) & 0xFF]),
            self.expression(nest.expression),
            b"\x01" if nest.exponent is not None else b"\x00",
            self.expression(nest.exponent) if nest.exponent is not None else b""
        ))
        return register
    # GENERATOR -> 5 VECTOR GENERATION
    def vector(self, vector: Vector) -> bytes:
        variables = b""
        for expression in vector.values:
            variables += self.expression(expression)
        register = self.new()
        self.ir.append(IRVector(
            register,
            bytes([(vector.signs.count("-") if vector.signs is not None else 0x00) & 0xFF]),
            bytes(len(vector.values) & 0xFF),
            variables,
            b"\x01" if vector.exponent is not None else b"\x00",
            self.expression(vector.exponent) if vector.exponent is not None else b""
        ))
        return register
    # GENERATOR -> 5 NUMBER GENERATION
    def number(self, number: Number) -> bytes:
        register = self.new()
        self.ir.append(IRNumber(
            register,
            bytes([(number.signs.count("-") if number.signs is not None else 0x00) & 0xFF]),
            bytes([len(number.representation) & 0xFF]),
            number.representation.encode(),
            b"\x01" if number.exponent is not None else b"\x00",
            self.expression(number.exponent) if number.exponent is not None else b""
        ))
        return register
    # GENERATOR -> VARIABLE GENERATOR
    def new(self) -> bytes:
        address = bytes([self.counter & 0xFF])
        self.counter += 1
        return address