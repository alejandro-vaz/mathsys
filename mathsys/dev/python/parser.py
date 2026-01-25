#^
#^  HEAD
#^

#> HEAD -> MODULES
from typing import Any, Self
from collections import defaultdict
from functools import cache, lru_cache

#> HEAD -> DATA
from .tokenizer import Token
from .start import Start
from .nonterminal import NonTerminal
from .grammar import GRAMMAR, score
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
KeyType = Any
@lru_cache(65536)
class Key:
    rule: type[NonTerminal] | str
    productions: tuple[type[Token | NonTerminal] | str, ...]
    slot: int
    starting: int
    plen: int
    full: bool
    at: type[Token | NonTerminal] | str | None
    def __init__(self, rule: type[NonTerminal] | str, productions: tuple[type[Token | NonTerminal] | str, ...], slot: int, starting: int) -> None:
        self.rule = rule
        self.productions = productions
        self.slot = slot
        self.starting = starting
        self.plen = productions.__len__()
        self.full = slot == self.plen
        self.at = productions[slot] if not self.full else None
        self.hashed = hash((rule, productions, slot, starting))
    def __hash__(self) -> int: return self.hashed
    def __eq__(self, value: Self) -> bool: return (
        self.rule == value.rule
        and self.productions == value.productions
        and self.slot == value.slot
        and self.starting == value.starting
    )

#> RESOURCES -> ACCESS
class Access:
    symbol: str | type[NonTerminal] | Token
    start: int
    end: int
    hashed: int
    def __init__(self, symbol: str | type[NonTerminal] | Token, start: int, end: int) -> None:
        self.symbol = symbol
        self.start = start
        self.end = end
        self.hashed = hash((symbol, start, end))
    def __hash__(self) -> int: return self.hashed
    def __eq__(self, value: Access) -> bool: return (
        self.symbol == value.symbol
        and self.start == value.start
        and self.end == value.end
    )


#^
#^  PARSER
#^

#> PARSER -> ASSEMBLE
def assemble(symbol: str | type[NonTerminal], children: tuple[Any]) -> NonTerminal | tuple:
    flat = []
    for child in children:
        if child is None: continue
        if isinstance(child, tuple): flat.extend(child)
        else: flat.append(child)
    if isinstance(symbol, str): return tuple(flat)
    else:
        if symbol in {Level1, Level2, Level3, Level4, Level5}: return flat[0]
        return symbol([element for element in flat if not isinstance(element, Token) or element.important()])

#> PARSER -> CLASS
class Parser:
    #= CLASS -> VARIABLES
    chart: list[dict[KeyType, set[tuple[Access, ...]]]]
    tokens: list[Token]
    pool: dict[Access, set[tuple[Access, ...]]]
    waiting: list[defaultdict[type[Token | NonTerminal] | str, set[KeyType]]]
    #= CLASS -> INIT
    def __init__(self) -> None: self.reset()
    #= CLASS -> RESET
    def reset(self) -> None:
        self.chart = []
        self.tokens = []
        self.pool = defaultdict(set)
        self.recall.cache_clear()
        self.materialize.cache_clear()
        self.best.cache_clear()
    #= CLASS -> RECALL
    @cache
    def recall(self, at: int, key: KeyType) -> set[tuple[Access, ...]]:
        chart = self.chart[at]
        if key not in chart: chart[key] = set()
        if key.at is not None: self.waiting[at][key.at].add(key)
        return chart[key]
    #= CLASS -> MATERIALIZE
    @cache
    def materialize(self, index: int, key: KeyType, end: int) -> Access:
        for backpointer in self.recall(index, key): self.pool[access := Access(key.rule, key.starting, end)].add(backpointer)
        return access
    #= CLASS -> BEST
    @cache
    def best(self, access: Access) -> tuple[int, Any]:
        if isinstance(access.symbol, Token): return 0, access.symbol
        bcore = -1
        btree = None
        node = self.pool[access]
        for pack in node:
            total = score(access.symbol)
            children = []
            for child in pack:
                points, tree = self.best(child)
                total += points
                children.append(tree)
            if total > bcore:
                bcore = total
                btree = assemble(access.symbol, tuple(children))
        return bcore, btree
    #= CLASS -> RUN
    def run(self, tokens: list[Token]) -> Start:
        self.tokens = [token for token in tokens if token.compilable()]
        self.chart = [{} for index in range(len(self.tokens) + 1)]
        self.waiting = [defaultdict(set) for index in range(len(self.tokens) + 1)]
        self.recall(0, Key("$", (Start,), 0, 0)).add(())
        self.loop()
        last = self.chart[len(self.tokens)].get(key := Key("$", (Start,), 1, 0))
        if last is None or not key.full: raise Syntax()
        for backpointer in last:
            first = backpointer[0]
            if isinstance(first, Access):
                for key in self.chart[first.end]:
                    if key.full and key.rule is first.symbol and key.starting == first.start:
                        state = None
                        end = None
                        for index, chart in enumerate(self.chart):
                            if (possible := chart.get(key)) is not None: state = possible; end = index; break
                        if state is None or end is None: raise Syntax()
                        return self.best(Access(key.rule, key.starting, end))[1]
        raise Syntax()
    #= CLASS -> LOOP
    def loop(self) -> None:
        worklist: set[KeyType] = set()
        tklen = len(self.tokens)
        for index in range(tklen + 1):
            worklist.update(self.chart[index])
            while worklist:
                value = (key := worklist.pop()).at
                if value is not None and (
                    value.__class__ == str
                    or hasattr(value, "freeze")
                ):
                    changed = []
                    for productions in GRAMMAR.productions[value]:
                        if not (other := self.recall(index, nkey := Key(value, productions, 0, index))):
                            other.add(())
                            changed.append(nkey)
                    worklist.update(changed)
                elif value is not None and index < tklen and hasattr(value, "compilable"):
                    if self.tokens[index].__class__ == value:
                        other = self.recall(index + 1, Key(key.rule, key.productions, key.slot + 1, key.starting))
                        for backpointer in self.recall(index, key): 
                            other.add(backpointer + (Access(self.tokens[index], index, index + 1),))
                elif key.full:
                    
                    for other in self.waiting[key.starting][key.rule]:
                        extra = self.recall(index, advanced := Key(other.rule, other.productions, other.slot + 1, other.starting))
                        stored = extra.__len__()
                        for backpointer in self.chart[key.starting][other]:
                            extra.add(backpointer + (self.materialize(index, key, index),))
                        if extra.__len__() > stored: 
                            worklist.add(advanced)
                            worklist.update(element for element in self.chart[index] if element.full)