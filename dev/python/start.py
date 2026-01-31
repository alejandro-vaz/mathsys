#^
#^  HEAD
#^

#> HEAD -> MODULES
from dataclasses import dataclass

#> HEAD -> DATA
from .nonterminal import NonTerminal
from .ir import Opcode, Binary, Pointer, Vec, node


#^
#^  START
#^

#> START -> CLASS
@dataclass(init = False, unsafe_hash = True)
class Start(NonTerminal):
    code = Opcode(0x01).binary()
    stream: tuple[Level1, ...]
    def __init__(self, items: list[Level1]) -> None: 
        self.stream = tuple(items)
        self.freeze()
    def latex(self, types: dict[str, str] = {}) -> str:
        match len(self.stream):
            case 0: delimiters = ["", ""]
            case 1: delimiters = [r"\(", r"\)"]
            case other: delimiters = [r"\[", r"\]"]
        values = r"\\ ".join(element.latex(types) for element in self.stream)
        while values.startswith(r"\\"): values = values[2:]
        return f"{delimiters[0]}{values}{delimiters[1]}"
    def ir(self, binary: list[Binary], nodes: list[Binary] = []) -> Pointer:
        pointers = Vec([element.ir(binary, nodes) for element in self.stream])
        return node(self.code + pointers, binary, nodes)

#> START -> TYPES
from .level1 import Level1