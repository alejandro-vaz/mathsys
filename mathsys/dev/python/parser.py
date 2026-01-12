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
from .data import Start, NonTerminal, Level1, Level2, Level3, Level4, Level5
from .grammar import syntax, NONTERMINALS
from .issues import BrokenSyntax


#^
#^  RESOURCES
#^

#> RESOURCES -> TYPES
Rule = type[NonTerminal] | str
Key = tuple[Rule, tuple[Rule | type[Token], ...], int, int]

#> RESOURCES -> STATE
@dataclass(eq = False)
class State:
    rule: Rule
    productions: tuple[Rule | type[Token], ...]
    slot: int
    starting: int
    backpointers: set[tuple[Any, ...]] = field(default_factory = set)
    def at(self) -> Rule | type[Token] | None: 
        return self.productions[self.slot] if self.slot < len(self.productions) else None
    def full(self) -> bool: return self.slot >= len(self.productions)   
    def key(self) -> Key: return self.rule, self.productions, self.slot, self.starting
    def __hash__(self) -> int: return hash(self.key())
    def __eq__(self, other: Any) -> bool: return isinstance(other, State) and self.key() == other.key()
    def __repr__(self) -> str:
        before = " ".join(str(element) for element in self.productions[:self.slot])
        after = " ".join(str(element) for element in self.productions[self.slot:])
        return f"[{self.rule} -> {before} • {after}, start={self.starting}]"

#> RESOURCES -> GRAMMAR
class Grammar:
    productions: dict[Rule, tuple[tuple[str, ...]]]
    bnf: str
    def __init__(self, bnf: str) -> None: 
        self.bnf = bnf
        self.productions = self.convert()
    def convert(self) -> dict[Rule, tuple[tuple[str, ...]]]:
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

#> PARSER -> CLASS
class Parser:
    #= CLASS -> VARIABLES
    grammar: Grammar
    chart: list[dict[tuple, State]]
    changed: bool
    tokens: list[Token]
    #= CLASS -> INIT
    def __init__(self) -> None: self.grammar = Grammar(syntax); self.reset()
    #= CLASS -> RESET
    def reset(self) -> None:
        self.build.cache_clear()
        self.chart = []
        self.changed = True
        self.tokens = []
    #= CLASS -> RECALL
    def recall(self, at: int, rule: Rule, productions: tuple[Rule | type[Token], ...], slot: int, starting: int) -> State:
        key = (rule, productions, slot, starting)
        if not key in self.chart[at]: self.chart[at][key] = State(rule, productions, slot, starting)
        return self.chart[at][key]
    #= CLASS -> BUILD
    @cache
    def build(self, brick: tuple[Rule, tuple[str, ...], int, int]) -> list: 
        state = None
        for chart in self.chart:
            search = chart.get(brick)
            if search is not None: state = search; break
        if state is None or not state.full(): return []
        trees = []
        for backpointer in state.backpointers:
            children = []
            valid = True
            for item in backpointer:
                match item:
                    case Token(): 
                        if item.important(): children.append([item])
                    case State():
                        offspring = self.build(item.key())
                        if not offspring: valid = False; break
                        children.append(offspring)
            if not valid: continue
            for produce in product(*children): 
                flat = []
                for item in produce:
                    if isinstance(item, list): flat.extend(item)
                    else: flat.append(item)
                match state.rule:
                    case str(): trees.append(flat)
                    case type() if issubclass(state.rule, NonTerminal):
                        if state.rule in {Level1, Level2, Level3, Level4, Level5}: trees.append(flat[0])
                        else: trees.append(state.rule(flat))
        return trees
    #= CLASS -> PREDICT
    def predict(self, state: State, index: int, value: type[NonTerminal] | str) -> None:
        for production in self.grammar.productions[value]:
            if not (other := self.recall(index, value, production, 0, index)).backpointers:
                other.backpointers.add(tuple())
                self.changed = True
    #= CLASS -> SCAN
    def scan(self, state: State, index: int, value: type[Token]) -> None:
        if isinstance(self.tokens[index], value):
            other = self.recall(index + 1, state.rule, state.productions, state.slot + 1, state.starting)
            stored = len(other.backpointers)
            for backpointer in state.backpointers: other.backpointers.add(backpointer + (self.tokens[index],))
            if len(other.backpointers) > stored: self.changed = True
    #= CLASS -> COMPLETE
    def complete(self, state: State, index: int, value: None) -> None:
        for other in list(self.chart[state.starting].values()):
            if other.at() == state.rule:
                extra = self.recall(index, other.rule, other.productions, other.slot + 1, other.starting)
                stored = len(extra.backpointers)
                for backpointer in other.backpointers: 
                    for each in state.backpointers: extra.backpointers.add(backpointer + (state,))
                if len(extra.backpointers) > stored: self.changed = True
    #= CLASS -> RUN
    def run(self, tokens: list[Token]) -> Start:
        print(tokens)
        print(self.grammar)
        self.tokens = [token for token in tokens if token.compilable()]
        self.chart = [dict() for index in range(len(self.tokens) + 1)]
        self.recall(0, "$", (Start,), 0, 0).backpointers.add(tuple())
        self.loop()
        last = self.chart[len(self.tokens)].get(("$", (Start,), 1, 0))
        if last is None or not last.full(): raise BrokenSyntax()
        trees = []
        for backpointer in last.backpointers:
            if len(backpointer) != 1: continue
            if isinstance(backpointer[0], State): trees.extend(self.build(backpointer[0].key()))
        return trees[0]
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