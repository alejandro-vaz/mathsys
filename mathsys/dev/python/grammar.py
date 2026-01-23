#^
#^  HEAD
#^

#> HEAD -> MODULES
from re import search

#> HEAD -> DATA
from .issues import BrokenSyntax
from .nonterminal import NonTerminal
from .start import Start
from .level1 import Declaration, Definition, Annotation, Node, Equation, Use, Level1
from .level2 import Expression, Level2
from .level3 import Term, Level3
from .level4 import Factor, Limit, Level4
from .level5 import Infinite, Variable, Nest, Tensor, Whole, Absolute, Undefined, Rational, Casts, Level5


#^
#^  EBNF
#^

#> EBNF -> EXTENSOR
class Extensor:
    counter: int
    parentheses: dict[str, str]
    more: dict[str, str]
    multiple: dict[str, str]
    optional: dict[str, str]
    def __init__(self) -> None: self.reset()
    def reset(self) -> None:
        self.counter = 0
        self.parentheses = {}
        self.more = {}
        self.multiple = {}
        self.optional = {}
    def run(self, ebnf: str) -> str:
        self.reset()
        out = set()
        for line in [line.strip() for line in ebnf.splitlines()]:
            if not line or line.startswith("#") or line.startswith("//"): continue
            rule, productions = [part.strip() for part in line.split("->", 1)]
            body, rules = self.expand(productions)
            out.add(f"{rule} -> {body}")
            out.update(rules)
        ordered = list(out)
        ordered.sort(key = lambda line: (1, int(rule[1:])) if (rule := line.split("->", 1)[0].strip()).startswith("$") else (0, rule))
        return "\n".join(ordered)
    def expand(self, expression: str) -> tuple[str, set[str]]:
        rules = set()
        while "(" in expression: expression = self.collapse(expression, rules)
        expression = self.postfix(expression, rules)
        return expression.strip(), rules
    def collapse(self, expression: str, rules: set[str]) -> str:
        hit = search(r"\(([^()]+)\)", expression)
        if not hit: return expression
        inner = hit.group(1)
        symbol = self.parentheses[inner] = self.parentheses[inner] if inner in self.parentheses else self.next()
        expanded, newrules = self.expand(inner)
        rules.update(newrules)
        rules.add(f"{symbol} -> {expanded}")
        return expression[:hit.start()] + symbol + expression[hit.end():]
    def postfix(self, expression: str, rules: set[str]) -> str:
        while hit := search(r"(?P<atom>\$[0-9]+|[A-Z][a-z]*[0-9]*|_?[A-Z]+)(?P<operator>[*+?])", expression):
            atom = hit.group("atom")
            match hit.group("operator"):
                case "+": 
                    symbol = self.more[atom] = self.more[atom] if atom in self.more else self.next()
                    production = f"{symbol} -> {atom} {symbol} | {atom}"
                case "*": 
                    symbol = self.multiple[atom] = self.multiple[atom] if atom in self.multiple else self.next()
                    production = f"{symbol} -> {atom} {symbol} |"
                case "?":
                    symbol = self.optional[atom] = self.optional[atom] if atom in self.optional else self.next()
                    production = f"{symbol} -> {atom} |"
            expression = expression[:hit.start()] + symbol + expression[hit.end():]
            rules.add(production)
        return expression
    def next(self) -> str: self.counter += 1; return f"${self.counter}"


#^
#^  NONTERMINALS
#^

#> NONTERMINALS -> ALL
NONTERMINALS = {item.__name__: item for item in {
    Start,
    Declaration,
    Definition,
    Annotation,
    Node,
    Equation,
    Use,
    Expression,
    Term,
    Factor,
    Limit,
    Infinite,
    Variable,
    Nest,
    Tensor,
    Whole,
    Absolute,
    Undefined,
    Rational,
    Casts,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5
}}


#^
#^  SYNTAX
#^

SYNTAX = Extensor().run(r"""
#> SYNTAX -> START
Start -> (_NEWLINES? Level1 _SPACES? (_NEWLINES Level1 _SPACES?)*)? _NEWLINES? _EOF

#> SYNTAX -> 1ºLEVEL
Declaration -> (TYPE _SPACES)? Variable _SPACES? _EQUALITY _SPACES? Level2
Definition -> (TYPE _SPACES)? Variable _SPACES? _BINDING _SPACES? Level2
Annotation -> TYPE _SPACES Variable (_SPACES? _COMMA _SPACES? Variable)*
Node -> Level2
Equation -> Level2 _SPACES? _EQUALITY _SPACES? Level2
Use -> _USE _SPACES MODULE

#> SYNTAX -> 2ºLEVEL
Expression -> (SIGN _SPACES?)* Level3 ((_SPACES? SIGN)+ _SPACES? Level3)*

#> SYNTAX -> 3ºLEVEL
Term -> Level4 ((_SPACES? OPERATOR)? _SPACES? Level4)*

#> SYNTAX -> 4ºLEVEL
Factor -> Level5 (_EXPONENTIATION _SPACES? Level2 _SPACES? _EXPONENTIATION)?
Limit -> _LIM _SPACES Variable _SPACES? _TO _SPACES? Level2 SIGN? _SPACES _OF _SPACES Nest (_EXPONENTIATION _SPACES? Level2 _SPACES? _EXPONENTIATION)?

#> SYNTAX -> 5ºLEVEL
Infinite -> _INF
Variable -> IDENTIFIER
Nest -> _OPEN _SPACES? Level2? _SPACES? _CLOSE
Tensor -> _ENTER _SPACES? (Level2 (_SPACES? _COMMA _SPACES? Level2)* _SPACES?)? _EXIT
Whole -> NUMBER
Absolute -> _PIPE _SPACES? Level2 _SPACES? _PIPE
Undefined -> _UNDEFINED
Rational -> RATIONAL
Casts -> Level5 TYPE

#> SYNTAX -> LEVELS
Level1 -> Declaration | Definition | Annotation | Node | Equation | Use
Level2 -> Expression
Level3 -> Term
Level4 -> Factor | Limit
Level5 -> Infinite | Variable | Nest | Tensor | Whole | Absolute | Undefined | Rational | Casts
""")

#> SYNTAX -> SCORE
def score(symbol: type[NonTerminal] | str) -> int: return -{
    Declaration: 1,
    Equation: 0
}.get(symbol, 0)