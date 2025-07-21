#
#   HEAD
#

# HEAD -> DATACLASSES
from .parser import Sheet, Declaration, Expression, Term, Factor


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
    def run(self, sheet: Sheet) -> str:
        self.sheet(sheet)
        return ''.join(self.latex)
    # CLASS -> SHEET GENERATION
    def sheet(self, sheet: Sheet) -> None:
        match sheet.type:
            case ["empty"]: pass
            case ["inline"]:
                self.latex.append("$")
                self.declaration(sheet.statements[0])
                self.latex.append("$")
            case ["normal"]:
                self.latex.append("$$")
                for declaration in sheet.statements:
                    self.declaration(declaration)
                    self.latex.append(r"\\")
                self.latex.pop()
                self.latex.append("$$")
    # CLASS -> DECLARATION GENERATION
    def declaration(self, declaration: Declaration) -> None:
        self.latex.append(f"{declaration.identifier}=")
        self.expression(declaration.expression)
    # CLASS -> EXPRESSION GENERATION
    def expression(self, expression: Expression) -> None:
        for index in range(len(expression.terms)): 
            self.term(expression.terms[index], index == 0)
    # CLASS -> TERM GENERATION
    def term(self, term: Term, noTermSign: bool) -> None:
        numerator = []
        denominator = []
        for index in range(len(term.factors)):
            if index == 0: numerator.append(term.factors[0]); continue
            match term.operators[index - 1]:
                case "*": numerator.append(term.factors[index])
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
    # CLASS -> FACTOR GENERATION
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