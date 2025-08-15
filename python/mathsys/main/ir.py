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
    adr: bytes[1]
    statementCount: bytes[1]
    statements: bytes[statementCount]
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.statementCount + self.statements


#
#   2ºLEVEL
#

# 2ºLEVEL -> DECLARATION
@dataclass
class IRDeclaration(Sequence):
    code = b"\x01"
    adr: bytes[1]
    charAmount: bytes[1]
    chars: bytes[charAmount]
    pointer: bytes[1]
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.charAmount + self.chars + self.pointer

# 2ºLEVEL -> NODE
@dataclass
class IRNode(Sequence):
    code = b"\x02"
    adr: bytes[1]
    pointer: bytes[1]
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.pointer

# 2ºLEVEL -> EQUATION
@dataclass
class IREquation(Sequence):
    code = b"\x03"
    adr: bytes[1]
    left: bytes[1]
    right: bytes[1]
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.left + self.right

# 2ºLEVEL -> COMMENT
@dataclass
class IRComment(Sequence):
    code = b"\x04"
    adr: bytes[1]
    amount: bytes[1]
    chars: bytes[amount]
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.left + self.right


#
#   3ºLEVEL
#

# 3ºLEVEL -> EXPRESSION
@dataclass
class IRExpression(Sequence):
    code = b"\x05"
    adr: bytes[1]
    amount: bytes[1]
    terms: bytes[amount]
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.amount + self.terms


#
#   4ºLEVEL
#

# 4ºLEVEL -> TERM
@dataclass
class IRTerm(Sequence):
    code = b"\x06"
    adr: bytes[1]
    amount: bytes[1]
    factors: bytes[amount]
    operators: bytes[amount - 1]
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.amount + self.factors + self.operators


#
#   5ºLEVEL
#

# 5ºLEVEL -> VARIABLE
@dataclass
class IRVariable(Sequence):
    code = b"\x07"
    adr: bytes[1]
    signs: bytes[1]
    varLength: bytes[1]
    varChars: bytes[varLength]
    exponent: b"\x00" | b"\x01"
    exponentAdr: bytes[exponent]
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.signs + self.varLength + self.varChars + self.exponent + self.exponentAdr

# 5ºLEVEL -> NEST
@dataclass
class IRNest(Sequence):
    code = b"\x08"
    adr: bytes[1]
    signs: bytes[1]
    pointer: bytes[1]
    exponent: b"\x00" | b"\x01"
    exponentAdr: bytes[exponent]
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.signs + self.pointer + self.exponent + self.exponentAdr

# 5ºLEVEL -> VECTOR
@dataclass
class IRVector(Sequence):
    code = b"\x09"
    adr: bytes[1]
    signs: bytes[1]
    variableAmount: bytes[1]
    variablesAdr: bytes[variableAmount]
    exponent: b"\x00" | b"\x01"
    exponentAdr: bytes[exponent]
    def __bytes__(self) -> bytes:
        return self.code + self.adr + self.signs + self.variableAmount + self.variablesAdr + self.exponent + self.exponentAdr

# 5ºLEVEL -> NUMBER
@dataclass
class IRNumber(Sequence):
    code = b"\x0A"
    adr: bytes[1]
    signs: bytes[1]
    length: bytes[1]
    value: bytes[length]
    exponent: b"\x00" | b"\x01"
    exponentAdr: bytes[exponent]
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
        return sum([bytes(node) for node in self.ir])
    # IR -> 1 SHEET GENERATION
    def sheet(self, sheet: Sheet) -> bytes:
        variables = b""
        for statement in sheet.statements:
            match statement:
                case Declaration(): variables.append(self.declaration(statement))
                case Node(): variables.append(self.node(statement))
                case Equation(): variables.append(self.equation(statement))
                case Comment(): variables.append(self.comment(statement))
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
        left = self.expression(equation.leftSide)
        right = self.expression(equation.rightSide)
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
            variables.append(self.term(term))
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
            variables.append(self.factor(factor))
        for operator in term.operators:
            match operator:
                case "·" | "*": operators.append(b"\x00")
                case "/": operators.append(b"\x01")
        register = self.new()
        self.ir.append(IRTerm(
            register,
            bytes([len(variables) & 0xFF]),
            variables,
            operators
        ))
        return register
    # GENERATOR -> 5 FACTOR GENERATION
    def factor(self, factor: Factor) -> str:
        match factor.value:
            case str(): variable = factor.value
            case Expression(): variable = self.expression(factor.value)
            case Vector(): variable = self.vector(factor.value)
        exponent = [self.expression(factor.exponent)] if factor.exponent is not None else []
        register = self.new()
        self.ir.append(IRNode(
            "factor",
            [
                register,
                *factor.type,
                *([factor.signs] if factor.signs is not None else []),
                variable,
                *(exponent)
            ]
        ))
        return register
    # GENERATOR -> 6 VECTOR GENERATION
    def vector(self, vector: Vector) -> str:
        variables = []
        for expression in vector.expressions:
            variables.append(self.expression(expression))
        register = self.new()
        self.ir.append(IRNode(
            "vector",
            [
                register,
                *variables
            ]
        ))
        return register
    # GENERATOR -> VARIABLE GENERATOR
    def new(self) -> bytes:
        address = bytes([self.counter & 0xFF])
        self.counter += 1
        return address