#
#   HEAD
#

# HEAD -> MODULES
from __future__ import annotations

# HEAD -> DATACLASSES
from .parser import Sheet, Declaration, Expression, Term, Variable, Brackets


#
#   GENERATOR
#

# GENERATOR -> CLASS
class Generator:
    # CLASS -> VARIABLES
    ir: list[str]
    counter: int
    memory: list[str]
    # CLASS -> INIT
    def __init__(self) -> None:
        self.ir = []
        self.counter = 0
        self.memory = []
    # CLASS -> RUN
    def run(self, sheet: Sheet) -> str:
        for declaration in sheet.statements:
            self.declaration(declaration)
        return "\n".join(self.ir)
    # CLASS -> DECLARATION GENERATION
    def declaration(self, declaration: Declaration) -> None:
        register = self.expression(declaration.expression)
        if not register: return None
        self.newInstruction(
            "store",
            target=register,
            name=declaration.identifier
        )
        self.memory.append(declaration.identifier)
    # CLASS -> EXPRESSION GENERATION
    def expression(self, expression: Expression) -> str | None:
        registers = []
        for value in expression.terms:
            match value:
                case Term(): registers.append(self.term(value))
                case Variable(): registers.append(self.variable(value))
                case Brackets(): registers.append(self.brackets(value))
        registers = [register for register in registers if register is not None]
        match len(registers):
            case 0: return None
            case 1: return registers[0]
            case x if x >= 2:
                while len(registers) > 1:
                    register = self.getRegister()
                    self.newInstruction(
                        "join", 
                        target=register, 
                        first=registers[0], 
                        second=registers[1]
                    )
                    registers.pop(0)
                    registers.pop(0)
                    registers = [register] + registers
                return registers[0]
    # CLASS -> TERM GENERATION
    def term(self, value: Term) -> str:
        register = self.getRegister()
        self.newInstruction(
            "num",
            target=register,
            type="Num",
            signs=value.signs,
            value=value.number
        )
        return register
    # CLASS -> VARIABLE GENERATION
    def variable(self, value: Variable) -> str | None:
        if not value.identifier in self.memory: return None
        register = self.getRegister()
        self.newInstruction(
            "load",
            target=register,
            name=value.identifier
        )
        return register
    # CLASS -> BRACKETS GENERATION
    def brackets(self, value: Brackets) -> str | None:
        register = self.expression(value.expression)
        if not register: return None
        self.newInstruction(
            "mod",
            target=register,
            method="signs",
            argument=value.signs
        )
        return register
    # CLASS -> INSTRUCTION GENERATOR
    def newInstruction(self, instruction: str, **data: str) -> None:
        match instruction:
            case "store": self.ir.append('store {target} "{name}"'.format(**data))
            case "load": self.ir.append('load {target} "{name}"'.format(**data))
            case "num": self.ir.append('num {target} "{type}" "{signs}" "{value}"'.format(**data))
            case "join": self.ir.append('join {target} {first} {second}'.format(**data))
            case "mod": self.ir.append('mod {target} "{method}" "{argument}"'.format(**data))
    # CLASS -> REGISTER NAME HELPER
    def getRegister(self) -> str:
        letters = []
        number = self.counter
        while True:
            number, remainder = divmod(number, 26)
            letters.append(chr(ord('a') + remainder))
            if number == 0:
                break
            number -= 1
        self.counter += 1
        return ''.join(reversed(letters))