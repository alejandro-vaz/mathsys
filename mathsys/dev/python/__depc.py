#^
#^  HEAD
#^

#> HEAD -> MODULES
from lark import Lark, Transformer, Token
from typing import cast
from asyncio import gather

#> HEAD -> DATA
from .parser import read, ñ, SYNTAX
from . import data


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
    async def run(self, content: str) -> data.Start: 
        entry = cast(data.Start, self.transform(self.lark.parse(content)))
        uses = [item for item in entry.stream if isinstance(item, data.Use)]
        tofetch = dict(zip((item.name for item in uses), await gather(*(read(item.name) for item in uses))))
        return data.Start(stream = [item if not isinstance(item, data.Use) else data.Use(
            name = item.name,
            start = await self.run(tofetch[item.name])
        ) for item in entry.stream])
    #~ CLASS -> LEVEL 1
    def level1(self, items: list[data.Level1]) -> data.Level1: return items[0]
    #~ CLASS -> LEVEL 2
    def level2(self, items: list[data.Level2]) -> data.Level2: return items[0]
    #~ CLASS -> LEVEL 3
    def level3(self, items: list[data.Level3]) -> data.Level3: return items[0]
    #~ CLASS -> LEVEL 4
    def level4(self, items: list[data.Level4]) -> data.Level4: return items[0]
    #~ CLASS -> LEVEL 5
    def level5(self, items: list[data.Level5]) -> data.Level5: return items[0]
    #~ CLASS -> START CONSTRUCT
    def start(self, items: list[data.Level1 | Token]) -> data.Start: 
        return data.Start(
            stream = [item if isinstance(item, data.Level1) else item[1:].strip() for item in items]
        )
    #~ CLASS -> 1 DECLARATION CONSTRUCT
    def declaration(self, items: list[Token | data.Variable | data.Level2]) -> data.Declaration: 
        return data.Declaration(
            group = ñ(cast(Token, items[0])) if len(items) == 3 else None,
            variable = cast(data.Variable, items[1] if len(items) == 3 else items[0]),
            value = cast(data.Level2, items[2] if len(items) == 3 else items[1])
        )
    #~ CLASS -> 1 DEFINITION CONSTRUCT
    def definition(self, items: list[Token | data.Variable | data.Level2]) -> data.Definition: 
        return data.Definition(
            group = ñ(cast(Token, items[0])) if len(items) == 3 else None,
            variable = cast(data.Variable, items[1] if len(items) == 3 else items[0]),
            value = cast(data.Level2, items[2] if len(items) == 3 else items[1])
        )
    #~ CLASS -> 1 ANNOTATION CONSTRUCT
    def annotation(self, items: list[Token | data.Variable]) -> data.Annotation:
        return data.Annotation(
            group = ñ(cast(Token, items[0])),
            variables = cast(list[data.Variable], items[1:])
        )
    #~ CLASS -> 1 NODE CONSTRUCT
    def node(self, items: list[data.Level2]) -> data.Node: 
        return data.Node(
            value = items[0]
        )
    #~ CLASS -> 1 EQUATION CONSTRUCT
    def equation(self, items: list[data.Level2]) -> data.Equation: 
        return data.Equation(
            leftside = items[0],
            rightside = items[1]
        )
    #~ CLASS -> 1 USE CONSTRUCT
    def use(self, items: list[Token]) -> data.Use:
        return data.Use(
            name = ñ(items[0])[1:-1],
            start = None
        )
    #~ CLASS -> 2 EXPRESSION CONSTRUCT
    def expression(self, items: list[Token | data.Level3]) -> data.Expression:
        signs = []
        for index in range(len(items)):
            if isinstance(items[index], data.Level3): 
                if index == 0: signs.append([])
                continue
            if index == 0: signs.append([ñ(cast(Token, items[index]))])
            if isinstance(items[index - 1], Token):
                signs[-1].append(ñ(cast(Token, items[index])))
            else:
                signs.append([ñ(cast(Token, items[index]))])
        return data.Expression(
            signs = signs,
            terms = [item for item in items if isinstance(item, data.Level3)]
        )
    #~ CLASS -> 3 TERM CONSTRUCT
    def term(self, items: list[Token | data.Level4]) -> data.Term:
        numerator = []
        denominator = []
        location = True
        for item in items:
            if isinstance(item, Token):
                match ñ(item):
                    case "*": location = True
                    case "/": location = False
            else: numerator.append(item) if location else denominator.append(item)
        return data.Term(
            numerator = numerator,
            denominator = denominator
        )
    #~ CLASS -> 4 FACTOR CONSTRUCT
    def factor(self, items: list[data.Level5 | data.Level2]) -> data.Factor:
        return data.Factor(
            value = cast(data.Level5, items[0]),
            exponent = cast(data.Level2, items[1]) if len(items) == 2 else None
        )
    #~ CLASS -> 4 LIMIT CONSTRUCT
    def limit(self, items: list[data.Variable | data.Level2 | Token | data.Nest]) -> data.Limit:
        return data.Limit(
            variable = cast(data.Variable, items[0]),
            approach = cast(data.Level2, items[1]),
            direction = ñ(items[2]) == "+" if isinstance(items[2], Token) else None,
            nest = cast(data.Nest, items[-2] if isinstance(items[-2], data.Nest) else items[-1]),
            exponent = items[-1] if isinstance(items[-1], data.Level2) else None
        )
    #~ CLASS -> 5 INFINITE CONSTRUCT
    def infinite(self, items: list) -> data.Infinite:
        return data.Infinite()
    #~ CLASS -> 5 VARIABLE CONSTRUCT
    def variable(self, items: list[Token]) -> data.Variable:
        return data.Variable(
            representation = ñ(items[0])
        )
    #~ CLASS -> 5 NEST CONSTRUCT
    def nest(self, items: list[data.Level2]) -> data.Nest:
        return data.Nest(
            value = items[0] if len(items) == 1 else None
        )
    #~ CLASS -> 5 TENSOR CONSTRUCT
    def tensor(self, items: list[data.Level2]) -> data.Tensor:
        return data.Tensor(
            values = items
        )
    #~ CLASS -> 5 WHOLE CONSTRUCT
    def whole(self, items: list[Token]) -> data.Whole:
        return data.Whole(
            value = int(ñ(items[0]))
        )
    #~ CLASS -> 5 ABSOLUTE CONSTRUCT
    def absolute(self, items: list[data.Level2]) -> data.Absolute:
        return data.Absolute(
            value = items[0]
        )
    #~ CLASS -> 5 UNDEFINED CONSTRUCT
    def undefined(self, items: list[Token]) -> data.Undefined:
        return data.Undefined()
    #~ CLASS -> 5 RATIONAL CONSTRUCT
    def rational(self, items: list[Token]) -> data.Rational:
        return data.Rational(
            whole = int(ñ(items[0]).split(".")[0]),
            decimal = int(ñ(items[0]).split(".")[1].rstrip("0")[::-1])
        )
    #~ CLASS -> 5 CASTS CONSTRUCT
    def casts(self, items: list[data.Level5 | Token]) -> data.Casts:
        return data.Casts(
            element = cast(data.Level5, items[0]),
            to = ñ(cast(Token, items[1]))
        )