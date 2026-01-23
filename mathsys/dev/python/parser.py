#^
#^  HEAD
#^

#> HEAD -> MODULES
from dataclasses import dataclass, field
from typing import Any
from collections import defaultdict
from itertools import product
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
@dataclass(frozen = True)
class Key:
    rule: type[NonTerminal] | str
    productions: tuple[type[Token | NonTerminal] | str, ...]
    slot: int
    starting: int

#> RESOURCES -> STATE
@dataclass(eq = False)
class State:
    rule: type[NonTerminal] | str
    productions: tuple[str | type[Token | NonTerminal], ...]
    slot: int
    starting: int
    backpointers: set[tuple[Any, ...]] = field(default_factory = set)
    def at(self) -> str | type[Token | NonTerminal] | None: 
        return self.productions[self.slot] if self.slot < len(self.productions) else None
    def full(self) -> bool: return self.slot >= len(self.productions)   
    def key(self) -> Key: return Key(self.rule, self.productions, self.slot, self.starting)
    def __hash__(self) -> int: return hash(self.key())
    def __eq__(self, other: Any) -> bool: return isinstance(other, State) and self.key() == other.key()
    def __repr__(self) -> str:
        before = " ".join(str(element) for element in self.productions[:self.slot])
        after = " ".join(str(element) for element in self.productions[self.slot:])
        return f"[{self.rule} -> {before} • {after}, starting = {self.starting}]"

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

#> RESOURCES -> SPPF
@dataclass(eq = False)
class SPPF:
    symbol: str | type[NonTerminal] | Token
    start: int
    end: int
    pack: set[tuple[SPPF, ...]] = field(default_factory = set)
    def follow(self, children: tuple[SPPF, ...]) -> bool:
        if children in self.pack: return False
        self.pack.add(children)
        return True
    def node(self) -> tuple[str | type[NonTerminal] | Token, int, int]: return self.symbol, self.start, self.end
    def __eq__(self, value: object) -> bool: return hash(self) == hash(value)
    def __hash__(self) -> int: return hash(self.node())


#^
#^  PARSER
#^

#> PARSER -> CLASS
class Parser:
    #= CLASS -> VARIABLES
    grammar = Grammar(SYNTAX)
    chart: list[dict[Key, State]]
    changed: bool
    tokens: list[Token]
    pool: dict[tuple[str | type[NonTerminal] | Token, int, int], SPPF]
    #= CLASS -> INIT
    def __init__(self) -> None: self.reset()
    #= CLASS -> RESET
    def reset(self) -> None:
        self.build.cache_clear()
        self.chart = []
        self.changed = True
        self.tokens = []
        self.pool = {}
    #= CLASS -> RECALL
    def recall(self, at: int, key: Key) -> State:
        if not key in self.chart[at]: self.chart[at][key] = State(key.rule, key.productions, key.slot, key.starting)
        return self.chart[at][key]
    #= CLASS -> CRGET
    def seek(self, symbol: str | type[NonTerminal] | Token, start: int, end: int) -> SPPF:
        key = (symbol, start, end)
        node = self.pool.get(key)
        if node is None:
            node = SPPF(symbol, start, end)
            self.pool[key] = node
        return node
    #= CLASS -> MATERIALIZE
    def materialize(self, state: State, end: int) -> SPPF:
        node = self.seek(state.rule, state.starting, end)
        for backpointer in state.backpointers: node.follow(tuple(backpointer))
        return node
    #= CLASS -> BUILD
    @cache
    def build(self, brick: Key) -> set: 
        state = None
        end = None
        for index, chart in enumerate(self.chart):
            if (possible := chart.get(brick)) is not None: state = possible; end = index; break
        if state is None or not state.full() or end is None: return set()
        key = (state.rule, state.starting, end)
        root = self.pool.get(key)
        if root is None: return set()
        def expand(node: SPPF) -> set:
            results = set()
            if isinstance(node.symbol, Token):
                results.add((node.symbol,) if node.symbol.important() else ())
                return results
            for pack in node.pack:
                choices = [expand(child) for child in pack]
                if any(len(choice) == 0 for choice in choices): continue
                for produce in product(*choices):
                    flat = []
                    for item in produce: (flat.extend if isinstance(item, tuple) else flat.append)(item)
                    match node.symbol:
                        case str(): results.add(tuple(flat))
                        case type():
                            if node.symbol in {Level1, Level2, Level3, Level4, Level5}: results.add(flat[0] if flat else None)
                            else: results.add(node.symbol(flat))
                        case other: results.append(flat)
            return results
        return expand(root)
    #= CLASS -> PREDICT
    def predict(self, state: State, index: int, value: type[NonTerminal] | str) -> None:
        for productions in self.grammar.productions[value]:
            if not (other := self.recall(index, Key(value, productions, 0, index))).backpointers:
                other.backpointers.add(tuple())
                self.changed = True
    #= CLASS -> SCAN
    def scan(self, state: State, index: int, value: type[Token]) -> None:
        if isinstance(self.tokens[index], value):
            other = self.recall(index + 1, Key(state.rule, state.productions, state.slot + 1, state.starting))
            stored = len(other.backpointers)
            node = self.seek(self.tokens[index], index, index + 1)
            for backpointer in state.backpointers: other.backpointers.add(backpointer + (node,))
            if len(other.backpointers) > stored: self.changed = True
    #= CLASS -> COMPLETE
    def complete(self, state: State, index: int, value: None) -> None:
        nodes = [self.materialize(state, index)]
        for other in list(self.chart[state.starting].values()):
            if other.at() == state.rule:
                extra = self.recall(index, Key(other.rule, other.productions, other.slot + 1, other.starting))
                stored = len(extra.backpointers)
                for backpointer in other.backpointers: 
                    for node in nodes: extra.backpointers.add(backpointer + (node,))
                if len(extra.backpointers) > stored: self.changed = True
    #= CLASS -> RUN
    def run(self, tokens: list[Token]) -> Start:
        self.tokens = [token for token in tokens if token.compilable()]
        self.chart = [{} for index in range(len(self.tokens) + 1)]
        self.recall(0, Key("$", (Start,), 0, 0)).backpointers.add(tuple())
        self.loop()
        last = self.chart[len(self.tokens)].get(Key("$", (Start,), 1, 0))
        if last is None or not last.full(): raise BrokenSyntax()
        trees = set()
        for backpointer in last.backpointers:
            first = backpointer[0]
            if isinstance(first, State): raise BrokenSyntax()
            if isinstance(first, SPPF):
                brick = None
                for key in self.chart[first.end]:
                    if key.rule == first.symbol and key.starting == first.start and key.slot == len(key.productions): 
                        brick = key; break
                if brick is not None: trees.update(self.build(brick))
        return score(trees)
    #= CLASS -> LOOP
    def loop(self) -> None:
        for index in range(len(self.tokens) + 1):
            self.changed = True
            while self.changed:
                self.changed = False
                items = list(self.chart[index].values())
                for state in items:
                    match state.at():
                        case value if value is not None and ((isinstance(value, type) and issubclass(value, NonTerminal)) or isinstance(value, str)):
                            self.predict(state, index, value)
                        case value if value is not None and index < len(self.tokens) and isinstance(value, type) and issubclass(value, Token) and value in ORDER:
                            self.scan(state, index, value)
                        case value if state.full() and value is None:
                            self.complete(state, index, value)