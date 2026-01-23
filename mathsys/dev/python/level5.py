#^
#^  HEAD
#^

#> HEAD -> MODULES
from abc import ABC
from dataclasses import dataclass
from typing import cast

#> HEAD -> DATA
from .nonterminal import NonTerminal
from .tokenizer import IDENTIFIER, NUMBER, RATIONAL, TYPE
from .ir import Opcode, node, Binary, Pointer, String, Option, Vec, BigUint, Group
from .latex import VARIABLES, CONVERSION


#^
#^  5ºLEVEL
#^

#> 5ºLEVEL -> NAMESPACE
class Level5(NonTerminal, ABC): pass

#> 5ºLEVEL -> INFINITE
@dataclass(init = False, unsafe_hash = True)
class Infinite(Level5):
    code = Opcode(0x0C).binary()
    def __init__(self, items: list) -> None: self.freeze()
    def latex(self, types: dict[str, str]) -> str: return r"\infty "
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        return node(self.code, binary, nodes)

#> 5ºLEVEL -> VARIABLE
@dataclass(init = False, unsafe_hash = True)
class Variable(Level5):
    code = Opcode(0x0D).binary()
    representation: str
    def __init__(self, items: list[IDENTIFIER]) -> None: self.representation = items[0].value; self.freeze()
    def latex(self, types: dict[str, str]) -> str:
        curated = self.representation
        for source, replace in VARIABLES.items(): curated = curated.replace(source, replace)
        return CONVERSION[types.get(self.representation, "@Undefined")](curated)
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        representation = String(self.representation)
        return node(self.code + representation, binary, nodes)

#> 5ºLEVEL -> NEST
@dataclass(init = False, unsafe_hash = True)
class Nest(Level5):
    code = Opcode(0x0E).binary()
    value: Level2 | None
    def __init__(self, items: list[Level2]) -> None: 
        self.value = items[0] if len(items) == 1 else None
        self.freeze()
    def latex(self, types: dict[str, str]) -> str:
        inside = self.value.latex(types) if self.value is not None else ""
        return fr"\left( {inside}\right) "
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = Option(self.value.ir(binary, nodes) if self.value is not None else None)
        return node(self.code + value, binary, nodes)

#> 5ºLEVEL -> TENSOR
@dataclass(init = False, unsafe_hash = True)
class Tensor(Level5):
    code = Opcode(0x0F).binary()
    values: tuple[Level2, ...]
    def __init__(self, items: list[Level2]) -> None: self.values = tuple(items); self.freeze()
    def latex(self, types: dict[str, str]) -> str:
        inside = r"\; " if len(self.values) == 0 else r"\\ ".join(value.latex(types) for value in self.values)
        return fr"\begin{{bmatrix}}{inside}\end{{bmatrix}}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        values = Vec([value.ir(binary, nodes) for value in self.values])
        return node(self.code + values, binary, nodes)

#> 5ºLEVEL -> WHOLE
@dataclass(init = False, unsafe_hash = True)
class Whole(Level5):
    code = Opcode(0x10).binary()
    value: int
    def __init__(self, items: list[NUMBER]) -> None: self.value = int(items[0].value); self.freeze()
    def latex(self, types: dict[str, str]) -> str: return str(self.value)
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = BigUint(self.value)
        return node(self.code + value, binary, nodes)

#> 5ºLEVEL -> ABSOLUTE
@dataclass(init = False, unsafe_hash = True)
class Absolute(Level5):
    code = Opcode(0x11).binary()
    value: Level2
    def __init__(self, items: list[Level2]) -> None: self.value = items[0]; self.freeze()
    def latex(self, types: dict[str, str]) -> str: return fr"\left| {self.value.latex(types)}\right| "
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        value = self.value.ir(binary, nodes)
        return node(self.code + value, binary, nodes)

#> 5ºLEVEL -> UNDEFINED
@dataclass(init = False, unsafe_hash = True)
class Undefined(Level5):
    code = Opcode(0x12).binary()
    def __init__(self, items: list) -> None: self.freeze()
    def latex(self, types: dict[str, str]) -> str: return r"\left. ?\right. "
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer: return node(self.code, binary, nodes)

#> 5ºLEVEL -> RATIONAL
@dataclass(init = False, unsafe_hash = True)
class Rational(Level5):
    code = Opcode(0x13).binary()
    whole: int
    decimal: int
    def __init__(self, items: list[RATIONAL]) -> None:
        self.whole = int(items[0].value.split(".", 1)[0])
        self.decimal = int(items[0].value.split(".", 1)[1].rstrip("0")[::-1])
        self.freeze()
    def latex(self, types: dict[str, str]) -> str: return f"{self.whole}.{str(self.decimal)[::-1]}"
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        whole = BigUint(self.whole)
        decimal = BigUint(self.decimal)
        return node(self.code + whole + decimal, binary, nodes)

#> 5ºLEVEL -> CASTS
@dataclass(init = False, unsafe_hash = True)
class Casts(Level5):
    code = Opcode(0x14).binary()
    element: Level5
    to: str
    def __init__(self, items: list[Level5 | TYPE]) -> None:
        self.element = cast(Level5, items[0])
        self.to = cast(TYPE, items[1]).value
        self.freeze()
    def latex(self, types: dict[str, str]) -> str: return self.element.latex(types)
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer:
        element = self.element.ir(binary, nodes)
        to = Group(self.to)
        return node(self.code + element + to, binary, nodes)

#> 5ºLEVEL -> TYPES
from .level2 import Level2