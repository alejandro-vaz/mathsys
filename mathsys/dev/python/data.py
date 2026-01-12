#^
#^  HEAD
#^

#> HEAD -> MODULES
from __future__ import annotations
from dataclasses import dataclass, FrozenInstanceError
from abc import ABC, abstractmethod
from typing import Any, cast

#> HEAD -> DATA
from .tokenizer import TYPE, MODULE, SIGN, Token, OPERATOR, IDENTIFIER, NUMBER, RATIONAL
from .latex import SPECIAL, VARIABLES, CONVERSION
from .ir import Binary, Opcode, Pointer, Sign, Option, BigUint, String, Group, Vec, node



#^
#^  NONTERMINAL
#^

#> NONTERMINAL -> START
class NonTerminal(ABC):
    frozen: bool = False
    @abstractmethod
    def __init__(self, items: list) -> None: pass
    @abstractmethod
    def create(self, items: list) -> None: pass
    @abstractmethod
    def latex(self, types: dict) -> str: pass
    @abstractmethod
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer: pass
    def __setattr__(self, name: str, value: Any) -> None:
        if self.frozen: raise FrozenInstanceError()
        return super().__setattr__(name, value)
    def freeze(self) -> None: self.frozen = True
    def __init_subclass__(cls) -> None:
        super().__init_subclass__()
        def replacement(self, items: list) -> None: cls.create(self, items); self.freeze()
        cls.__init__ = replacement


#^
#^  START
#^

#> START -> CLASS
@dataclass(init = False, unsafe_hash = True)
class Start(NonTerminal):
    code = Opcode(0x01).binary()
    stream: tuple[Level1, ...]
    def create(self, items: list[Level1]) -> None: self.stream = tuple(items) 
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
class Level1(NonTerminal, ABC): pass

#> 1ºLEVEL -> DECLARATION
@dataclass(init = False, unsafe_hash = True)
class Declaration(Level1, NonTerminal):
    code = Opcode(0x02).binary()
    group: str | None
    variable: Variable
    value: Level2
    def create(self, items: list[TYPE | Variable | Level2]) -> None:
        self.group = cast(TYPE, items[0]).value if len(items) == 3 else None
        self.variable = cast(Variable, items[1] if len(items) == 3 else items[0])
        self.value = cast(Level2, items[2] if len(items) == 3 else items[1])
    def latex(self, types: dict) -> str:
        types[self.variable.representation] = types.get(self.variable.representation, self.group if self.group is not None else "@Undefined")
        return f"{self.variable.latex(types)}={self.value.latex(types)}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        group = Option(Group(self.group) if self.group is not None else None)
        variable = self.variable.ir(binary, nodes)
        value = self.value.ir(binary, nodes)
        return node(self.code + group + variable + value, binary, nodes)

#> 1ºLEVEL -> DEFINITION
@dataclass(init = False, unsafe_hash = True)
class Definition(Level1, NonTerminal):
    code = Opcode(0x03).binary()
    group: str | None
    variable: Variable
    value: Level2
    def create(self, items: list[TYPE | Variable | Level2]) -> None:
        self.group = cast(TYPE, items[0]).value if len(items) == 3 else None
        self.variable = cast(Variable, items[1] if len(items) == 3 else items[0])
        self.value = cast(Level2, items[2] if len(items) == 3 else items[1])
    def latex(self, types: dict) -> str:
        types[self.variable.representation] = types.get(self.variable.representation, self.group if self.group is not None else "@Undefined")
        return fr"{self.variable.latex(types)}\equiv {self.value.latex(types)}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        group = Option(Group(self.group) if self.group is not None else None)
        variable = self.variable.ir(binary, nodes)
        value = self.value.ir(binary, nodes)
        return node(self.code + group + variable + value, binary, nodes)

#> 1ºLEVEL -> ANNOTATION
@dataclass(init = False, unsafe_hash = True)
class Annotation(Level1, NonTerminal):
    code = Opcode(0x04).binary()
    group: str
    variables: tuple[Variable]
    def create(self, items: list[TYPE | Variable]) -> None:
        self.group = cast(TYPE, items[0]).value
        self.variables = cast(tuple[Variable], tuple(items[1:]))
    def latex(self, types: dict) -> str:
        for variable in self.variables: types[variable.representation] = types.get(variable.representation, self.group)
        variables = ",".join(variable.latex(types) for variable in self.variables)
        return fr"\text{{{self.group} }}{variables}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        group = Group(self.group)
        variables = Vec([variable.ir(binary, nodes) for variable in self.variables])
        return node(self.code + group + variables, binary, nodes)

