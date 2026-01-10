#^
#^  HEAD
#^

#> HEAD -> MODULES
from dataclasses import dataclass, field
from typing import Any
from collections import defaultdict
from itertools import product
from functools import cache

#> HEAD -> DAT
from ebnf import Extensor
from tokenizer import Tokenizer, Token, ORDER
from grammar import NonTerminal, syntax


#^
#^  RESOURCES
#^

#> RESOURCES -> STATE
@dataclass(eq = False)
class State:
    rule: str
    productions: tuple[str, ...]
    slot: int
    starting: int
    backpointers: set[tuple[Any, ...]] = field(default_factory = set)
    def at(self) -> str | None:
        if self.slot < len(self.productions): return self.productions[self.slot]
        return None
    def full(self) -> bool:
        return self.slot >= len(self.productions)
    def key(self) -> tuple[str, tuple[str, ...], int, int]: 
        return self.rule, self.productions, self.slot, self.starting
    def __hash__(self) -> int: return hash(self.key())
    def __eq__(self, other: Any) -> bool: return isinstance(other, State) and self.key() == other.key()
    def __repr__(self) -> str:
        before = " ".join(self.productions[:self.slot])
        after = " ".join(self.productions[self.slot:])
        return f"[{self.rule} -> {before} • {after}, start={self.starting}]"

#> RESOURCES -> GRAMMAR
class Grammar:
    productions: dict[str, tuple[tuple[str, ...]]]
    ebnf: str
    def __init__(self, ebnf: str) -> None: 
        self.ebnf = ebnf
        self.productions = self.convert()
    def convert(self) -> dict[str, tuple[tuple[str, ...]]]:
        syntax = defaultdict(list)
        for line in [line.strip() for line in self.rebase().splitlines()]:
            rule, productions = [part.strip() for part in line.split("->", 1)]
            for variant in [variant.strip() for variant in productions.split("|")]:
                syntax[rule].append(tuple([] if not variant else variant.split()))
        frozen = {}
        for key, value in syntax.items(): frozen[key] = tuple(value)
        frozen["$"] = ("Start",)
        return frozen
    def rebase(self) -> str: return Extensor().run(self.ebnf)
    def __repr__(self) -> str: return self.ebnf

#> RESOURCES -> PARSE
@dataclass(kw_only = True)
class Parse:
    tree: tuple | None
    chart: list[dict[tuple, State]]
    def __repr__(self) -> str: return str(self.tree)


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
    def __init__(self, grammar: Grammar) -> None: self.grammar = grammar; self.reset()
    #= CLASS -> RESET
    def reset(self) -> None:
        self.build.cache_clear()
        self.chart = []
        self.changed = True
        self.tokens = []
    #= CLASS -> RECALL
    def recall(self, at: int, rule: str, productions: tuple[str, ...], slot: int, starting: int) -> State:
        key = (rule, productions, slot, starting)
        if not key in self.chart[at]: self.chart[at][key] = State(rule, productions, slot, starting)
        return self.chart[at][key]
    #= CLASS -> BUILD
    @cache
    def build(self, brick: tuple[str, tuple[str, ...], int, int]) -> list:
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
                    case Token(): children.append([item])
                    case State():
                        offspring = self.build(item.key())
                        if not offspring: valid = False; break
                        children.append(offspring)
            if not valid: continue
            for produce in product(*children): trees.append((state.rule, list(produce)))
        return trees
    #= CLASS -> PREDICT
    def predict(self, state: State, index: int, value: str) -> None:
        for production in self.grammar.productions[value]:
            if not (other := self.recall(index, value, production, 0, index)).backpointers:
                other.backpointers.add(tuple())
                self.changed = True
    #= CLASS -> SCAN
    def scan(self, state: State, index: int, value: str) -> None:
        if self.tokens[index].__class__.__name__ == value:
            other = self.recall(index + 1, state.rule, state.productions, state.slot + 1, state.starting)
            stored = len(other.backpointers)
            for backpointer in state.backpointers:
                other.backpointers.add(backpointer + (self.tokens[index],))
            if len(other.backpointers) > stored: self.changed = True
    #= CLASS -> COMPLETE
    def complete(self, state: State, index: int, value: str | None) -> None:
        for other in list(self.chart[state.starting].values()):
            if other.at() == state.rule:
                extra = self.recall(index, other.rule, other.productions, other.slot + 1, other.starting)
                stored = len(extra.backpointers)
                for backpointer in other.backpointers:
                    for each in state.backpointers: extra.backpointers.add(backpointer + (state,))
                if len(extra.backpointers) > stored: self.changed = True
    #= CLASS -> PARSE
    def parse(self, tokens: list[Token]) -> Parse:
        self.tokens = tokens
        self.chart = [dict() for index in range(len(self.tokens) + 1)]
        self.recall(0, "$", ('Start',), 0, 0).backpointers.add(tuple())
        for index in range(len(self.tokens) + 1):
            self.changed = True
            while self.changed:
                self.changed = False
                items = list(self.chart[index].values())
                for state in items:
                    match state.at():
                        case value if value is not None and value in self.grammar.productions:
                            self.predict(state, index, value)
                        case value if value is not None and index < len(self.tokens) and value in [item.__name__ for item in ORDER]:
                            self.scan(state, index, value)
                        case value if state.full():
                            self.complete(state, index, value)
        if (last := self.chart[len(tokens)].get(("$", ('Start',), 1, 0))) is None or not last.full(): return Parse(
            tree = None, 
            chart = self.chart
        )
        trees = []
        for backpointer in last.backpointers:
            if len(backpointer) != 1: continue
            if isinstance(backpointer[0], State): trees.extend(self.build(backpointer[0].key()))
        return Parse(
            tree = trees[0],
            chart = self.chart
        )

# Example 3: small expression grammar
tokens = Tokenizer().run("use \"module\"")
print("tokens =", tokens)
grammar = Grammar(syntax)
print(grammar.rebase())
parser = Parser(grammar)
res3 = parser.parse(tokens)

print("Accepted:", res3.tree is not None)
print(res3)