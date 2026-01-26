#^
#^  HEAD
#^

#> HEAD -> MODULES
from collections import defaultdict
from typing import overload, Literal

#> HEAD -> DATA
from .tokenizer import Token
from .start import Start
from .nonterminal import NonTerminal
from .grammar import GRAMMAR


#^
#^  RESOURCES
#^

#> RESOURCES -> KEY
State = tuple[
    type[NonTerminal] | str,
    tuple[type[Token | NonTerminal] | str, ...],
    int,
    int
]

#> RESOURCES -> SPPF
SPPF = tuple[
    str | type[NonTerminal] | Token,
    int,
    int
]

#> RESOURCES -> PACKED
Packed = tuple[
    tuple[type[Token | NonTerminal] | str, ...],
    int,
    tuple[SPPF, ...]
]


#^
#^  PARSER
#^

#> PARSER -> CLASS
class Parser:
    #= CLASS -> VARIABLES
    chart: list[set[State]]
    waiting: list[defaultdict[type[NonTerminal] | str, set[State]]]
    perimeter: dict[State, SPPF]
    forest: dict[SPPF, list[Packed]]
    #= CLASS -> INIT
    def __init__(self) -> None: self.reset()
    #= CLASS -> RESET
    def reset(self) -> None:
        self.chart = []
        self.waiting = []
        self.perimeter = {}
        self.forest = {}
    #= CLASS -> RUN OVERLOADS
    @overload
    def run(self, tokens: list[Token], build: Literal[True]) -> Start: ...
    @overload
    def run(self, tokens: list[Token], build: Literal[False]) -> bool: ...
    #= CLASS -> RUN
    def run(self, tokens: list[Token], build: bool) -> Start | bool:
        tokens = [token for token in tokens if token.compilable()]
        self.chart = [set() for index in range(len(tokens) + 1)]
        self.waiting = [defaultdict(set) for index in range(len(tokens) + 1)]
        start = ("$", (Start,), 0, 0)
        self.chart[0].add(start)
        self.perimeter[start] = ('e', 0, 0)
        self.parse(tokens)
        root = ("$", (Start,), 1, 0)
        print(("$", 0, len(tokens)) in self.forest)
        return Start([]) if build else root in self.chart[len(tokens)]
    #= CLASS -> LOOP
    def parse(self, tokens: list[Token]) -> None:
        tklen = len(tokens)
        for index in range(tklen + 1):
            agenda = list(self.chart[index])
            while agenda:
                rule, productions, slot, starting = popped = agenda.pop()
                assert popped in self.perimeter
                at = productions[slot] if not slot == productions.__len__() else None # type: ignore
                if at is None:
                    completed = (rule, starting, index)
                    self.forest.setdefault(completed, [])
                    for parent in self.waiting[starting][rule]:
                        prule, pproductions, pslot, pstarting = parent
                        pnode = (prule, pstarting, index)
                        self.forest.setdefault(pnode, [])
                        packed = (pproductions, pstarting, (completed,))
                        if packed not in self.forest[pnode]: self.forest[pnode].append(packed)
                        if parent not in self.chart[index]:
                            self.chart[index].add(parent)
                            agenda.append(parent)
                elif hasattr(at, "compilable") and index < tklen:
                    if (token := tokens[index]).__class__ == at:
                        sppf = (token, index, index + 1)
                        self.forest.setdefault(sppf, [])
                        self.chart[index + 1].add((rule, productions, slot + 1, starting))
                elif at.__class__ is str or hasattr(at, "freeze"):
                    at: str | type[NonTerminal]
                    self.waiting[index][at].add((rule, productions, slot + 1, starting))
                    for newrule in GRAMMAR.productions[at]:
                        key = (rule, productions, slot + 1, starting) if not newrule else (at, newrule, 0, index)
                        if not newrule: self.perimeter[key] = ("e", 0, 0)
                        if key not in self.chart[index]:
                            self.chart[index].add(key)
                            agenda.append(key)