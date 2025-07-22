#
#   HEAD
#

# HEAD -> DATACLASSES
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
    Factor, 
    # DATACLASSES -> 6ºLEVEL
    Level6,
    Vector
)


#
#   GENERATOR
#

# GENERATOR -> CLASS
class LaTeX:
    # CLASS -> VARIABLES
    latex: list[str]
    # CLASS -> INIT
    def __init__(self) -> None:
        self.latex = []
    # CLASS -> RUN
    def run(self, sheet: Level1) -> str:
        self.sheet(sheet)
        return ''.join(self.latex)
    # CLASS -> 1 SHEET GENERATION
    def sheet(self, sheet: Sheet) -> None:
        match sheet.type:
            case ["empty"]: return
            case ["inline"]:
                self.latex.append("$")
                match sheet.statements[0]:
                    case Declaration(): self.declaration(sheet.statements[0])
                    case Node(): self.node(sheet.statements[0])
                    case Equation(): self.equation(sheet.statements[0])
                    case Comment(): self.comment(sheet.statements[0])
                self.latex.append("$")
            case ["normal"]:
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
    # CLASS -> 2 DECLARATION GENERATION
    def declaration(self, declaration: Declaration) -> None:
        self.latex.append(f"{declaration.identifier}=")
        self.expression(declaration.expression)
    # CLASS -> 2 NODE GENERATION
    def node(self, node: Node) -> None:
        self.expression(node.expression)
    # CLASS -> 2 EQUATION GENERATION
    def equation(self, equation: Equation) -> None:
        self.expression(equation.leftSide)
        self.latex.append("=")
        self.expression(equation.rightSide)
    # CLASS -> 2 COMMENT GENERATION
    def comment(self, comment: Comment) -> None:
        self.latex.append(r"\text{")
        self.latex.append(comment.text)
        self.latex.append(r"}")
    # CLASS -> 3 EXPRESSION GENERATION
    def expression(self, expression: Expression) -> None:
        for index in range(len(expression.terms)): 
            self.term(expression.terms[index], index == 0)
    # CLASS -> 4 TERM GENERATION
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
                if index == 0:
                    self.factor(numerator[index], noTermSign or index != 0, createFraction = True)
                else:
                    self.factor(numerator[index], True)
                self.latex.append(r"\cdot ")
            self.latex.pop()
            self.latex.append(r"}{")
            for index in range(len(denominator)):
                self.factor(denominator[index], True)
                self.latex.append(r"\cdot ")
            self.latex.pop()
            self.latex.append(r"}")
        else:
            for index in range(len(numerator)):
                self.factor(numerator[index], noTermSign or index != 0)
                self.latex.append(r"\cdot ")
            self.latex.pop()
    # CLASS -> 5 FACTOR GENERATION
    def factor(self, factor: Factor, noSign: bool, createFraction: bool = False) -> None:
        match factor.type:
            case ["unsigned", "number"]:
                self.latex.append('' if noSign else "+")
                self.latex.append(r'\frac{' if createFraction else '')
                self.latex.append(factor.value)
            case ["unsigned", "identifier"]: 
                self.latex.append('' if noSign else "+")
                self.latex.append(r'\frac{' if createFraction else '')
                self.latex.append(factor.value)
            case ["unsigned", "expression"]: 
                self.latex.append('' if noSign else "+")
                self.latex.append(r'\frac{' if createFraction else '')
                self.latex.append(r"\left( ")
                self.expression(factor.value)
                self.latex.append(r"\right) ")
            case ["unsigned", "vector"]:
                self.latex.append('' if noSign else "+")
                self.latex.append(r'\frac{' if createFraction else '')
                self.vector(factor.value)
            case ["signed", "number"]: 
                self.latex.append(factor.signs)
                self.latex.append(r"\frac{" if createFraction else '')
                self.latex.append(factor.value)
            case ["signed", "identifier"]:
                self.latex.append(factor.signs)
                self.latex.append(r"\frac{" if createFraction else '')
                self.latex.append(factor.value)
            case ["signed", "expression"]: 
                self.latex.append(factor.signs)
                self.latex.append(r'\frac{' if createFraction else '')
                self.latex.append(r"\left( ")
                self.expression(factor.value)
                self.latex.append(r"\right) ")
            case ["signed", "vector"]:
                self.latex.append(factor.signs)
                self.latex.append(r"\frac{" if createFraction else '')
                self.vector(factor.value)
    # CLASS -> 6 VECTOR GENERATION
    def vector(self, vector: Vector) -> None:
        self.latex.append(r"\begin{bmatrix}")
        if vector.expressions:
            for expression in vector.expressions:
                self.expression(expression)
                self.latex.append(r"\\ ")
            self.latex.pop()
        else:
            self.latex.append(r"\; ")
        self.latex.append(r"\end{bmatrix}")