#^
#^  HEAD
#^

#> HEAD -> MODULES
from dataclasses import dataclass

#> HEAD -> DATA
from .local import SPECIAL, VARIABLES, CONVERSION, types


#^
#^  START
#^

#> START -> CLASS
@dataclass(kw_only = True)
class Start:
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
@dataclass(kw_only = True)
class Declaration:
    group: str
    variable: str
    expression: str
    def __str__(self) -> str:
        return f"{self.variable}={self.expression}"

#> 1ºLEVEL -> DEFINITION
@dataclass(kw_only = True)
class Definition:
    group: str
    variable: str
    expression: str
    def __str__(self) -> str:
        return fr"{self.variable}\equiv {self.expression}"

#> 1ºLEVEL -> ANNOTATION
@dataclass(kw_only = True)
class Annotation:
    group: str
    variables: list[str]
    def __str__(self) -> str:
        variables = ",".join(self.variables)
        match self.group:
            case "@Integer": return fr"{variables}\in\mathbb{{Z}}"
            case "@Natural": return fr"{variables}\in\mathbb{{N}}"
            case "@Rational": return fr"{variables}\in\mathbb{{Q}}"
            case "@Whole": return fr"{variables}\in\mathbb{{W}}"
            case other: return ""

#> 1ºLEVEL -> NODE
@dataclass(kw_only = True)
class Node:
    expression: str
    def __str__(self) -> str:
        return self.expression

#> 1ºLEVEL -> EQUATION
@dataclass(kw_only = True)
class Equation:
    leftexpression: str
    rightexpression: str
    def __str__(self) -> str:
        return f"{self.leftexpression}={self.rightexpression}"

#> 1ºLEVEL -> COMMENT
@dataclass(kw_only = True)
class Comment:
    text: str
    def __str__(self) -> str:
        curated = "".join(SPECIAL.get(character, character) for character in self.text)
        return fr"\\\text{{{curated}}}"

#> 1ºLEVEL -> USE
@dataclass(kw_only = True)
class Use:
    name: str
    start: bool
    def __str__(self) -> str:
        delimiters = ["", ""] if self.start else [r"\color{brown}", r"\color{black}"]
        return fr"{delimiters[0]}\textbf{{use {self.name}}}{delimiters[1]}"


#^
#^  2ºLEVEL
#^

#> 2ºLEVEL -> EXPRESSION
@dataclass(kw_only = True)
class Expression:
    signs: list[bool | None]
    terms: list[str]
    def __str__(self) -> str:
        string = []
        for index in range(len(self.terms)):
            sign = ("+" if self.signs[index] else "-") if self.signs[index] is not None else ""
            string.append(f"{sign}{self.terms[index]}")
        return "".join(string)


#^
#^  3ºLEVEL
#^

#> 3ºLEVEL -> TERM
@dataclass(kw_only = True)
class Term:
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
@dataclass(kw_only = True)
class Factor:
    value: str
    exponent: str | None
    def __str__(self) -> str:
        exponent = f"^{{{self.exponent}}}" if self.exponent is not None else ""
        return f"{self.value}{exponent}"

#> 4ºLEVEL -> LIMIT
@dataclass(kw_only = True)
class Limit:
    variable: str
    approach: str
    direction: bool | None
    nest: str
    exponent: str | None
    def __str__(self) -> str:
        direction = f"^{{{"+" if self.direction else "-"}}}" if self.direction is not None else ""
        exponent = f"^{{{self.exponent}}}" if self.exponent is not None else ""
        return fr"\lim_{{\substack{{{self.variable}\to {self.approach}{direction}}}}}{self.nest}{exponent}"


#^
#^  5ºLEVEL
#^

#> 5ºLEVEL -> INFINITE
@dataclass(kw_only = True)
class Infinite:
    def __str__(self) -> str:
        return r"\infty "

#> 5ºLEVEL -> VARIABLE
@dataclass(kw_only = True)
class Variable:
    representation: str
    def __str__(self) -> str:
        curated = self.representation
        for source, replace in VARIABLES.items(): curated = curated.replace(source, replace)
        identifier = CONVERSION[types[self.representation] if self.representation in types else None](curated)
        return identifier

#> 5ºLEVEL -> NEST
@dataclass(kw_only = True)
class Nest:
    expression: str
    def __str__(self) -> str:
        inside = self.expression if self.expression else r"\, "
        return fr"\left( {inside}\right) "

#> 5ºLEVEL -> TENSOR
@dataclass(kw_only = True)
class Tensor:
    values: list[str]
    def __str__(self) -> str:
        inside = r"\; " if len(self.values) == 0 else r"\\ ".join(self.values)
        return fr"\begin{{bmatrix}}{inside}\end{{bmatrix}}"

#> 5ºLEVEL -> WHOLE
@dataclass(kw_only = True)
class Whole:
    value: int
    def __str__(self) -> str:
        return f"{self.value}"

#> 5ºLEVEL -> ABSOLUTE
@dataclass(kw_only = True)
class Absolute:
    expression: str
    def __str__(self) -> str:
        return fr"\left| {self.expression}\right| "