#> 1ºLEVEL -> NODE
@dataclass(init = False, unsafe_hash = True)
class Node(Level1, NonTerminal):
    code = Opcode(0x05).binary()
    value: Level2
    def create(self, items: list[Level2]) -> None:
        self.value = items[0]
    def latex(self, types: dict) -> str: return self.value.latex(types)
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = self.value.ir(binary, nodes)
        return node(self.code + value, binary, nodes)

#> 1ºLEVEL -> EQUATION
@dataclass(init = False, unsafe_hash = True)
class Equation(Level1, NonTerminal):
    code = Opcode(0x06).binary()
    leftside: Level2
    rightside: Level2
    def create(self, items: list[Level2]) -> None:
        self.leftside = items[0]
        self.rightside = items[1]
    def latex(self, types: dict) -> str: return f"{self.leftside.latex(types)}={self.rightside.latex(types)}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        leftside = self.leftside.ir(binary, nodes)
        rightside = self.rightside.ir(binary, nodes)
        return node(self.code + leftside + rightside, binary, nodes)

#> 1ºLEVEL -> USE
@dataclass(init = False, unsafe_hash = True)
class Use(Level1, NonTerminal):
    code = Opcode(0x07).binary()
    name: str
    start: Start | None
    def create(self, items: list[MODULE]) -> None:
        self.name = items[0].value
        self.start = None
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
class Level2(NonTerminal, ABC): pass

#> 2ºLEVEL -> EXPRESSION
@dataclass(init = False, unsafe_hash = True)
class Expression(Level2, NonTerminal):
    code = Opcode(0x08).binary()
    signs: tuple[tuple[bool, ...], ...]
    terms: tuple[Level3, ...]
    def create(self, items: list[SIGN | Level3]) -> None:
        signs = []
        for index in range(len(items)):
            if isinstance(items[index], Level3):
                if index == 0: signs.append([])
                continue
            if index == 0: signs.append([cast(Token, items[index]).value == "+"])
            if isinstance(items[index - 1], Token): signs[-1].append(cast(Token, items[index]).value == "+")
            else: signs.append([cast(Token, items[index]).value == "+"])
        self.signs = tuple([tuple(nested) for nested in signs])
        self.terms = tuple([item for item in items if isinstance(item, Level3)])
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
class Level3(NonTerminal, ABC): pass

#> 3ºLEVEL -> TERM
@dataclass(init = False, unsafe_hash = True)
class Term(Level3, NonTerminal):
    code = Opcode(0x09).binary()
    numerator: tuple[Level4, ...]
    denominator: tuple[Level4, ...]
    def create(self, items: list[Level4 | OPERATOR]) -> None:
        numerator = []
        denominator = []
        location = True
        for item in items:
            if isinstance(item, OPERATOR):
                match item.value:
                    case "*": location = True
                    case "/": location = False
            else: numerator.append(item) if location else denominator.append(item)
        self.numerator = tuple(numerator)
        self.denominator = tuple(denominator)
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
class Level4(NonTerminal, ABC): pass

#> 4ºLEVEL -> FACTOR
@dataclass(init = False, unsafe_hash = True)
class Factor(Level4, NonTerminal):
    code = Opcode(0x0A).binary()
    value: Level5
    exponent: Level2 | None
    def create(self, items: list[Level5 | Level2]) -> None:
        self.value = cast(Level5, items[0])
        self.exponent = cast(Level2, items[1]) if len(items) == 2 else None
    def latex(self, types: dict) -> str:
        exponent = f"^{{{self.exponent.latex(types)}}}" if self.exponent is not None else ""
        return f"{self.value.latex(types)}{exponent}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = self.value.ir(binary, nodes)
        exponent = Option(self.exponent.ir(binary, nodes) if self.exponent is not None else None)
        return node(self.code + value + exponent, binary, nodes)

#> 4ºLEVEL -> LIMIT
@dataclass(init = False, unsafe_hash = True)
class Limit(Level4, NonTerminal):
    code = Opcode(0x0B).binary()
    variable: Variable
    approach: Level2
    direction: bool | None
    nest: Nest
    exponent: Level2 | None
    def create(self, items: list[Variable | Level2 | SIGN | Nest]) -> None:
        self.variable = cast(Variable, items[0])
        self.approach = cast(Level2, items[1])
        self.direction = items[2].value == "+" if isinstance(items[2], SIGN) else None
        self.nest = items[-2] if isinstance(items[-2], Nest) else cast(Nest, items[-1])
        self.exponent = items[-1] if isinstance(items[-1], Level2) else None
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
class Level5(NonTerminal, ABC): pass

