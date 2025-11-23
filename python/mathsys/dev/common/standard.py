#^
#^  HEAD
#^

#> HEAD -> DATACLASSES
from ..main.parser import (
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
#^  DATA
#^

#> DATA -> LIBRARY
data = Start(statements = [Definition(
    objectType = "Number",
    identifier = Variable(representation = "pi"),
    expression = Expression(
        signs = [None],
        terms = [Term(
            numerator = [Factor(
                value = Number(
                    whole = "3",
                    decimal = "14"
                ),
                exponent = None
            )],
            denominator = []
        )]
    )
)])