#^
#^  HEAD
#^

#> HEAD -> MODULES
from __future__ import annotations
from dataclasses import dataclass
from abc import ABC, abstractmethod

#> HEAD -> DATA
from .latex import SPECIAL, VARIABLES, CONVERSION
from .ir import Binary, Opcode, Pointer, Sign, Option, BigUint, String, Group, Vec, node


#^
#^  START
#^

#> START -> CLASS
@dataclass(kw_only = True, frozen = True)
class Start:
    code = Opcode(0x01).binary()
    stream: list[Level1 | str]
    def latex(self, types: dict = {}) -> str:
        match len(self.stream):
            case 0: delimiters = ["", ""]
            case 1: delimiters = [r"\(", r"\)"]
            case other: delimiters = [r"\[", r"\]"]
        values = r"\\ ".join(f"\text{{{"".join(SPECIAL.get(character, character) for character in element)}}}" if isinstance(element, str) else element.latex(types) for element in self.stream)
        while values.startswith(r"\\"): values = values[2:]
        return f"{delimiters[0]}{values}{delimiters[1]}"
    def ir(self, binary: list[Binary], nodes: list[Binary] = []) -> Pointer:
        pointers = Vec([element.ir(binary, nodes) for element in self.stream if isinstance(element, Level1)])
        return node(self.code + pointers, binary, nodes)


#^
#^  1ºLEVEL
#^

#> 1ºLEVEL -> NAMESPACE
class Level1(ABC):
    @abstractmethod
    def latex(self, types: dict) -> str: pass
    @abstractmethod
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer: pass

#> 1ºLEVEL -> DECLARATION
@dataclass(kw_only = True, frozen = True)
class Declaration(Level1):
    code = Opcode(0x02).binary()
    group: str | None
    variable: Variable
    value: Level2
    def latex(self, types: dict) -> str:
        types[self.variable.representation] = types.get(self.variable.representation, self.group if self.group is not None else "@Undefined")
        return f"{self.variable.latex(types)}={self.value.latex(types)}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        group = Option(Group(self.group) if self.group is not None else None)
        variable = self.variable.ir(binary, nodes)
        value = self.value.ir(binary, nodes)
        return node(self.code + group + variable + value, binary, nodes)

#> 1ºLEVEL -> DEFINITION
@dataclass(kw_only = True, frozen = True)
class Definition(Level1):
    code = Opcode(0x03).binary()
    group: str | None
    variable: Variable
    value: Level2
    def latex(self, types: dict) -> str:
        types[self.variable.representation] = types.get(self.variable.representation, self.group if self.group is not None else "@Undefined")
        return fr"{self.variable.latex(types)}\equiv {self.value.latex(types)}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        group = Option(Group(self.group) if self.group is not None else None)
        variable = self.variable.ir(binary, nodes)
        value = self.value.ir(binary, nodes)
        return node(self.code + group + variable + value, binary, nodes)

#> 1ºLEVEL -> ANNOTATION
@dataclass(kw_only = True, frozen = True)
class Annotation(Level1):
    code = Opcode(0x04).binary()
    group: str
    variables: list[Variable]
    def latex(self, types: dict) -> str:
        for variable in self.variables: types[variable.representation] = types.get(variable.representation, self.group)
        variables = ",".join(variable.latex(types) for variable in self.variables)
        return fr"\text{{{self.group} }}{variables}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        group = Group(self.group)
        variables = Vec([variable.ir(binary, nodes) for variable in self.variables])
        return node(self.code + group + variables, binary, nodes)

#> 1ºLEVEL -> NODE
@dataclass(kw_only = True, frozen = True)
class Node(Level1):
    code = Opcode(0x05).binary()
    value: Level2
    def latex(self, types: dict) -> str: return self.value.latex(types)
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = self.value.ir(binary, nodes)
        return node(self.code + value, binary, nodes)

#> 1ºLEVEL -> EQUATION
@dataclass(kw_only = True, frozen = True)
class Equation(Level1):
    code = Opcode(0x06).binary()
    leftside: Level2
    rightside: Level2
    def latex(self, types: dict) -> str: return f"{self.leftside.latex(types)}={self.rightside.latex(types)}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        leftside = self.leftside.ir(binary, nodes)
        rightside = self.rightside.ir(binary, nodes)
        return node(self.code + leftside + rightside, binary, nodes)

