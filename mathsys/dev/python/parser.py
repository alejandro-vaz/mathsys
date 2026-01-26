#^
#^  HEAD
#^

#> HEAD -> MODULES
from collections import defaultdict

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
    bool,
    type[Token | NonTerminal] | str | None
]
def makestate(rule: type[NonTerminal] | str, productions: tuple[type[Token | NonTerminal] | str, ...], slot: int, starting: int) -> State: return (
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
    chart: list[set[State]]
    waiting: list[defaultdict[type[NonTerminal] | str, set[State]]]
    #= CLASS -> INIT
    def __init__(self) -> None: self.reset()
    #= CLASS -> RESET
    def reset(self) -> None:
        self.chart = []
        self.waiting = []
    #= CLASS -> RUN
    def run(self, tokens: list[Token]) -> Start:
        tokens = [token for token in tokens if token.compilable()]
        self.chart = [set() for index in range(len(tokens) + 1)]
        self.waiting = [defaultdict(set) for index in range(len(tokens) + 1)]
        self.chart[0].add(makestate("$", (Start,), 0, 0))
        self.loop(tokens)
        print(makestate("$", (Start,), 1, 0) in self.chart[len(tokens)])
        return Start([])
    #= CLASS -> LOOP
    def loop(self, tokens: list[Token]) -> None:
        tklen = len(tokens)
        for index in range(tklen + 1):
            agenda = list(self.chart[index])
            while agenda:
                rule, productions, slot, starting, full, at = agenda.pop()
                #complete
                if full:
                    for nextone in [waiting for waiting in self.waiting[starting][rule]]:
                        if nextone not in self.chart[index]:
                            self.chart[index].add(nextone)
                            agenda.append(nextone)
                #scan
                elif hasattr(at, "compilable") and index < tklen:
                    if tokens[index].__class__ == at:
                        self.chart[index + 1].add(makestate(rule, productions, slot + 1, starting))
                #predict
                elif at.__class__ is str or hasattr(at, "freeze"):
                    self.waiting[index][at].add(makestate(rule, productions, slot + 1, starting)) # type: ignore
                    for newrule in GRAMMAR.productions[at]: # type: ignore
                        key = makestate(rule, productions, slot + 1, starting) if not newrule else makestate(at, newrule, 0, index) # type: ignore
                        if key not in self.chart[index]:
                            self.chart[index].add(key)
                            agenda.append(key)
