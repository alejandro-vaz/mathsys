#
#   HEAD
#

# HEAD -> DATACLASSES
from .parser import (
    # DATACLASSES -> START
    Start,
    # DATACLASSES -> 1ºLEVEL
    Level1,
    Debug,
    Declaration,
    Definition,
    Node,
    Equation,
    Comment, 
    # DATACLASSES -> 2ºLEVEL
    Level2,
    Expression,
    # DATACLASSES -> 3ºLEVEL
    Level3,
    Term,
    # DATACLASSES -> 4ºLEVEL
    Level4,
    Factor,
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
    def run(self, start: Start) -> str:
        self.latex = []
        self.start(start)
        return ''.join([string for string in self.latex if string is not None])
    # GENERATOR -> START GENERATION
    def start(self, start: Start) -> None:
        match len(start.statements):
            case 0: delimiter = ""
            case 1: delimiter = "$"
            case _: delimiter = "$$"
        self.latex.append(delimiter)
        for statement in start.statements:
            self.level1(statement)
            self.latex.append(r"\\ ")
        if len(start.statements) >= 1: self.latex.pop()
        self.latex.append(delimiter)
    # GENERATOR -> 1 LEVEL GENERATION
    def level1(self, level1: Level1) -> None:
        match level1:
            case Debug(): self.debug(level1)
            case Declaration(): self.declaration(level1)
            case Definition(): self.definition(level1)
            case Node(): self.node(level1)
            case Equation(): self.equation(level1)
            case Comment(): self.comment(level1)
    # GENERATOR -> 1 DEBUG GENERATION
    def debug(self, debug: Debug) -> None: pass
    # GENERATOR -> 1 DECLARATION GENERATION
    def declaration(self, declaration: Declaration) -> None:
        self.variable(declaration.identifier)
        self.latex.append("=")
        self.expression(declaration.expression)
    # GENERATOR -> 1 DEFINITION GENERATION
    def definition(self, definition: Definition) -> None:
        self.variable(definition.identifier)
        self.latex.append(r"\equiv ")
        self.expression(definition.expression)
    # GENERATOR -> 1 NODE GENERATION
    def node(self, node: Node) -> None:
        self.expression(node.value)
    # GENERATOR -> 1 EQUATION GENERATION
    def equation(self, equation: Equation) -> None:
        self.expression(equation.left)
        self.latex.append("=")
        self.expression(equation.right)
    # GENERATOR -> 1 COMMENT GENERATION
    def comment(self, comment: Comment) -> None:
        self.latex.append(r"\text{")
        self.latex.append(comment.content)
        self.latex.append(r"}")
    # GENERATOR -> 2 LEVEL GENERATION
    def level2(self, level2: Level2) -> None:
        match level2:
            case Expression(): self.expression(level2)
    # GENERATOR -> 2 EXPRESSION GENERATION
    def expression(self, expression: Expression) -> None:
        for index in range(len(expression.terms)):
            if expression.signs[index] is not None: self.latex.append(expression.signs[index])
            self.level3(expression.terms[index])
    # GENERATOR -> 3 LEVEL GENERATION
    def level3(self, level3: Level3) -> None:
        match level3:
            case Term(): self.term(level3)
    # GENERATOR -> 3 TERM GENERATION
    def term(self, term: Term) -> None:
        if term.denominator: self.latex.append(r"\frac{")
        for index in range(len(term.numerator)):
            if index != 0 and not (
                isinstance(term.numerator[index - 1].value, Number) and
                not isinstance(term.numerator[index].value, Number | Infinite)
            ): self.latex.append(r"\cdot ")
            self.level4(term.numerator[index])
        if term.denominator: self.latex.append(r"}{")
        for index in range(len(term.denominator)):
            if index != 0 and not (
                isinstance(term.denominator[index - 1].value, Number) and
                not isinstance(term.denominator[index].value, Number | Infinite)
            ): self.latex.append(r"\cdot ")
            self.level4(term.denominator[index])
        if term.denominator: self.latex.append(r"}")
    # GENERATOR -> 4 LEVEL GENERATION
    def level4(self, level4: Level4) -> None:
        match level4:
            case Factor(): self.factor(level4)
    # GENERATOR -> 4 FACTOR GENERATION
    def factor(self, factor: Factor) -> None:
        self.level5(factor.value)
        if factor.exponent is not None:
            self.latex.append(r"^{")
            self.expression(factor.exponent)
            self.latex.append(r"}")
    # GENERATOR -> 5 LEVEL GENERATION
    def level5(self, level5: Level5) -> None:
        match level5:
            case Infinite(): self.infinite(level5)
            case Limit(): self.limit(level5)
            case Variable(): self.variable(level5)
            case Nest(): self.nest(level5)
            case Vector(): self.vector(level5)
            case Number(): self.number(level5)
    # GENERATOR -> 5 INFINITE GENERATION
    def infinite(self, infinite: Infinite) -> None: self.latex.append(r"\infty ")
    # GENERATOR -> 5 LIMIT GENERATION
    def limit(self, limit: Limit) -> None:
        self.latex.append(r"\lim_{\substack{")
        self.variable(limit.variable)
        self.latex.append(r"\to ")
        self.expression(limit.approach)
        if limit.direction is not None: 
            self.latex.append(r"^{")
            self.latex.append(r"+" if limit.direction else r"-")
            self.latex.append(r"}")
        self.latex.append(r"}}")
        self.nest(limit.of)
    # GENERATOR -> 5 VARIABLE GENERATION
    def variable(self, variable: Variable) -> None:
        self.latex.append(self.MAPPINGS.get(variable.representation, variable.representation))
    # GENERATOR -> 5 NEST GENERATION
    def nest(self, nest: Nest) -> None:
        self.latex.append(r"\left( ")
        self.expression(nest.expression)
        self.latex.append(r"\right) ")
    # GENERATOR -> 5 VECTOR GENERATION
    def vector(self, vector: Vector) -> None:
        self.latex.append(r"\begin{bmatrix}")
        if vector.values:
            for expression in vector.values:
                self.expression(expression)
                self.latex.append(r"\\ ")
            self.latex.pop()
        else:
            self.latex.append(r"\; ")
        self.latex.append(r"\end{bmatrix}")
    # GENERATOR -> 5 NUMBER GENERATION
    def number(self, number: Number) -> None:
        self.latex.append(str(number.whole))
        if number.decimal is not None:
            self.latex.append(r".")
            self.latex.append(str(number.decimal))