#> 1ºLEVEL -> USE
@dataclass(kw_only = True, frozen = True)
class Use(Level1):
    code = Opcode(0x07).binary()
    name: str
    start: Start | None
    def latex(self, types: dict) -> str:
        if self.start is not None: self.start.latex(types)
        delimiters = ["", ""] if self.start is not None else [r"\color{brown}", r"\color{black}"]
        return fr"{delimiters[0]}\text{{use {self.name}}}{delimiters[1]}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        name = String(self.name)
        start = Option(self.start.ir(binary, nodes) if self.start is not None else None)
        return node(self.code + name + start, binary, nodes)


#^
#^  2ºLEVEL
#^

#> 2ºLEVEL -> NAMESPACE
class Level2(ABC):
    @abstractmethod
    def latex(self, types: dict) -> str: pass
    @abstractmethod
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer: pass

#> 2ºLEVEL -> EXPRESSION
@dataclass(kw_only = True, frozen = True)
class Expression(Level2):
    code = Opcode(0x08).binary()
    signs: list[list[bool]]
    terms: list[Level3]
    def latex(self, types: dict) -> str:
        string = []
        for index in range(len(self.terms)):
            signs = "".join("+" if sign else "-" for sign in self.signs[index])
            string.append(f"{signs}{self.terms[index].latex(types)}")
        return "".join(string)
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        signs = Vec([Vec([Sign(sign) for sign in signlist]) for signlist in self.signs])
        terms = Vec([term.ir(binary, nodes) for term in self.terms])
        return node(self.code + signs + terms, binary, nodes)


#^
#^  3ºLEVEL
#^

#> 3ºLEVEL -> NAMESPACE
class Level3(ABC):
    @abstractmethod
    def latex(self, types: dict) -> str: pass
    @abstractmethod
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer: pass

#> 3ºLEVEL -> TERM
@dataclass(kw_only = True, frozen = True)
class Term(Level3):
    code = Opcode(0x09).binary()
    numerator: list[Level4]
    denominator: list[Level4]
    def latex(self, types: dict) -> str:
        numerator = r"\cdot ".join(value.latex(types) for value in self.numerator)
        denominator = r"\cdot ".join(value.latex(types) for value in self.denominator)
        return fr"\frac{{{numerator}}}{{{denominator}}}" if len(self.denominator) != 0 else numerator
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        numerator = Vec([factor.ir(binary, nodes) for factor in self.numerator])
        denominator = Vec([factor.ir(binary, nodes) for factor in self.denominator])
        return node(self.code + numerator + denominator, binary, nodes)


#^
#^  4ºLEVEL
#^

#> 4ºLEVEL -> NAMESPACE
class Level4(ABC):
    @abstractmethod
    def latex(self, types: dict) -> str: pass
    @abstractmethod
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer: pass

#> 4ºLEVEL -> FACTOR
@dataclass(kw_only = True, frozen = True)
class Factor(Level4):
    code = Opcode(0x0A).binary()
    value: Level5
    exponent: Level2 | None
    def latex(self, types: dict) -> str:
        exponent = f"^{{{self.exponent.latex(types)}}}" if self.exponent is not None else ""
        return f"{self.value.latex(types)}{exponent}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = self.value.ir(binary, nodes)
        exponent = Option(self.exponent.ir(binary, nodes) if self.exponent is not None else None)
        return node(self.code + value + exponent, binary, nodes)

#> 4ºLEVEL -> LIMIT
@dataclass(kw_only = True, frozen = True)
class Limit(Level4):
    code = Opcode(0x0B).binary()
    variable: Variable
    approach: Level2
    direction: bool | None
    nest: Nest
    exponent: Level2 | None
    def latex(self, types: dict) -> str:
        direction = f"^{{{"+" if self.direction else "-"}}}" if self.direction is not None else ""
        exponent = f"^{{{self.exponent.latex(types)}}}" if self.exponent is not None else ""
        return fr"\lim_{{\substack{{{self.variable.latex(types)}\to {self.approach.latex(types)}{direction}}}}}{self.nest.latex(types)}{exponent}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        variable = self.variable.ir(binary, nodes)
        approach = self.approach.ir(binary, nodes)
        sign = Option(Sign(self.direction) if self.direction is not None else None)
        nest = self.nest.ir(binary, nodes)
        exponent = Option(self.exponent.ir(binary, nodes) if self.exponent is not None else None)
        return node(self.code + variable + approach + sign + nest + exponent, binary, nodes)


