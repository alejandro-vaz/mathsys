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
        for index in range(len(term.factors)):
            self.factor(term.factors[index], noTermSign or index != 0)
            self.latex.append(r"\cdot ")
        self.latex.pop()
    # CLASS -> FACTOR GENERATION
    def factor(self, factor: Factor, noSign: bool) -> None:
        match factor.type:
            case ["unsigned", "number"]: self.latex.append(f"{'' if noSign else '+'}{factor.value}")
            case ["unsigned", "identifier"]: self.latex.append(f"{'' if noSign else '+'}{factor.value}")
            case ["unsigned", "expression"]: 
                self.latex.append(fr"{'' if noSign else '+'}\left( ")
                self.expression(factor.value)
                self.latex.append(r"\right) ")
            case ["signed", "number"]: self.latex.append(fr"{factor.signs}{factor.value}")
            case ["signed", "identifier"]: self.latex.append(fr"{factor.signs}{factor.value}")
            case ["signed", "expression"]: 
                self.latex.append(fr"{factor.signs}\left( ")
                self.expression(factor.value)
                self.latex.append(r"\right) ")