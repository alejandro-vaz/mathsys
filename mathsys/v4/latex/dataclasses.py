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
@dataclass
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
@dataclass
class Declaration:
    group: str
    variable: str
    expression: str
    def __str__(self) -> str:
        return f"{self.variable}={self.expression}"

#> 1ºLEVEL -> DEFINITION
@dataclass
class Definition:
    group: str
    variable: str
    expression: str
    def __str__(self) -> str:
        return f"{self.variable}\equiv {self.expression}"

#> 1ºLEVEL -> ANNOTATION
@dataclass
class Annotation:
    group: str
    variables: list[str]
    def __str__(self) -> str:
        return ""

#> 1ºLEVEL -> NODE
@dataclass
class Node:
    expression: str
    def __str__(self) -> str:
        return self.expression

#> 1ºLEVEL -> EQUATION
@dataclass
class Equation:
    leftexpression: str
    rightexpression: str
    def __str__(self) -> str:
        return f"{self.leftexpression}={self.rightexpression}"

#> 1ºLEVEL -> COMMENT
@dataclass
class Comment:
    text: str
    def __str__(self) -> str:
        curated = "".join(SPECIAL.get(character, character) for character in self.text)
        return fr"\\\text{{{curated}}}"

#> 1ºLEVEL -> USE
@dataclass
class Use:
    name: str
    start: bool
    def __str__(self) -> str:
        delimiters = [
            "" if self.start else r"\color{brown}",
            "" if self.start else r"\color{black}"
        ]
        return fr"{delimiters[0]}\textbf{{use {self.name}}}{delimiters[1]}"


#^
#^  2ºLEVEL
#^

#> 2ºLEVEL -> EXPRESSION
@dataclass
class Expression:
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
@dataclass
class Factor:
    value: str
    exponent: str
    def __str__(self) -> str:
        exponent = f"^{{{self.exponent}}}" if self.exponent else ""
        return f"{self.value}{exponent}"

#> 4ºLEVEL -> LIMIT
@dataclass
class Limit:
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
class Infinite:
    def __str__(self) -> str:
        return r"\infty "

#> 5ºLEVEL -> VARIABLE
@dataclass
class Variable:
    representation: str
    def __str__(self) -> str:
        curated = self.representation
        for source, replace in VARIABLES.items(): curated = curated.replace(source, replace)
        identifier = CONVERSION[types[self.representation] if self.representation in types else None](curated)
        return identifier

#> 5ºLEVEL -> NEST
@dataclass
class Nest:
    expression: str
    def __str__(self) -> str:
        inside = self.expression if self.expression else r"\, "
        return fr"\left( {inside}\right) "

#> 5ºLEVEL -> TENSOR
@dataclass
class Tensor:
    values: list[str]
    def __str__(self) -> str:
        inside = r"\; " if len(self.values) == 0 else r"\\ ".join(self.values)
        return fr"\begin{{bmatrix}}{inside}\end{{bmatrix}}"

#> 5ºLEVEL -> NUMBER
@dataclass
class Number:
    whole: str
    decimal: str
    def __str__(self) -> str:
        decimal = f".{self.decimal}" if self.decimal else ""
        return f"{self.whole}{decimal}"