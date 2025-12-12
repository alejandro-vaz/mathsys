#^
#^  HEAD
#^

#> HEAD -> MODULES
from lark import Lark, Transformer, Token

#> HEAD -> DATA
from .local import MODULES, ñ, SYNTAX
from . import dataclasses as parser


#^
#^  PARSER
#^

#> PARSER -> CLASS
class Parser(Transformer):
    #~ CLASS -> VARIABLES
    lark: Lark
    #~ CLASS -> INIT
    def __init__(self) -> None: super(); self.lark = Lark(SYNTAX, parser="earley")
    #~ CLASS -> RUN
    def run(self, content: str) -> parser.Start: return self.transform(self.lark.parse(content))
    #~ CLASS -> LEVEL 1
    def level1(self, items: list[parser.Level1]) -> parser.Level1: return items[0]
    #~ CLASS -> LEVEL 2
    def level2(self, items: list[parser.Level2]) -> parser.Level2: return items[0]
    #~ CLASS -> LEVEL 3
    def level3(self, items: list[parser.Level3]) -> parser.Level3: return items[0]
    #~ CLASS -> LEVEL 4
    def level4(self, items: list[parser.Level4]) -> parser.Level4: return items[0]
    #~ CLASS -> LEVEL 5
    def level5(self, items: list[parser.Level5]) -> parser.Level5: return items[0]
    #~ CLASS -> START CONSTRUCT
    def start(self, items: list[parser.Level1]) -> parser.Start: 
        return parser.Start(
            statements = items
        )
    #~ CLASS -> 1 DECLARATION CONSTRUCT
    def declaration(self, items: list[Token | parser.Variable | parser.Expression]) -> parser.Declaration: 
        return parser.Declaration(
            group = ñ(items[0]) if len(items) == 3 else None,
            variable = items[1] if len(items) == 3 else items[0],
            expression = items[2] if len(items) == 3 else items[1]
        )
    #~ CLASS -> 1 DEFINITION CONSTRUCT
    def definition(self, items: list[Token | parser.Variable | parser.Expression]) -> parser.Definition: 
        return parser.Definition(
            group = ñ(items[0]) if len(items) == 3 else None,
            variable = items[1] if len(items) == 3 else items[0],
            expression = items[2] if len(items) == 3 else items[1]
        )
    #~ CLASS -> 1 ANNOTATION CONSTRUCT
    def annotation(self, items: list[Token | parser.Variable]) -> parser.Annotation:
        return parser.Annotation(
            group = ñ(items[0]),
            variables = [item for item in items if isinstance(item, parser.Variable)]
        )
    #~ CLASS -> 1 NODE CONSTRUCT
    def node(self, items: list[parser.Expression]) -> parser.Node: 
        return parser.Node(
            expression = items[0]
        )
    #~ CLASS -> 1 EQUATION CONSTRUCT
    def equation(self, items: list[parser.Expression]) -> parser.Equation: 
        return parser.Equation(
            leftexpression = items[0],
            rightexpression = items[1]
        )
    #~ CLASS -> 1 COMMENT CONSTRUCT
    def comment(self, items: list[Token]) -> parser.Comment:
        return parser.Comment(
            text = items[0].value
        )
    #~ CLASS -> 1 USE CONSTRUCT
    def use(self, items: list[Token]) -> parser.Use:
        return parser.Use(
            name = ñ(items[0]),
            start = self.run(MODULES.get(ñ(items[0]), None)) if MODULES.get(ñ(items[0]), None) is not None else None
        )
    #~ CLASS -> 2 EXPRESSION CONSTRUCT
    def expression(self, items: list[Token | parser.Level3]) -> parser.Expression:
        return parser.Expression(
            signs = ([] if isinstance(items[0], Token) else [None]) + [ñ(item) for item in items if isinstance(item, Token)],
            terms = [item for item in items if isinstance(item, parser.Level3)]
        )
    #~ CLASS -> 3 TERM CONSTRUCT
    def term(self, items: list[Token | parser.Level4]) -> parser.Term:
        numerator = []
        denominator = []
        location = True
        for item in items:
            if isinstance(item, Token):
                match ñ(item):
                    case "*": location = True
                    case "/": location = False
            else: numerator.append(item) if location else denominator.append(item)
        return parser.Term(
            numerator = numerator,
            denominator = denominator
        )
    #~ CLASS -> 4 FACTOR CONSTRUCT
    def factor(self, items: list[parser.Level5 | parser.Expression]) -> parser.Factor:
        return parser.Factor(
            value = items[0],
            exponent = items[1] if len(items) == 2 else None
        )
    #~ CLASS -> 4 LIMIT CONSTRUCT
    def limit(self, items: list[parser.Variable | parser.Expression | Token | parser.Nest]) -> parser.Limit:
        return parser.Limit(
            variable = items[0],
            approach = items[1],
            direction = ñ(items[2]) == "+" if isinstance(items[2], Token) else None,
            nest = items[-2] if isinstance(items[-2], parser.Nest) else items[-1],
            exponent = items[-1] if isinstance(items[-1], parser.Expression) else None
        )
    #~ CLASS -> 5 INFINITE CONSTRUCT
    def infinite(self, items: list) -> parser.Infinite:
        return parser.Infinite()
    #~ CLASS -> 5 VARIABLE CONSTRUCT
    def variable(self, items: list[Token]) -> parser.Variable:
        return parser.Variable(
            representation = ñ(items[0])
        )
    #~ CLASS -> 5 NEST CONSTRUCT
    def nest(self, items: list[parser.Expression]) -> parser.Nest:
        return parser.Nest(
            expression = items[0] if len(items) == 1 else None
        )
    #~ CLASS -> 5 TENSOR CONSTRUCT
    def tensor(self, items: list[parser.Expression]) -> parser.Tensor:
        return parser.Tensor(
            values = items
        )
    #~ CLASS -> 5 NUMBER CONSTRUCT
    def number(self, items: list[Token]) -> parser.Number:
        return parser.Number(
            value = int(ñ(items[0]) + (ñ(items[1]) if len(items) == 2 else "0")),
            shift = len(str(int((ñ(items[1]) if len(items) == 2 else "0")[::-1])))
        )