#^
#^  HEAD
#^

#> HEAD -> DATACLASSES
from dataclasses import dataclass
from .parser import (
    #~ DATACLASSES -> START
    Start,
    #~ DATACLASSES -> 1ºLEVEL
    Level1,
    Declaration,
    Definition,
    Node,
    Equation,
    Comment, 
    #~ DATACLASSES -> 2ºLEVEL
    Level2,
    Expression,
    #~ DATACLASSES -> 3ºLEVEL
    Level3,
    Term,
    #~ DATACLASSES -> 4ºLEVEL
    Level4,
    Factor,
    Limit,
    #~ DATACLASSES -> 5ºLEVEL
    Level5,
    Infinite,
    Variable,
    Nest,
    Vector,
    Number
)


#^
#^  MAPPINGS
#^

#> MAPPINGS -> VARIABLES
VARIABLES = {
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
    "Omega": r"\Omega "
}

#> MAPPINGS -> SPECIAL
SPECIAL = {
    '\\': r'\\',
    '{': r'\{',
    '}': r'\}',
    '$': r'\$'
}


#^
#^  START
#^

#> START -> CLASS
@dataclass(frozen = True)
class LTXStart:
    statements: list[str]
    def __str__(self) -> str:
        match len(self.statements):
            case 0: delimiters = ["", ""]
            case 1: delimiters = [r"\(", r"\)"]
            case other: delimiters = [r"\[", r"\]"]
        values = r"\\ ".join(self.statements)
        while values.startswith(r"\\"): values = values[2:]
        return f"{delimiters[0]}{values}{delimiters[1]}"


#^
#^  1ºLEVEL
#^

#> 1ºLEVEL -> DECLARATION
@dataclass(frozen = True)
class LTXDeclaration:
    identifier: str
    expression: str
    def __str__(self) -> str:
        return f"{self.identifier}={self.expression}"

#> 1ºLEVEL -> DEFINITION
@dataclass(frozen = True)
class LTXDefinition:
    identifier: str
    expression: str
    def __str__(self) -> str:
        return f"{self.identifier}\equiv {self.expression}"

#> 1ºLEVEL -> NODE
@dataclass(frozen = True)
class LTXNode:
    value: str
    def __str__(self) -> str:
        return self.value

#> 1ºLEVEL -> EQUATION
@dataclass(frozen = True)
class LTXEquation:
    left: str
    right: str
    def __str__(self) -> str:
        return f"{self.left}={self.right}"

#> 1ºLEVEL -> COMMENT
@dataclass(frozen = True)
class LTXComment:
    text: str
    def __str__(self) -> str:
        curated = "".join(SPECIAL.get(character, character) for character in self.text)
        return fr"\\\text{{{curated}}}"


#^
#^  2ºLEVEL
#^

#> 2ºLEVEL -> EXPRESSION
@dataclass(frozen = True)
class LTXExpression:
    signs: list[str]
    terms: list[str]
    def __str__(self) -> str:
        string = "".join([f"{self.signs[index]}{self.terms[index]}" for index in range(len(self.terms))])
        return string


#^
#^  3ºLEVEL
#^

#> 3ºLEVEL -> TERM
@dataclass(frozen = True)
class LTXTerm:
    numerator: list[str]
    denominator: list[str]
    def __str__(self) -> str:
        numerator = "".join(self.numerator)
        denominator = "".join(self.denominator)
        assembly = fr"\frac{{{numerator}}}{{{denominator}}}" if len(self.denominator) != 0 else numerator
        return assembly


#^
#^  4ºLEVEL
#^

#> 4ºLEVEL -> FACTOR
@dataclass(frozen = True)
class LTXFactor:
    value: str
    exponent: str
    def __str__(self) -> str:
        exponent = f"^{{{self.exponent}}}" if self.exponent else ""
        return f"{self.value}{exponent}"

#> 4ºLEVEL -> LIMIT
@dataclass(frozen = True)
class LTXLimit:
    variable: str
    approach: str
    direction: str
    nest: str
    exponent: str
    def __str__(self) -> str:
        direction = f"^{{{self.direction}}}" if self.direction else ""
        exponent = f"^{{{self.exponent}}}" if self.exponent else ""
        return fr"\lim_{{\substack{{{self.variable}\to {self.approach}{direction}}}}}{self.nest}{exponent}"


#^
#^  5ºLEVEL
#^

#> 5ºLEVEL -> INFINITE
@dataclass(frozen = True)
class LTXInfinite:
    def __str__(self) -> str:
        return r"\infty "

#> 5ºLEVEL -> VARIABLE
@dataclass(frozen = True)
class LTXVariable:
    name: str
    def __str__(self) -> str:
        curated = self.name
        for source, replace in VARIABLES.items(): curated = curated.replace(source, replace)
        return curated

#> 5ºLEVEL -> NEST
@dataclass(frozen = True)
class LTXNest:
    expression: str
    def __str__(self) -> str:
        inside = self.expression if self.expression else r"\, "
        return fr"\left( {inside}\right) "

#> 5ºLEVEL -> VECTOR
@dataclass(frozen = True)
class LTXVector:
    values: list[str]
    def __str__(self) -> str:
        inside = r"\; " if len(self.values) == 0 else r"\\ ".join(self.values)
        return fr"\begin{{bmatrix}}{inside}\end{{bmatrix}}"

#> 5ºLEVEL -> NUMBER
@dataclass(frozen = True)
class LTXNumber:
    whole: str
    decimal: str
    def __str__(self) -> str:
        decimal = f".{self.decimal}" if self.decimal else ""
        return f"{self.whole}{decimal}"


#^
#^  LATEX
#^

