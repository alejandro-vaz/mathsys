#^
#^  HEAD
#^

#> HEAD -> MODULES
from typing import Any, Self
from collections import defaultdict
from functools import cache

#> HEAD -> DATA
from .tokenizer import Token, ORDER
from .start import Start
from .nonterminal import NonTerminal
from .grammar import SYNTAX, NONTERMINALS, score
from .issues import BrokenSyntax
from .level1 import Level1
from .level2 import Level2
from .level3 import Level3
from .level4 import Level4
from .level5 import Level5


#^
#^  RESOURCES
#^

#> RESOURCES -> KEY
@cache
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
    def __eq__(self, value: Self) -> bool: return self.hashed == value.hashed

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
    def __eq__(self, value: Access) -> bool: return self.hashed == value.hashed

#> RESOURCES -> GRAMMAR
class Grammar:
    productions: dict[str | type[NonTerminal], tuple[tuple[str, ...]]]
    bnf: str
    def __init__(self, bnf: str) -> None: 
        self.bnf = bnf
        self.productions = self.convert()
    def convert(self) -> dict[str | type[NonTerminal], tuple[tuple[str, ...]]]:
        syntax = defaultdict(list)
        for line in [line.strip() for line in self.bnf.splitlines()]:
            rule, productions = [part.strip() for part in line.split("->", 1)]
            for variant in [variant.strip() for variant in productions.split("|")]:
                if not variant: syntax[self.transform(rule)].append(tuple())
                else: syntax[self.transform(rule)].append(tuple(self.transform(item) for item in variant.split()))
        frozen = {}
        for key, value in syntax.items(): frozen[key] = tuple(value)
        frozen["$"] = (Start,)
        return frozen
    def transform(self, atom: str) -> type[NonTerminal | Token] | str:
        if atom in (token := {item.__name__: item for item in ORDER}): return token[atom]
        if atom in NONTERMINALS: return NONTERMINALS[atom]
        return atom
    def __repr__(self) -> str: return self.bnf


#^
#^  PARSER
#^

#> PARSER -> ASSEMBLE
def assemble(symbol: str | type[NonTerminal] | Token, children: tuple[Any]) -> NonTerminal | tuple:
    flat = []
    for child in children:
        if child is None: continue
        if isinstance(child, tuple): flat.extend(child)
        else: flat.append(child)
    match symbol:
        case str(): return tuple(flat)
        case type():
            if symbol in {Level1, Level2, Level3, Level4, Level5}: return flat[0]
            return symbol([element for element in flat if not isinstance(element, Token) or element.important()])
    raise BrokenSyntax()

#> PARSER -> CLASS
class Parser:
    #= CLASS -> VARIABLES
    grammar = Grammar(SYNTAX)
    chart: list[dict[Key, set[tuple[Access, ...]]]]
    tokens: list[Token]
    pool: dict[Access, set[tuple[Access, ...]]]
    waiting: list[defaultdict[type[Token | NonTerminal] | str, set[Key]]]
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
    def recall(self, at: int, key: Key) -> set[tuple[Access, ...]]:
        chart = self.chart[at]
        if key not in chart:
            chart[key] = set()
            if key.at is not None: self.waiting[at][key.at].add(key)
        return chart[key]

        #item = chart[key] = chart.get(key, set())
        #return item
    #= CLASS -> MATERIALIZE
    @cache
    def materialize(self, index: int, key: Key, end: int) -> Access:
        for backpointer in self.recall(index, key): self.pool[access := Access(key.rule, key.starting, end)].add(backpointer)
        return access
    #= CLASS -> BUILD
    def build(self, key: Key) -> Start: 
        state = None
        end = None
        for index, chart in enumerate(self.chart):
            if (possible := chart.get(key)) is not None: state = possible; end = index; break
        if state is None or not key.full or end is None: raise BrokenSyntax()
        return self.best(Access(key.rule, key.starting, end))[1]
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
        tklen = len(tokens)
        self.tokens = [token for token in tokens if token.compilable()]
        self.chart = [{} for index in range(tklen + 1)]
        self.waiting = [defaultdict(set) for index in range(tklen + 1)]
        self.recall(0, Key("$", (Start,), 0, 0)).add(())
        self.loop()
        last = self.chart[len(self.tokens)].get(key := Key("$", (Start,), 1, 0))
        if last is None or not key.full: raise BrokenSyntax()
        for backpointer in last:
            first = backpointer[0]
            if isinstance(first, Access):
                for key in self.chart[first.end]:
                    if key.full and key.rule is first.symbol and key.starting == first.start: return self.build(key)
        raise BrokenSyntax()
    #= CLASS -> LOOP
    def loop(self) -> None:
        worklist: set[Key] = set()
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
                    for productions in self.grammar.productions[value]:
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
                    waiting = self.waiting[key.starting].get(key.rule)
                    if not waiting: continue
                    completed = self.materialize(index, key, index)
                    for other in waiting:
                        nkey = Key(
                            other.rule,
                            other.productions,
                            other.slot + 1,
                            other.starting
                        )
                        extra = self.recall(index, nkey)
                        stored = len(extra)
                        for backpointer in self.chart[key.starting][other]:
                            extra.add(backpointer + (completed,))
                        if len(extra) > stored: worklist.add(nkey)
                #elif key.full:
                #    for other in self.chart[key.starting] | {key: value}:
                #        if other.at == key.rule:
                #            extra = self.recall(index, Key(other.rule, other.productions, other.slot + 1, other.starting))
                #            stored = len(extra)
                #            for backpointer in self.chart[key.starting][other]:
                #                extra.add(backpointer + (self.materialize(index, key, index),))
                #            if len(extra) > stored: worklist.update(self.chart[index])