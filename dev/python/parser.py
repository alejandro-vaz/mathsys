#^
#^  HEAD
#^

#> HEAD -> MODULES
from collections import defaultdict, deque
from functools import lru_cache
from typing import cast, Literal, overload

#> HEAD -> DATA
from .tokenizer import Token
from .start import Start
from .nonterminal import NonTerminal
from .grammar import GRAMMAR, SCORE
from .issues import Syntax, InputTooLong


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

#> RESOURCES -> ACCESS
SPPF = tuple[
    str | type[NonTerminal] | Token,
    int,
    int
]


#^
#^  PARSER
#^

#> PARSER -> ASSEMBLE
def assemble(symbol: str | type[NonTerminal], children: tuple[None | Token | NonTerminal | tuple[Token | NonTerminal]]) -> NonTerminal | tuple[Token | NonTerminal]:
    flat = []
    for child in children: 
        if child is None: continue
        flat.extend(child) if isinstance(child, tuple) else flat.append(child)
    return tuple(flat) if isinstance(symbol, str) else (
        flat[0] if symbol.placeholder() 
        else symbol([item for item in flat if not isinstance(item, Token) or item.important()])
    )

#> PARSER -> CLASS
class Parser:
    #= CLASS -> VARIABLES
    chart: list[dict[State, set[tuple[SPPF, ...]]]]
    pool: dict[SPPF, set[tuple[SPPF, ...]]]
    waiting: list[defaultdict[type[Token | NonTerminal] | str, set[State]]]
    #= CLASS -> RESET
    def prepare(self, tokens: list[Token]) -> list[Token]:
        tokens = [token for token in tokens if token.compilable()]
        if (length := len(tokens)) > (maximum := 2500): raise InputTooLong(length, maximum)
        self.recall.cache_clear()
        self.chart = [{} for index in range(length + 1)]
        self.pool = defaultdict(set)
        self.waiting = [defaultdict(set) for index in range(length + 1)]
        self.recall(0, ("$", (Start,), 0, 0)).add(())
        return tokens
    #= CLASS -> RECALL
    @lru_cache
    def recall(self, index: int, state: State) -> set[tuple[SPPF, ...]]:
        rule, productions, slot, starting = state
        at = productions[slot] if not slot == productions.__len__() else None
        if at is not None: self.waiting[index][at].add(state)
        return self.chart[index].setdefault(state, set())
    #= CLASS -> BEST
    def best(self, node: SPPF) -> tuple[int, NonTerminal | Token | None | tuple[Token | NonTerminal] | None]:
        symbol, start, end = node
        if isinstance(symbol, Token): return 0, symbol
        bcore = -1
        btree = None
        derivations = self.pool[node]
        for pack in derivations:
            total = SCORE.get(symbol, 0)
            children = []
            for child in pack:
                points, tree = self.best(child)
                total += points
                children.append(tree)
            if total > bcore:
                bcore = total
                btree = assemble(symbol, tuple(children))
        return bcore, btree
    #= CLASS -> RUN OVERLOADS
    @overload
    def run(self, tokens: list[Token], build: Literal[True]) -> Start: ...
    @overload
    def run(self, tokens: list[Token], build: Literal[False]) -> bool: ...
    #= CLASS -> RUN
    def run(self, tokens: list[Token], build: bool) -> Start | bool:
        tokens = self.prepare(tokens)
        try:
            for backpointer in self.loop(tokens):
                symbol, start, end = backpointer[0]
                for key in self.chart[end]:
                    rule, productions, slot, starting = key
                    if slot == len(productions) and rule is symbol and starting == start:
                        for index, chart in enumerate(self.chart):
                            if key in chart: 
                                return cast(Start, self.best((rule, starting, index))) if build else True
            assert False
        except Syntax:
            if not build: return False
            else: raise
    #= CLASS -> LOOP
    def loop(self, tokens: list[Token]) -> set[tuple[SPPF, ...]]:
        tklen = len(tokens)
        for index in range(tklen + 1):
            token = tokens[index] if tklen > index else None
            agenda = deque(self.chart[index])
            completed = set()
            while agenda:
                rule, productions, slot, starting = current = agenda.popleft()
                at = productions[slot] if not slot == productions.__len__() else None
                if at is None:
                    for other in self.waiting[starting][rule]:
                        extra = self.recall(index, advanced := (other[0], other[1], other[2] + 1, other[3]))
                        stored = extra.__len__()
                        self.pool[pointer := (rule, starting, index)].update(self.recall(index, current))
                        for backpointer in self.chart[starting][other]:
                            extra.add(backpointer + (pointer,))
                        if extra.__len__() > stored: 
                            agenda.append(advanced)
                            agenda.extend(completed)
                    completed.add(current)
                elif token.__class__ == at:
                    other = self.recall(index + 1, (rule, productions, slot + 1, starting))
                    for backpointer in self.recall(index, current): 
                        other.add(backpointer + ((token, index, index + 1),)) # type: ignore
                elif at.__class__ is str or hasattr(at, "freeze"):
                    changed = []
                    for productions in GRAMMAR.productions[at]: # type: ignore
                        if not (other := self.recall(index, key := (at, productions, 0, index))): # type: ignore
                            other.add(())
                            changed.append(key)
                    agenda.extend(changed)
        try: return self.chart[len(tokens)][("$", (Start,), 1, 0)]
        except: raise Syntax()