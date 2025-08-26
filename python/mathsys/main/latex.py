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
    Definition,
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
    Infinite,
    Limit,
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
    MAPPINGS = {
        "alpha": r"\alpha ",
        "Alpha": r"A",
        "beta": r"\beta ",
        "Beta": r"B",
        "gamma": r"\gamma ",
        "Gamma": r"\Gamma ",
        "delta": r"\delta ",
        "Delta": r"\Delta ",
        "epsilon": r"\epsilon ",
        "Epsilon": r"E",
        "zeta": r"\zeta ",
        "Zeta": r"Z",
        "eta": r"\eta ",
        "Eta": r"H",
        "theta": r"\theta ",
        "Theta": r"\Theta ",
        "iota": r"\iota ",
        "Iota": r"I",
        "kappa": r"\kappa ",
        "Kappa": r"K",
        "lambda": r"\lambda ",
        "Lambda": r"\Lambda ",
        "mu": r"\mu ",
        "Mu": r"M",
        "nu": r"\nu ",
        "Nu": r"N",
        "xi": r"\xi ",
        "Xi": r"\Xi ",
        "omicron": r"\omicron ",
        "Omicron": r"O",
        "pi": r"\pi ",
        "Pi": r"\pi ",
        "rho": r"\rho ",
        "Rho": r"P",
        "sigma": r"\sigma ",
        "Sigma": r"\Sigma ",
        "tau": r"\tau ",
        "Tau": r"T",
        "upsilon": r"\upsilon ",
        "Upsilon": r"\Upsilon ",
        "phi": r"\phi ",
        "Phi": r"\Phi ",
        "chi": r"\chi ",
        "Chi": r"X",
        "psi": r"\psi ",
        "Psi": r"\Psi ",
        "omega": r"\omega ",
        "Omega": r"\Omega ",
        "varepsilon": r"\varepsilon ",
        "vartheta": r"\vartheta ",
        "varpi": r"\varpi ",
        "varrho": r"\varrho ",
        "varsigma": r"\varsigma ",
        "varphi": r"\varphi "
    }
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
            case 0: delimiter = ""
            case 1: delimiter = "$"
            case _: delimiter = "$$"
        self.latex.append(delimiter)
        for statement in sheet.statements:
            match statement:
                case Definition(): self.definition(statement)
                case Declaration(): self.declaration(statement)
                case Node(): self.node(statement)
                case Equation(): self.equation(statement)
                case Comment(): self.comment(statement)
            self.latex.append(r"\\ ")
        if len(sheet.statements) >= 1: self.latex.pop()
        self.latex.append(delimiter)
    # GENERATOR -> 2 DEFINITION GENERATION
    def definition(self, definition: Definition) -> None:
        self.latex.append(self.MAPPINGS[definition.identifier] if definition.identifier in self.MAPPINGS else definition.identifier)
        self.latex.append(r"\equiv ")
        self.expression(definition.expression)
    # GENERATOR -> 2 DECLARATION GENERATION
    def declaration(self, declaration: Declaration) -> None:
        self.latex.append(self.MAPPINGS[declaration.identifier] if declaration.identifier in self.MAPPINGS else declaration.identifier)
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
        for index in range(len(numerator)):
            match numerator[index]:
                case Infinite(): self.infinite(numerator[index], noTermSign or index != 0, index == 0 and denominator)
                case Limit(): self.limit(numerator[index], noTermSign or index != 0, index == 0 and denominator)
                case Variable(): self.variable(numerator[index], noTermSign or index != 0, index == 0 and denominator)
                case Nest(): self.nest(numerator[index], noTermSign or index != 0, index == 0 and denominator)
                case Vector(): self.vector(numerator[index], noTermSign or index != 0, index == 0 and denominator)
                case Number(): self.number(numerator[index], noTermSign or index != 0, index == 0 and denominator)
            self.latex.append(r"\cdot ")
        self.latex.pop()
        if denominator: 
            self.latex.append(r"}{")
            for index in range(len(denominator)):
                match denominator[index]:
                    case Infinite(): self.infinite(denominator[index], True, False)
                    case Limit(): self.limit(denominator[index], True, False)
                    case Variable(): self.variable(denominator[index], True, False)
                    case Nest(): self.nest(denominator[index], True, False)
                    case Vector(): self.vector(denominator[index], True, False)
                    case Number(): self.number(denominator[index], True, False)
                self.latex.append(r"\cdot ")
            self.latex.pop()
            self.latex.append(r"}")
    # GENERATOR -> 5 INFINITE GENERATION
    def infinite(self, infinite: Infinite, noSign: bool, createFraction: bool) -> None:
        if noSign:
            self.latex.append(infinite.signs)
        else:
            self.latex.append(infinite.signs if infinite.signs is not None else "+")
        if createFraction:
            self.latex.append(r"\frac{")
        self.latex.append(r"\infty ")
        if infinite.exponent is not None:
            self.latex.append(r"^{")
            self.expression(infinite.exponent)
            self.latex.append(r"}")
    # GENERATOR -> 5 LIMIT GENERATION
    def limit(self, limit: Limit, noSign: bool, createFraction: bool) -> None:
        if noSign:
            self.latex.append(limit.signs)
        else:
            self.latex.append(limit.signs if limit.signs is not None else "+")
        if createFraction:
            self.latex.append(r"\frac{")
        self.latex.append(r"\lim_{\substack{")
        self.latex.append(self.MAPPINGS[limit.variable] if limit.variable in self.MAPPINGS else limit.variable)
        self.latex.append(r"\to ")
        self.expression(limit.to)
        self.latex.append(r"}}\left( ")
        self.expression(limit.at)
        self.latex.append(r"\right) ")
        # no level5, but expression
    # GENERATOR -> 5 VARIABLE GENERATION
    def variable(self, variable: Variable, noSign: bool, createFraction: bool) -> None:
        if noSign: 
            self.latex.append(variable.signs)
        else: 
            self.latex.append(variable.signs if variable.signs is not None else "+")
        if createFraction: 
            self.latex.append(r"\frac")
        self.latex.append(self.MAPPINGS[variable.representation] if variable.representation in self.MAPPINGS else variable.representation)
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