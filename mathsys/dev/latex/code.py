#^
#^  HEAD
#^

#> HEAD -> DATA
from .local import types
from . import dataclasses as latex
from ..parser import dataclasses as parser


#^
#^  LATEX
#^

#> LATEX -> GENERATOR
class LaTeX:
    #~ GENERATOR -> INIT
    def __init__(self) -> None: pass
    #~ GENERATOR -> RUN
    def run(self, start: parser.Start) -> str: types.clear(); return self.start(start)
    #~ GENERATOR -> START GENERATION
    def start(self, start: parser.Start) -> str:
        return str(latex.Start(
            statements = [self.level1(statement) for statement in start.statements if self.level1(statement)]
        ))
    #~ GENERATOR -> 1 LEVEL GENERATION
    def level1(self, level1: parser.Level1) -> str:
        match level1:
            case parser.Declaration(): return self.declaration(level1)
            case parser.Definition(): return self.definition(level1)
            case parser.Annotation(): return self.annotation(level1)
            case parser.Node(): return self.node(level1)
            case parser.Equation(): return self.equation(level1)
            case parser.Comment(): return self.comment(level1)
            case parser.Use(): return self.use(level1)
    #~ GENERATOR -> 1 DECLARATION GENERATION
    def declaration(self, declaration: parser.Declaration) -> str:
        if declaration.group is not None: types[declaration.variable.representation] = types.get(declaration.variable.representation, declaration.group)
        return str(latex.Declaration(
            group = declaration.group if declaration.group is not None else "",
            variable = self.variable(declaration.variable),
            expression = self.expression(declaration.expression)
        ))
    #~ GENERATOR -> 1 DEFINITION GENERATION
    def definition(self, definition: parser.Definition) -> str:
        if definition.group is not None: types[definition.variable.representation] = types.get(definition.variable.representation, definition.group)
        return str(latex.Definition(
            group = definition.group if definition.group is not None else "",
            variable = self.variable(definition.variable),
            expression = self.expression(definition.expression)
        ))
    #~ GENERATOR -> 1 ANNOTATION GENERATION
    def annotation(self, annotation: parser.Annotation) -> str:
        if annotation.group is not None: 
            for variable in annotation.variables:
                types[variable.representation] = types.get(variable.representation, annotation.group)
        return str(latex.Annotation(
            group = annotation.group if annotation.group is not None else "",
            variables = [self.variable(variable) for variable in annotation.variables]
        ))
    #~ GENERATOR -> 1 NODE GENERATION
    def node(self, node: parser.Node) -> str:
        return str(latex.Node(
            expression = self.expression(node.expression)
        ))
    #~ GENERATOR -> 1 EQUATION GENERATION
    def equation(self, equation: parser.Equation) -> str:
        return str(latex.Equation(
            leftexpression = self.expression(equation.leftexpression),
            rightexpression = self.expression(equation.rightexpression)
        ))
    #~ GENERATOR -> 1 COMMENT GENERATION
    def comment(self, comment: parser.Comment) -> str:
        return str(latex.Comment(
            text = comment.text
        ))
    #~ GENERATOR -> 1 USE GENERATION
    def use(self, use: parser.Use) -> str:
        return str(latex.Use(
            name = use.name,
            start = use.start is not None
        ))
    #~ GENERATOR -> 2 LEVEL GENERATION
    def level2(self, level2: parser.Level2) -> str:
        match level2:
            case parser.Expression(): return self.expression(level2)
    #~ GENERATOR -> 2 EXPRESSION GENERATION
    def expression(self, expression: parser.Expression) -> str:
        return str(latex.Expression(
            signs = [sign if sign is not None else "" for sign in expression.signs],
            terms = [self.level3(term) for term in expression.terms]
        ))
    #~ GENERATOR -> 3 LEVEL GENERATION
    def level3(self, level3: parser.Level3) -> str:
        match level3:
            case parser.Term(): return self.term(level3)
    #~ GENERATOR -> 3 TERM GENERATION
    def term(self, term: parser.Term) -> str:
        numerator = []
        for index in range(len(term.numerator)):
            value = self.level4(term.numerator[index])
            if index != 0:
                if isinstance(term.numerator[index - 1], parser.Factor):
                    if isinstance(term.numerator[index - 1].value, parser.Number):
                        if isinstance(term.numerator[index], parser.Factor):
                            if isinstance(term.numerator[index].value, parser.Number | parser.Infinite):
                                value = r"\cdot " + value
                    else: value = r"\cdot " + value
                else: value = r"\cdot " + value
            numerator.append(value)
        denominator = []
        for index in range(len(term.denominator)):
            value = self.level4(term.denominator[index])
            if index != 0:
                if isinstance(term.denominator[index - 1], parser.Factor):
                    if isinstance(term.denominator[index - 1].value, parser.Number):
                        if isinstance(term.denominator[index], parser.Factor):
                            if isinstance(term.denominator[index].value, parser.Number | parser.Infinite):
                                value = r"\cdot " + value
                    else: value = r"\cdot " + value
                else: value = r"\cdot " + value
            denominator.append(value)
        return str(latex.Term(
            numerator = numerator,
            denominator = denominator
        ))
    #~ GENERATOR -> 4 LEVEL GENERATION
    def level4(self, level4: parser.Level4) -> str:
        match level4:
            case parser.Factor(): return self.factor(level4)
            case parser.Limit(): return self.limit(level4)
    #~ GENERATOR -> 4 FACTOR GENERATION
    def factor(self, factor: parser.Factor) -> str:
        return str(latex.Factor(
            value = self.level5(factor.value),
            exponent = self.expression(factor.exponent) if factor.exponent is not None else None
        ))
    #~ GENERATOR -> 4 LIMIT GENERATION
    def limit(self, limit: parser.Limit) -> str:
        return str(latex.Limit(
            variable = self.variable(limit.variable),
            approach = self.expression(limit.approach),
            direction = limit.direction,
            nest = self.nest(limit.nest),
            exponent = self.expression(limit.exponent) if limit.exponent is not None else None
        ))
    #~ GENERATOR -> 5 LEVEL GENERATION
    def level5(self, level5: parser.Level5) -> str:
        match level5:
            case parser.Infinite(): return self.infinite(level5)
            case parser.Variable(): return self.variable(level5)
            case parser.Nest(): return self.nest(level5)
            case parser.Tensor(): return self.tensor(level5)
            case parser.Number(): return self.number(level5)
    #~ GENERATOR -> 5 INFINITE GENERATION
    def infinite(self, infinite: parser.Infinite) -> str: 
        return str(latex.Infinite())
    #~ GENERATOR -> 5 VARIABLE GENERATION
    def variable(self, variable: parser.Variable) -> str:
        return str(latex.Variable(
            representation = variable.representation
        ))
    #~ GENERATOR -> 5 NEST GENERATION
    def nest(self, nest: parser.Nest) -> str:
        return str(latex.Nest(
            expression = self.expression(nest.expression) if nest.expression is not None else ""
        ))
    #~ GENERATOR -> 5 TENSOR GENERATION
    def tensor(self, vector: parser.Tensor) -> str:
        return str(latex.Tensor(
            values = [self.expression(value) for value in vector.values]
        ))
    #~ GENERATOR -> 5 NUMBER GENERATION
    def number(self, number: parser.Number) -> str:
        return str(latex.Number(
            value = number.value,
            shift = number.shift
        ))