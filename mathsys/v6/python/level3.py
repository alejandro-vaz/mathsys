#^
#^  HEAD
#^

#> HEAD -> MODULES
from abc import ABC
from dataclasses import dataclass

#> HEAD -> DATA
from .nonterminal import NonTerminal
from .ir import Opcode, Binary, Vec, node, Pointer
from .tokenizer import OPERATOR



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

#> 3ºLEVEL -> TYPES
from .level4 import Level4