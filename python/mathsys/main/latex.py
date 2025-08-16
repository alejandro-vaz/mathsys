#
#   HEAD
#

# HEAD -> DATACLASSES
from dataclasses import dataclass
from .parser import (
    # DATACLASSES -> 1ºLEVEL
    Level1,
    Sheet, 
    # DATACLASSES -> 2ºLEVEL
    Level2,
    Declaration,
    Node,
    Equation,
    Comment,
    # DATACLASSES -> 3ºLEVEL
    Level3,
    Expression, 
    # DATACLASSES -> 4ºLEVEL
    Level4,
    Term, 
    # DATACLASSES -> 5ºLEVEL
    Level5,
    Variable,
    Nest,
    Vector,
    Number
)


#
#   LATEX
#

# LATEX -> GENERATOR
class LaTeX:
    # GENERATOR -> VARIABLES
    latex: list[str]
    # GENERATOR -> INIT
    def __init__(self) -> None:
        self.latex = []
    # GENERATOR -> RUN
    def run(self, sheet: Level1) -> str:
        self.latex = []
        self.sheet(sheet)
        return ''.join([string for string in self.latex if string is not None])
    # GENERATOR -> 1 SHEET GENERATION
    def sheet(self, sheet: Sheet) -> None:
        match len(sheet.statements):
            case 0: 
                pass
            case 1:
                self.latex.append("$")
                match sheet.statements[0]:
                    case Declaration(): self.declaration(sheet.statements[0])
                    case Node(): self.node(sheet.statements[0])
                    case Equation(): self.equation(sheet.statements[0])
                    case Comment(): self.comment(sheet.statements[0])
                self.latex.append("$")
            case _:
                self.latex.append("$$")
                for statement in sheet.statements:
                    match statement:
                        case Declaration(): self.declaration(statement)
                        case Node(): self.node(statement)
                        case Equation(): self.equation(statement)
                        case Comment(): self.comment(statement)
                    self.latex.append(r"\\ ")
                self.latex.pop()
                self.latex.append("$$")
    # GENERATOR -> 2 DECLARATION GENERATION
    def declaration(self, declaration: Declaration) -> None:
        self.latex.append(declaration.identifier)
        self.latex.append("=")
        self.expression(declaration.expression)
    # GENERATOR -> 2 NODE GENERATION
    def node(self, node: Node) -> None:
        self.expression(node.expression)
    # GENERATOR -> 2 EQUATION GENERATION
    def equation(self, equation: Equation) -> None:
        self.expression(equation.left)
        self.latex.append("=")
        self.expression(equation.right)
    # GENERATOR -> 2 COMMENT GENERATION
    def comment(self, comment: Comment) -> None:
        self.latex.append(r"\text{")
        self.latex.append(comment.text)
        self.latex.append(r"}")
    # GENERATOR -> 3 EXPRESSION GENERATION
    def expression(self, expression: Expression) -> None:
        for index in range(len(expression.terms)): 
            self.term(expression.terms[index], index == 0)
    # GENERATOR -> 4 TERM GENERATION
    def term(self, term: Term, noTermSign: bool) -> None:
        numerator = []
        denominator = []
        for index in range(len(term.factors)):
            if index == 0: numerator.append(term.factors[0]); continue
            match term.operators[index - 1]:
                case "*" | "·": numerator.append(term.factors[index])
                case "/": denominator.append(term.factors[index])
        if denominator:
            for index in range(len(numerator)):
                match numerator[index]:
                    case Variable(): self.variable(numerator[index], noTermSign or index != 0, index == 0)
                    case Nest(): self.nest(numerator[index], noTermSign or index != 0, index == 0)
                    case Vector(): self.vector(numerator[index], noTermSign or index != 0, index == 0)
                    case Number(): self.number(numerator[index], noTermSign or index != 0, index == 0)
                self.latex.append(r"\cdot ")
            self.latex.pop()
            self.latex.append(r"}{")
            for index in range(len(denominator)):
                match denominator[index]:
                    case Variable(): self.variable(denominator[index], True, False)
                    case Nest(): self.nest(denominator[index], True, False)
                    case Vector(): self.vector(denominator[index], True, False)
                    case Number(): self.number(denominator[index], True, False)
                self.latex.append(r"\cdot ")
            self.latex.pop()
            self.latex.append(r"}")
        else:
            for index in range(len(numerator)):
                match numerator[index]:
                    case Variable(): self.variable(numerator[index], noTermSign or index != 0, False)
                    case Nest(): self.nest(numerator[index], noTermSign or index != 0, False)
                    case Vector(): self.vector(numerator[index], noTermSign or index != 0, False)
                    case Number(): self.number(numerator[index], noTermSign or index != 0, False)
                self.latex.append(r"\cdot ")
            self.latex.pop()
    # GENERATOR -> 5 VARIABLE GENERATION
    def variable(self, variable: Variable, noSign: bool, createFraction: bool) -> None:
        if noSign: 
            self.latex.append(variable.signs)
        else: 
            self.latex.append(variable.signs if variable.signs is not None else "+")
        if createFraction: 
            self.latex.append(r"\frac")
        self.latex.append(variable.representation)
        if variable.exponent is not None:
            self.latex.append(r"^{")
            self.expression(variable.exponent)
            self.latex.append(r"}")
    # GENERATOR -> 5 NEST GENERATION
    def nest(self, nest: Nest, noSign: bool, createFraction: bool) -> None:
        if noSign:
            self.latex.append(nest.signs)
        else:
            self.latex.append(nest.signs if nest.signs is not None else "+")
        if createFraction:
            self.latex.append(r"\frac")
        self.latex.append(r"\left( ")
        self.expression(nest.expression)
        self.latex.append(r"\right) ")
        if nest.exponent is not None:
            self.latex.append(r"^{")
            self.expression(nest.exponent)
            self.latex.append(r"}")
    # GENERATOR -> 5 VECTOR GENERATION
    def vector(self, vector: Vector, noSign: bool, createFraction: bool) -> None:
        if noSign:
            self.latex.append(vector.signs)
        else:
            self.latex.append(vector.signs if vector.signs is not None else "+")
        if createFraction:
            self.latex.append(r"\frac{")
        self.latex.append(r"\begin{bmatrix}")
        if vector.values:
            for expression in vector.values:
                self.expression(expression)
                self.latex.append(r"\\ ")
            self.latex.pop()
        else:
            self.latex.append(r"\; ")
        self.latex.append(r"\end{bmatrix}")
        if vector.exponent is not None:
            self.latex.append(r"^{")
            self.expression(vector.exponent)
            self.latex.append(r"}")
    # GENERATOR -> 5 NUMBER GENERATION
    def number(self, number: Number, noSign: bool, createFraction: bool) -> None:
        if noSign:
            self.latex.append(number.signs)
        else:
            self.latex.append(number.signs if number.signs is not None else "+")
        if createFraction:
            self.latex.append(r"\frac{")
        self.latex.append(number.representation)
        if number.exponent is not None:
            self.latex.append(r"^{")
            self.expression(number.exponent)
            self.latex.append(r"}")