#^
#^  5ºLEVEL
#^

#> 5ºLEVEL -> NAMESPACE
class Level5(ABC):
    @abstractmethod
    def latex(self, types: dict) -> str: pass
    @abstractmethod
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer: pass

#> 5ºLEVEL -> INFINITE
@dataclass(kw_only = True, frozen = True)
class Infinite(Level5):
    code = Opcode(0x0C).binary()
    def latex(self, types: dict) -> str: return r"\infty "
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        return node(self.code, binary, nodes)

#> 5ºLEVEL -> VARIABLE
@dataclass(kw_only = True, frozen = True)
class Variable(Level5):
    code = Opcode(0x0D).binary()
    representation: str
    def latex(self, types: dict) -> str:
        curated = self.representation
        for source, replace in VARIABLES.items(): curated = curated.replace(source, replace)
        return CONVERSION[types.get(self.representation, "Undefined")](curated)
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        representation = String(self.representation)
        return node(self.code + representation, binary, nodes)

#> 5ºLEVEL -> NEST
@dataclass(kw_only = True, frozen = True)
class Nest(Level5):
    code = Opcode(0x0E).binary()
    value: Level2 | None
    def latex(self, types: dict) -> str:
        inside = self.value.latex(types) if self.value is not None else ""
        return fr"\left( {inside}\right) "
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = Option(self.value.ir(binary, nodes) if self.value is not None else None)
        return node(self.code + value, binary, nodes)

#> 5ºLEVEL -> TENSOR
@dataclass(kw_only = True, frozen = True)
class Tensor(Level5):
    code = Opcode(0x0F).binary()
    values: list[Level2]
    def latex(self, types: dict) -> str:
        inside = r"\; " if len(self.values) == 0 else r"\\ ".join(value.latex(types) for value in self.values)
        return fr"\begin{{bmatrix}}{inside}\end{{bmatrix}}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        values = Vec([value.ir(binary, nodes) for value in self.values])
        return node(self.code + values, binary, nodes)

#> 5ºLEVEL -> WHOLE
@dataclass(kw_only = True, frozen = True)
class Whole(Level5):
    code = Opcode(0x10).binary()
    value: int
    def latex(self, types: dict) -> str: return str(self.value)
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = BigUint(self.value)
        return node(self.code + value, binary, nodes)

#> 5ºLEVEL -> ABSOLUTE
@dataclass(kw_only = True, frozen = True)
class Absolute(Level5):
    code = Opcode(0x11).binary()
    value: Level2
    def latex(self, types: dict) -> str: return fr"\left| {self.value.latex(types)}\right| "
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = self.value.ir(binary, nodes)
        return node(self.code + value, binary, nodes)

#> 5ºLEVEL -> UNDEFINED
@dataclass(kw_only = True, frozen = True)
class Undefined(Level5):
    code = Opcode(0x12).binary()
    def latex(self, types: dict) -> str: return r"\left. ?\right. "
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer: return node(self.code, binary, nodes)

#> 5ºLEVEL -> RATIONAL
@dataclass(kw_only = True, frozen = True)
class Rational(Level5):
    code = Opcode(0x13).binary()
    whole: int
    decimal: int
    def latex(self, types: dict) -> str: return f"{self.whole}.{str(self.decimal)[::-1]}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        whole = BigUint(self.whole)
        decimal = BigUint(self.decimal)
        return node(self.code + whole + decimal, binary, nodes)

#> 5ºLEVEL -> CASTS
@dataclass(kw_only = True, frozen = True)
class Casts(Level5):
    code = Opcode(0x14).binary()
    element: Level5
    to: str
    def latex(self, types: dict) -> str: return self.element.latex(types)
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        element = self.element.ir(binary, nodes)
        to = Group(self.to)
        return node(self.code + element + to, binary, nodes)