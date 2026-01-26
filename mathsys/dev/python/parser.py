#^
#^  HEAD
#^

#> HEAD -> MODULES
from collections import defaultdict
from typing import overload, Literal, cast
from functools import cache

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
    int,
    type[Token | NonTerminal] | str | None
]
def sign(rule: type[NonTerminal] | str, productions: tuple[type[Token | NonTerminal] | str, ...], slot: int, starting: int) -> State: return (
    rule,
    productions,
    slot,
    starting,
    productions[slot] if not slot == productions.__len__() else None
)

#> RESOURCES -> SPPF
SPPF = tuple[
    str | type[NonTerminal] | Token,
    int,
    int
]
def record(symbol: str | type[NonTerminal] | Token, start: int, end: int) -> SPPF: return (symbol, start, end)


#^
#^  PARSER
#^

#> PARSER -> CLASS
class Parser:
    #= CLASS -> VARIABLES
    chart: list[set[State]]
    waiting: list[defaultdict[type[NonTerminal] | str, set[State]]]
    #= CLASS -> INIT
    def __init__(self) -> None: self.reset()
    #= CLASS -> RESET
    def reset(self) -> None:
        self.chart = []
        self.waiting = []
        self.creations = {}
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
        self.chart[0].add(sign("$", (Start,), 0, 0))
        self.parse(tokens)
        root = sign("$", (Start,), 1, 0)
        return Start([]) if build else root in self.chart[len(tokens)]
    #= CLASS -> LOOP
    def parse(self, tokens: list[Token]) -> None:
        tklen = len(tokens)
        for index in range(tklen + 1):
            agenda = list(self.chart[index])
            while agenda:
                rule, productions, slot, starting, at = popped = agenda.pop() # type: ignore
                if at is None:
                    for parent in [parent for parent in self.waiting[starting][rule]]:
                        if parent not in self.chart[index]:
                            self.chart[index].add(parent)
                            agenda.append(parent)
                elif hasattr(at, "compilable") and index < tklen:
                    if tokens[index].__class__ == at:
                        self.chart[index + 1].add(sign(rule, productions, slot + 1, starting))
                elif at.__class__ is str or hasattr(at, "freeze"):
                    at: str | type[NonTerminal]
                    self.waiting[index][at].add(sign(rule, productions, slot + 1, starting))
                    for newrule in GRAMMAR.productions[at]:
                        key = sign(rule, productions, slot + 1, starting) if not newrule else sign(at, newrule, 0, index)
                        if key not in self.chart[index]:
                            self.chart[index].add(key)
                            agenda.append(key)