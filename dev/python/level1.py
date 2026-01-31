#^
#^  HEAD
#^

#> HEAD -> MODULES
from dataclasses import dataclass
from abc import ABC
from typing import cast

#> HEAD -> DATA
from .tokenizer import TYPE, MODULE
from .nonterminal import NonTerminal
from .ir import Opcode, Option, Group, Pointer, Binary, node, String, Vec


#^
#^  1ºLEVEL
#^

#> 1ºLEVEL -> NAMESPACE
class Level1(NonTerminal, ABC): pass

#> 1ºLEVEL -> DECLARATION
@dataclass(init = False, unsafe_hash = True)
class Declaration(Level1):
    code = Opcode(0x02).binary()
    group: str | None
    variable: Variable
    value: Level2
    def __init__(self, items: list[TYPE | Variable | Level2]) -> None:
        self.group = cast(TYPE, items[0]).value if len(items) == 3 else None
        self.variable = cast(Variable, items[1] if len(items) == 3 else items[0])
        self.value = cast(Level2, items[2] if len(items) == 3 else items[1])
        self.freeze()
    def latex(self, types: dict[str, str]) -> str:
        types[self.variable.representation] = types.get(self.variable.representation, self.group if self.group is not None else "@Undefined")
        return f"{self.variable.latex(types)}={self.value.latex(types)}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        group = Option(Group(self.group) if self.group is not None else None)
        variable = self.variable.ir(binary, nodes)
        value = self.value.ir(binary, nodes)
        return node(self.code + group + variable + value, binary, nodes)

#> 1ºLEVEL -> DEFINITION
@dataclass(init = False, unsafe_hash = True)
class Definition(Level1):
    code = Opcode(0x03).binary()
    group: str | None
    variable: Variable
    value: Level2
    def __init__(self, items: list[TYPE | Variable | Level2]) -> None:
        self.group = cast(TYPE, items[0]).value if len(items) == 3 else None
        self.variable = cast(Variable, items[1] if len(items) == 3 else items[0])
        self.value = cast(Level2, items[2] if len(items) == 3 else items[1])
        self.freeze()
    def latex(self, types: dict[str, str]) -> str:
        types[self.variable.representation] = types.get(self.variable.representation, self.group if self.group is not None else "@Undefined")
        return fr"{self.variable.latex(types)}\equiv {self.value.latex(types)}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        group = Option(Group(self.group) if self.group is not None else None)
        variable = self.variable.ir(binary, nodes)
        value = self.value.ir(binary, nodes)
        return node(self.code + group + variable + value, binary, nodes)

#> 1ºLEVEL -> ANNOTATION
@dataclass(init = False, unsafe_hash = True)
class Annotation(Level1):
    code = Opcode(0x04).binary()
    group: str
    variables: tuple[Variable]
    def __init__(self, items: list[TYPE | Variable]) -> None:
        self.group = cast(TYPE, items[0]).value
        self.variables = cast(tuple[Variable], tuple(items[1:]))
        self.freeze()
    def latex(self, types: dict[str, str]) -> str:
        for variable in self.variables: types[variable.representation] = types.get(variable.representation, self.group)
        variables = ",".join(variable.latex(types) for variable in self.variables)
        return fr"\text{{{self.group} }}{variables}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        group = Group(self.group)
        variables = Vec([variable.ir(binary, nodes) for variable in self.variables])
        return node(self.code + group + variables, binary, nodes)

#> 1ºLEVEL -> NODE
@dataclass(init = False, unsafe_hash = True)
class Node(Level1):
    code = Opcode(0x05).binary()
    value: Level2
    def __init__(self, items: list[Level2]) -> None: self.value = items[0]; self.freeze()
    def latex(self, types: dict[str, str]) -> str: return self.value.latex(types)
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = self.value.ir(binary, nodes)
        return node(self.code + value, binary, nodes)

#> 1ºLEVEL -> EQUATION
@dataclass(init = False, unsafe_hash = True)
class Equation(Level1):
    code = Opcode(0x06).binary()
    leftside: Level2
    rightside: Level2
    def __init__(self, items: list[Level2]) -> None:
        self.leftside = items[0]
        self.rightside = items[1]
        self.freeze()
    def latex(self, types: dict[str, str]) -> str: return f"{self.leftside.latex(types)}={self.rightside.latex(types)}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        leftside = self.leftside.ir(binary, nodes)
        rightside = self.rightside.ir(binary, nodes)
        return node(self.code + leftside + rightside, binary, nodes)

#> 1ºLEVEL -> USE
@dataclass(init = False, unsafe_hash = True)
class Use(Level1):
    code = Opcode(0x07).binary()
    name: str
    start: Start | None
    def __init__(self, items: list[MODULE]) -> None:
        self.name = items[0].value
        self.start = None
        self.freeze()
    def latex(self, types: dict[str, str]) -> str:
        if self.start is not None: self.start.latex(types)
        delimiters = ["", ""] if self.start is not None else [r"\color{brown}", r"\color{black}"]
        return fr"{delimiters[0]}\text{{use {self.name}}}{delimiters[1]}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        name = String(self.name)
        start = Option(self.start.ir(binary, nodes) if self.start is not None else None)
        return node(self.code + name + start, binary, nodes)

#> 1ºLEVEL -> TYPES
from .start import Start
from .level2 import Level2
from .level5 import Variable