#^
#^  HEAD
#^

#> HEAD -> MODULES
from dataclasses import dataclass

#> HEAD -> DATA
from .nonterminal import NonTerminal
from .ir import Opcode, Binary, Pointer, Vec, node
from .latex import SPECIAL


#^
#^  START
#^

#> START -> CLASS
@dataclass(init = False, unsafe_hash = True)
class Start(NonTerminal):
    code = Opcode(0x01).binary()
    stream: tuple[Level1, ...]
    def create(self, items: list[Level1]) -> None: self.stream = tuple(items) 
    def latex(self, types: dict[str, str] = {}) -> str:
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

#> START -> TYPES
from .level1 import Level1