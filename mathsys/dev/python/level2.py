#^
#^  HEAD
#^

#> HEAD -> MODULES
from dataclasses import dataclass
from abc import ABC
from typing import cast, TYPE_CHECKING

#> HEAD -> DATA
from .tokenizer import SIGN
from .nonterminal import NonTerminal
from .ir import Opcode, Binary, Pointer, Vec, node, Sign


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
            if index == 0: signs.append([cast(SIGN, items[index]).value == "+"])
            if isinstance(items[index - 1], SIGN): signs[-1].append(cast(SIGN, items[index]).value == "+")
            else: signs.append([cast(SIGN, items[index]).value == "+"])
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

#> 2ºLEVEL -> TYPES
from .level3 import Level3