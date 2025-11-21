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
    Annotation,
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
    Tensor,
    Number
)


#^
#^  MAPPINGS
#^

#> MAPPINGS -> VARIABLES
VARIABLES = {
    "epsilon": r"\epsilon ",
    "Epsilon": r"E",
    "omicron": r"\omicron ",
    "Omicron": r"O",
    "upsilon": r"\upsilon ",
    "Upsilon": r"\Upsilon ",
    "lambda": r"\lambda ",
    "Lambda": r"\Lambda ",
    "alpha": r"\alpha ",
    "Alpha": r"A",
    "gamma": r"\gamma ",
    "Gamma": r"\Gamma ",
    "delta": r"\delta ",
    "Delta": r"\Delta ",
    "theta": r"\theta ",
    "Theta": r"\Theta ",
    "kappa": r"\kappa ",
    "Kappa": r"K",
    "sigma": r"\sigma ",
    "Sigma": r"\Sigma ",
    "omega": r"\omega ",
    "Omega": r"\Omega ",
    "beta": r"\beta ",
    "Beta": r"B",
    "zeta": r"\zeta ",
    "Zeta": r"Z",
    "iota": r"\iota ",
    "Iota": r"I",
    "eta": r"\eta ",
    "Eta": r"H",
    "rho": r"\rho ",
    "Rho": r"P",
    "tau": r"\tau ",
    "Tau": r"T",
    "phi": r"\phi ",
    "Phi": r"\Phi ",
    "chi": r"\chi ",
    "Chi": r"X",
    "psi": r"\psi ",
    "Psi": r"\Psi ",
    "mu": r"\mu ",
    "Mu": r"M",
    "nu": r"\nu ",
    "Nu": r"N",
    "xi": r"\xi ",
    "Xi": r"\Xi ",
    "pi": r"\pi ",
    "Pi": r"\pi "
}

#> MAPPINGS -> SPECIAL
SPECIAL = {
    '\\': r'\\',
    '{': r'\{',
    '}': r'\}',
    '$': r'\$'
}

#> MAPPINGS -> TYPE TABLE
TYPES = {}

#> MAPPINGS -> CONVERSION TABLE
CONVERSION = {
    None: lambda name: name,
    "Infinite": lambda name: f"\overset{{\infty}}{{{name}}}",
    "Nexists": lambda name: fr"\nexists\,{name}",
    "Number": lambda name: name,
    "Tensor": lambda name: f"\overline{{{name}}}",
    "Undefined": lambda name: f"\overset{{?}}{{{name}}}",
    "Variable": lambda name: f"{{^{{*}}{name}}}"
}


#^
#^  START
#^

#> START -> CLASS
@dataclass
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
@dataclass
class LTXDeclaration:
    objectType: str | None
    identifier: str
    expression: str
    def __str__(self) -> str:
        TYPES[self.identifier] = TYPES.get(self.identifier, self.objectType)
        return f"{self.identifier}={self.expression}"

#> 1ºLEVEL -> DEFINITION
@dataclass
class LTXDefinition:
    objectType: str | None
    identifier: str
    expression: str
    def __str__(self) -> str:
        TYPES[self.identifier] = TYPES.get(self.identifier, self.objectType)
        return f"{self.identifier}\equiv {self.expression}"

#> 1ºLEVEL -> ANNOTATION
@dataclass
class LTXAnnotation:
    objectType: str
    identifier: str
    def __str__(self) -> str:
        TYPES[self.identifier] = TYPES.get(self.identifier, self.objectType)
        return fr"\textbf{{{TYPES[self.identifier]} }}{self.identifier}"

#> 1ºLEVEL -> NODE
@dataclass
class LTXNode:
    value: str
    def __str__(self) -> str:
        return self.value

#> 1ºLEVEL -> EQUATION
@dataclass
class LTXEquation:
    left: str
    right: str
    def __str__(self) -> str:
        return f"{self.left}={self.right}"

#> 1ºLEVEL -> COMMENT
@dataclass
class LTXComment:
    text: str
    def __str__(self) -> str:
        curated = "".join(SPECIAL.get(character, character) for character in self.text)
        return fr"\\\text{{{curated}}}"


#^
#^  2ºLEVEL
#^

#> 2ºLEVEL -> EXPRESSION
@dataclass
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
@dataclass
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
@dataclass
class LTXFactor:
    value: str
    exponent: str
    def __str__(self) -> str:
        exponent = f"^{{{self.exponent}}}" if self.exponent else ""
        return f"{self.value}{exponent}"

#> 4ºLEVEL -> LIMIT
@dataclass
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
@dataclass
class LTXInfinite:
    def __str__(self) -> str:
        return r"\infty "

#> 5ºLEVEL -> VARIABLE
@dataclass
class LTXVariable:
    name: str
    def __str__(self) -> str:
        curated = self.name
        for source, replace in VARIABLES.items(): curated = curated.replace(source, replace)
        identifier = CONVERSION[TYPES[self.name] if self.name in TYPES else None](curated)
        return identifier

#> 5ºLEVEL -> NEST
@dataclass
class LTXNest:
    expression: str
    def __str__(self) -> str:
        inside = self.expression if self.expression else r"\, "
        return fr"\left( {inside}\right) "

#> 5ºLEVEL -> TENSOR
@dataclass
class LTXTensor:
    values: list[str]
    def __str__(self) -> str:
        inside = r"\; " if len(self.values) == 0 else r"\\ ".join(self.values)
        return fr"\begin{{bmatrix}}{inside}\end{{bmatrix}}"

#> 5ºLEVEL -> NUMBER
@dataclass
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
    def run(self, start: Start) -> str: TYPES.clear(); return self.start(start)
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
            case Annotation(): return self.annotation(level1)
            case Node(): return self.node(level1)
            case Equation(): return self.equation(level1)
            case Comment(): return self.comment(level1)
    #~ GENERATOR -> 1 DECLARATION GENERATION
    def declaration(self, declaration: Declaration) -> str:
        return str(LTXDeclaration(
            objectType = declaration.objectType,
            identifier = self.variable(declaration.identifier),
            expression = self.expression(declaration.expression)
        ))
    #~ GENERATOR -> 1 DEFINITION GENERATION
    def definition(self, definition: Definition) -> str:
        return str(LTXDefinition(
            objectType = definition.objectType,
            identifier = self.variable(definition.identifier),
            expression = self.expression(definition.expression)
        ))
    #~ GENERATOR -> 1 ANNOTATION GENERATION
    def annotation(self, annotation: Annotation) -> str:
        return str(LTXAnnotation(
            objectType = annotation.objectType,
            identifier = self.variable(annotation.identifier)
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
            case Tensor(): return self.tensor(level5)
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
    #~ GENERATOR -> 5 TENSOR GENERATION
    def tensor(self, vector: Tensor) -> str:
        return str(LTXTensor(
            values = [self.expression(value) for value in vector.values]
        ))
    #~ GENERATOR -> 5 NUMBER GENERATION
    def number(self, number: Number) -> str:
        return str(LTXNumber(
            whole = number.whole,
            decimal = number.decimal if number.decimal is not None else ""
        ))