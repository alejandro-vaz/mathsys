#^
#^  HEAD
#^

#> HEAD -> MODULES
from typing import Any
from collections import defaultdict
from functools import cache, lru_cache

#> HEAD -> DATA
from .tokenizer import Token
from .start import Start
from .nonterminal import NonTerminal
from .grammar import GRAMMAR, score, Temporal
from .issues import Syntax
from .level1 import Level1
from .level2 import Level2
from .level3 import Level3
from .level4 import Level4
from .level5 import Level5


#^
#^  RESOURCES
#^

#> RESOURCES -> KEY
Key = tuple[
    type[NonTerminal] | Temporal,
    tuple[type[Token | NonTerminal] | Temporal, ...],
    int,
    int,
    bool,
    type[Token | NonTerminal] | Temporal | None
]
@lru_cache(512)
def makekey(rule: type[NonTerminal] | Temporal, productions: tuple[type[Token | NonTerminal] | Temporal], slot: int, starting: int) -> Key: return (
    rule,
    productions,
    slot,
    starting,
    full := slot == productions.__len__(),
    at := productions[slot] if not full else None
)

#> RESOURCES -> ACCESS
Access = tuple[
    Temporal | type[NonTerminal] | Token,
    int,
    int
]
def makeaccess(symbol: Temporal | type[NonTerminal] | Token, start: int, end: int) -> Access: return (
    symbol,
    start,
    end
)


#^
#^  PARSER
#^

#> PARSER -> ASSEMBLE
def assemble(symbol: Temporal | type[NonTerminal], children: tuple[Any]) -> NonTerminal | tuple:
    flat = []
    for child in children:
        if child is None: continue
        if isinstance(child, tuple): flat.extend(child)
        else: flat.append(child)
    if isinstance(symbol, Temporal): return tuple(flat)
    else:
        if symbol in {Level1, Level2, Level3, Level4, Level5}: return flat[0]
        return symbol([element for element in flat if not isinstance(element, Token) or element.important()])

#> PARSER -> CLASS
class Parser:
    #= CLASS -> VARIABLES
    chart: list[dict[Key, set[tuple[Access, ...]]]]
    tokens: list[Token]
    pool: dict[Access, set[tuple[Access, ...]]]
    waiting: list[defaultdict[type[Token | NonTerminal] | Temporal, set[Key]]]
    #= CLASS -> INIT
    def __init__(self) -> None: self.reset()
    #= CLASS -> RESET
    def reset(self) -> None:
        self.chart = []
        self.tokens = []
        self.pool = defaultdict(set)
        self.materialize.cache_clear()
    #= CLASS -> RECALL
    def recall(self, at: int, key: Key) -> set[tuple[Access, ...]]:
        chart = self.chart[at]
        if key not in chart: chart[key] = set()
        if key[5] is not None: self.waiting[at][key[5]].add(key)
        return chart[key]
    #= CLASS -> MATERIALIZE
    @cache
    def materialize(self, index: int, key: Key, end: int) -> Access:
        for backpointer in self.recall(index, key): self.pool[access := makeaccess(key[0], key[3], end)].add(backpointer)
        return access
    #= CLASS -> BEST
    def best(self, access: Access) -> tuple[int, Any]:
        if isinstance(access[0], Token): return 0, access[0]
        bcore = -1
        btree = None
        node = self.pool[access]
        for pack in node:
            total = score(access[0])
            children = []
            for child in pack:
                points, tree = self.best(child)
                total += points
                children.append(tree)
            if total > bcore:
                bcore = total
                btree = assemble(access[0], tuple(children))
        return bcore, btree
    #= CLASS -> RUN
    def run(self, tokens: list[Token]) -> Start:
        self.tokens = [token for token in tokens if token.compilable()]
        self.chart = [{} for index in range(len(self.tokens) + 1)]
        self.waiting = [defaultdict(set) for index in range(len(self.tokens) + 1)]
        self.recall(0, makekey("$", (Start,), 0, 0)).add(())
        self.loop()
        return Start([])
        #last = self.chart[len(self.tokens)].get(key := makekey("$", (Start,), 1, 0))
        #if last is None or not key[4]: raise Syntax()
        #for backpointer in last:
        #    first = backpointer[0]
        #    if isinstance(first, tuple):
        #        for key in self.chart[first[2]]:
        #            if key[4] and key[0] is first[0] and key[3] == first[1]:
        #                state = None
        #                end = None
        #                for index, chart in enumerate(self.chart):
        #                    if (possible := chart.get(key)) is not None: state = possible; end = index; break
        #                if state is None or end is None: raise Syntax()
        #                return self.best(makeaccess(key[0], key[3], end))[1]
        #raise Syntax()
    #= CLASS -> LOOP
    def loop(self) -> None:
        worklist: set[Key] = set()
        tklen = len(self.tokens)
        for index in range(tklen + 1):
            worklist.update(self.chart[index])
            while worklist:
                value = (key := worklist.pop())[5]
                if value is not None and (
                    value.__class__ == Temporal
                    or hasattr(value, "freeze")
                ):
                    changed = []
                    for productions in GRAMMAR.productions[value]:
                        if not (other := self.recall(index, nkey := makekey(value, productions, 0, index))):
                            other.add(())
                            changed.append(nkey)
                    worklist.update(changed)
                elif value is not None and index < tklen and hasattr(value, "compilable"):
                    if self.tokens[index].__class__ == value:
                        other = self.recall(index + 1, makekey(key[0], key[1], key[2] + 1, key[3]))
                        for backpointer in self.recall(index, key): 
                            other.add(backpointer + (makeaccess(self.tokens[index], index, index + 1),))
                elif key[4]:
                    for other in self.waiting[key[3]][key[0]]:
                        extra = self.recall(index, advanced := makekey(other[0], other[1], other[2] + 1, other[3]))
                        stored = extra.__len__()
                        for backpointer in self.chart[key[3]][other]:
                            extra.add(backpointer + (self.materialize(index, key, index),))
                        if extra.__len__() > stored: 
                            worklist.add(advanced)
                            worklist.update(element for element in self.chart[index] if element[4])