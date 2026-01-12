#^
#^  HEAD
#^

#> HEAD -> MODULES
from abc import ABC
from typing import cast
from dataclasses import dataclass

#> HEAD -> DATA
from .ir import Opcode, Binary, Option, node, Pointer, Sign
from .tokenizer import SIGN
from .nonterminal import NonTerminal


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

#> 4ºLEVEL -> TYPES
from .level2 import Level2
from .level5 import Level5, Variable, Nest