#> 5ºLEVEL -> INFINITE
@dataclass(init = False, unsafe_hash = True)
class Infinite(Level5, NonTerminal):
    code = Opcode(0x0C).binary()
    def create(self, items: list) -> None: pass
    def latex(self, types: dict) -> str: return r"\infty "
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        return node(self.code, binary, nodes)

#> 5ºLEVEL -> VARIABLE
@dataclass(init = False, unsafe_hash = True)
class Variable(Level5, NonTerminal):
    code = Opcode(0x0D).binary()
    representation: str
    def create(self, items: list[IDENTIFIER]) -> None: self.representation = items[0].value
    def latex(self, types: dict) -> str:
        curated = self.representation
        for source, replace in VARIABLES.items(): curated = curated.replace(source, replace)
        return CONVERSION[types.get(self.representation, "@Undefined")](curated)
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        representation = String(self.representation)
        return node(self.code + representation, binary, nodes)

#> 5ºLEVEL -> NEST
@dataclass(init = False, unsafe_hash = True)
class Nest(Level5, NonTerminal):
    code = Opcode(0x0E).binary()
    value: Level2 | None
    def create(self, items: list[Level2]) -> None: self.value = items[0] if len(items) == 1 else None
    def latex(self, types: dict) -> str:
        inside = self.value.latex(types) if self.value is not None else ""
        return fr"\left( {inside}\right) "
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = Option(self.value.ir(binary, nodes) if self.value is not None else None)
        return node(self.code + value, binary, nodes)

#> 5ºLEVEL -> TENSOR
@dataclass(init = False, unsafe_hash = True)
class Tensor(Level5, NonTerminal):
    code = Opcode(0x0F).binary()
    values: tuple[Level2, ...]
    def create(self, items: list[Level2]) -> None: self.values = tuple(items)
    def latex(self, types: dict) -> str:
        inside = r"\; " if len(self.values) == 0 else r"\\ ".join(value.latex(types) for value in self.values)
        return fr"\begin{{bmatrix}}{inside}\end{{bmatrix}}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        values = Vec([value.ir(binary, nodes) for value in self.values])
        return node(self.code + values, binary, nodes)

#> 5ºLEVEL -> WHOLE
@dataclass(init = False, unsafe_hash = True)
class Whole(Level5, NonTerminal):
    code = Opcode(0x10).binary()
    value: int
    def create(self, items: list[NUMBER]) -> None: self.value = int(items[0].value)
    def latex(self, types: dict) -> str: return str(self.value)
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = BigUint(self.value)
        return node(self.code + value, binary, nodes)

#> 5ºLEVEL -> ABSOLUTE
@dataclass(init = False, unsafe_hash = True)
class Absolute(Level5, NonTerminal):
    code = Opcode(0x11).binary()
    value: Level2
    def create(self, items: list[Level2]) -> None: self.value = items[0]
    def latex(self, types: dict) -> str: return fr"\left| {self.value.latex(types)}\right| "
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = self.value.ir(binary, nodes)
        return node(self.code + value, binary, nodes)

#> 5ºLEVEL -> UNDEFINED
@dataclass(init = False, unsafe_hash = True)
class Undefined(Level5, NonTerminal):
    code = Opcode(0x12).binary()
    def create(self, items: list) -> None: pass
    def latex(self, types: dict) -> str: return r"\left. ?\right. "
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer: return node(self.code, binary, nodes)

#> 5ºLEVEL -> RATIONAL
@dataclass(init = False, unsafe_hash = True)
class Rational(Level5, NonTerminal):
    code = Opcode(0x13).binary()
    whole: int
    decimal: int
    def create(self, items: list[RATIONAL]) -> None:
        self.whole = int(items[0].value.split(".")[0])
        self.decimal = int(items[0].value.split(".")[1].rstrip("0")[::-1])
    def latex(self, types: dict) -> str: return f"{self.whole}.{str(self.decimal)[::-1]}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        whole = BigUint(self.whole)
        decimal = BigUint(self.decimal)
        return node(self.code + whole + decimal, binary, nodes)

#> 5ºLEVEL -> CASTS
@dataclass(init = False, unsafe_hash = True)
class Casts(Level5, NonTerminal):
    code = Opcode(0x14).binary()
    element: Level5
    to: str
    def create(self, items: list[Level5 | TYPE]) -> None:
        self.element = cast(Level5, items[0])
        self.to = cast(TYPE, items[1]).value
    def latex(self, types: dict) -> str: return self.element.latex(types)
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        element = self.element.ir(binary, nodes)
        to = Group(self.to)
        return node(self.code + element + to, binary, nodes)