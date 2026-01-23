#^
#^  HEAD
#^

#> HEAD -> MODULES
from dataclasses import dataclass
from abc import ABC

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
class Expression(Level2):
    code = Opcode(0x08).binary()
    signs: tuple[tuple[bool, ...], ...]
    terms: tuple[Level3, ...]
    def __init__(self, items: list[SIGN | Level3]) -> None:
        signs = []
        terms = []
        current = []
        for item in items:
            if isinstance(item, SIGN): current.append(item.value == "+")
            else:
                signs.append(current)
                terms.append(item)
                current = []
        self.signs = tuple(tuple(group) for group in signs)
        self.terms = tuple(terms)
        self.freeze()
    def latex(self, types: dict[str, str]) -> str:
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