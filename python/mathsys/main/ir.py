#
#   HEAD
#

# HEAD -> DATACLASSES
from .parser import Sheet, Declaration, Expression, Term, Variable, Brackets


#
#   GENERATOR
#

# GENERATOR -> CLASS
class LaTeXGenerator:
    # CLASS -> VARIABLES
    latex: list[str]
    counter: int
    memory: list[str]
    # CLASS -> INIT
    def __init__(self) -> None:
        self.latex = []
    # CLASS -> RUN
    def run(self, sheet: Sheet) -> str:
        self.latex.append("$$\n")
        for declaration in sheet.statements:
            self.declaration(declaration)
        self.latex.append("$$")
        return "".join(self.latex)
    # CLASS -> DECLARATION GENERATION
    def declaration(self, declaration: Declaration) -> None:
        self.latex.append(f"{declaration.identifier}=")
        self.expression(declaration.expression)
        self.latex.append("\n")
    # CLASS -> EXPRESSION GENERATION
    def expression(self, expression: Expression) -> None:
        registers = []
        for value in expression.terms:
            match value:
                case Term():
                    self.term(value)
                case Variable():
                    self.variable(value)
                case Brackets():
                    self.brackets(value)
    # CLASS -> TERM GENERATION
    def term(self, value: Term) -> None:
        self.latex.append(f"{value.signs}{value.number}")
    # CLASS -> VARIABLE GENERATION
    def variable(self, value: Variable) -> None:
        self.latex.append(f"{value.signs}{value.identifier}")
    # CLASS -> BRACKETS GENERATION
    def brackets(self, value: Brackets) -> None:
        self.latex.append(fr"{value.signs}\left( ")
        self.expression(value.expression)
        self.latex.append(r"\right) ")