#> LATEX -> GENERATOR
class LaTeX:
    #~ GENERATOR -> INIT
    def __init__(self) -> None: pass
    #~ GENERATOR -> RUN
    def run(self, start: Start) -> str: return self.start(start)
    #~ GENERATOR -> START GENERATION
    def start(self, start: Start) -> str:
        return str(LTXStart(
            statements = [self.level1(statement) for statement in start.statements if self.level1(statement)]
        ))
    #~ GENERATOR -> 1 LEVEL GENERATION
    def level1(self, level1: Level1) -> str:
        match level1:
            case Declaration(): return self.declaration(level1)
            case Definition(): return self.definition(level1)
            case Node(): return self.node(level1)
            case Equation(): return self.equation(level1)
            case Comment(): return self.comment(level1)
    #~ GENERATOR -> 1 DECLARATION GENERATION
    def declaration(self, declaration: Declaration) -> str:
        return str(LTXDeclaration(
            identifier = self.variable(declaration.identifier),
            expression = self.expression(declaration.expression)
        ))
    #~ GENERATOR -> 1 DEFINITION GENERATION
    def definition(self, definition: Definition) -> str:
        return str(LTXDefinition(
            identifier = self.variable(definition.identifier),
            expression = self.expression(definition.expression)
        ))
    #~ GENERATOR -> 1 NODE GENERATION
    def node(self, node: Node) -> str:
        return str(LTXNode(
            value = self.expression(node.value)
        ))
    #~ GENERATOR -> 1 EQUATION GENERATION
    def equation(self, equation: Equation) -> str:
        return str(LTXEquation(
            left = self.expression(equation.left),
            right = self.expression(equation.right)
        ))
    #~ GENERATOR -> 1 COMMENT GENERATION
    def comment(self, comment: Comment) -> str:
        return str(LTXComment(
            text = comment.content
        ))
    #~ GENERATOR -> 2 LEVEL GENERATION
    def level2(self, level2: Level2) -> str:
        match level2:
            case Expression(): return self.expression(level2)
    #~ GENERATOR -> 2 EXPRESSION GENERATION
    def expression(self, expression: Expression) -> str:
        return str(LTXExpression(
            signs = [sign if sign is not None else "" for sign in expression.signs],
            terms = [self.level3(term) for term in expression.terms]
        ))
    #~ GENERATOR -> 3 LEVEL GENERATION
    def level3(self, level3: Level3) -> str:
        match level3:
            case Term(): return self.term(level3)
    #~ GENERATOR -> 3 TERM GENERATION
    def term(self, term: Term) -> str:
        numerator = []
        for index in range(len(term.numerator)):
            value = self.level4(term.numerator[index])
            if index != 0:
                if isinstance(term.numerator[index - 1], Factor):
                    if isinstance(term.numerator[index - 1].value, Number):
                        if isinstance(term.numerator[index], Factor):
                            if isinstance(term.numerator[index].value, Number | Infinite):
                                value = r"\cdot " + value
                    else: value = r"\cdot " + value
                else: value = r"\cdot " + value
            numerator.append(value)
        denominator = []
        for index in range(len(term.denominator)):
            value = self.level4(term.denominator[index])
            if index != 0:
                if isinstance(term.denominator[index - 1], Factor):
                    if isinstance(term.denominator[index - 1].value, Number):
                        if isinstance(term.denominator[index], Factor):
                            if isinstance(term.denominator[index].value, Number | Infinite):
                                value = r"\cdot " + value
                    else: value = r"\cdot " + value
                else: value = r"\cdot " + value
            denominator.append(value)
        return str(LTXTerm(
            numerator = numerator,
            denominator = denominator
        ))
    #~ GENERATOR -> 4 LEVEL GENERATION
    def level4(self, level4: Level4) -> str:
        match level4:
            case Factor(): return self.factor(level4)
            case Limit(): return self.limit(level4)
    #~ GENERATOR -> 4 FACTOR GENERATION
    def factor(self, factor: Factor) -> str:
        return str(LTXFactor(
            value = self.level5(factor.value),
            exponent = self.expression(factor.exponent) if factor.exponent is not None else ""
        ))
    #~ GENERATOR -> 4 LIMIT GENERATION
    def limit(self, limit: Limit) -> str:
        return str(LTXLimit(
            variable = self.variable(limit.variable),
            approach = self.expression(limit.approach),
            direction = "+" if limit.direction else "-" if limit.direction is not None else "",
            nest = self.nest(limit.of),
            exponent = self.expression(limit.exponent) if limit.exponent is not None else ""
        ))
    #~ GENERATOR -> 5 LEVEL GENERATION
    def level5(self, level5: Level5) -> str:
        match level5:
            case Infinite(): return self.infinite(level5)
            case Variable(): return self.variable(level5)
            case Nest(): return self.nest(level5)
            case Vector(): return self.vector(level5)
            case Number(): return self.number(level5)
    #~ GENERATOR -> 5 INFINITE GENERATION
    def infinite(self, infinite: Infinite) -> str: 
        return str(LTXInfinite())
    #~ GENERATOR -> 5 VARIABLE GENERATION
    def variable(self, variable: Variable) -> str:
        return str(LTXVariable(
            name = variable.representation
        ))
    #~ GENERATOR -> 5 NEST GENERATION
    def nest(self, nest: Nest) -> str:
        return str(LTXNest(
            expression = self.expression(nest.expression) if nest.expression is not None else ""
        ))
    #~ GENERATOR -> 5 VECTOR GENERATION
    def vector(self, vector: Vector) -> str:
        return str(LTXVector(
            values = [self.expression(value) for value in vector.values]
        ))
    #~ GENERATOR -> 5 NUMBER GENERATION
    def number(self, number: Number) -> str:
        return str(LTXNumber(
            whole = number.whole,
            decimal = number.decimal if number.decimal is not None else ""
        ))