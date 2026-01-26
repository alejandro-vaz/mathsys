#^
#^  HEAD
#^

#> HEAD -> MODULES
import cProfile
from typing import cast

#> HEAD -> DATA
from .tokenizer import Token
from .start import Start
from .nonterminal import NonTerminal
from .grammar import GRAMMAR


#^
#^  RESOURCES
#^

#> RESOURCES -> KEY
Key = tuple[
    type[NonTerminal] | str,
    tuple[type[Token | NonTerminal] | str, ...],
    int,
    int,
    bool,
    type[Token | NonTerminal] | str | None
]
def makekey(rule: type[NonTerminal] | str, productions: tuple[type[Token | NonTerminal] | str, ...], slot: int, starting: int) -> Key: return (
    rule,
    productions,
    slot,
    starting,
    full := slot == productions.__len__(),
    productions[slot] if not full else None
)


#^
#^  PARSER
#^

#> PARSER -> CLASS
class Parser:
    #= CLASS -> VARIABLES
    chart: list[set[Key]]
    #= CLASS -> INIT
    def __init__(self) -> None: self.reset()
    #= CLASS -> RESET
    def reset(self) -> None:
        self.chart = []
    #= CLASS -> RUN
    def run(self, tokens: list[Token]) -> Start:
        tokens = [token for token in tokens if token.compilable()]
        self.chart = [set() for index in range(len(tokens) + 1)]
        self.chart[0].add(makekey("$", (Start,), 0, 0))
        self.loop(tokens)
        print(makekey("$", (Start,), 1, 0) in self.chart[len(tokens)])
        return Start([])
    #= CLASS -> LOOP
    def loop(self, tokens: list[Token]) -> None:
        tklen = len(tokens)
        for index in range(tklen + 1):
            agenda = list(self.chart[index])
            while agenda:
                rule, productions, slot, starting, full, at = agenda.pop()
                #predict
                if isinstance(at, str) or (isinstance(at, type) and issubclass(at, NonTerminal)):
                    
                    for newrule in GRAMMAR.productions[at]:
                        key = makekey(rule, productions, slot + 1, starting) if not newrule else makekey(at, newrule, 0, index)
                        if key not in self.chart[index]:
                            self.chart[index].add(key)
                            agenda.append(key)
                #scan
                elif index < tklen and isinstance(at, type) and issubclass(at, Token):
                    if tokens[index].__class__ == at:
                        self.chart[index + 1].add(makekey(rule, productions, slot + 1, starting))
                #complete
                elif full:
                    for (strule, stproductions, stslot, ststarting, stfull, stat) in [waiting for waiting in self.chart[starting] if waiting[5] == rule]:
                        nextone = makekey(strule, stproductions, stslot + 1, ststarting)
                        if nextone not in self.chart[index]:
                            self.chart[index].add(nextone)
                            agenda.append(nextone)