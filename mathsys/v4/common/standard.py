#^
#^  HEAD
#^

#> HEAD -> PARSER
from ..parser.dataclasses import (
    #~ PARSER -> START
    Start,
    #~ PARSER -> 1ºLEVEL
    Level1,
    Declaration,
    Definition,
    Annotation,
    Node,
    Equation,
    Comment, 
    #~ PARSER -> 2ºLEVEL
    Level2,
    Expression,
    #~ PARSER -> 3ºLEVEL
    Level3,
    Term,
    #~ PARSER -> 4ºLEVEL
    Level4,
    Factor,
    Limit,
    #~ PARSER -> 5ºLEVEL
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
data = Start(statements = [
    #~ LIBRARY -> PI
    Definition(
        group = "Number",
        variable = Variable(representation = "pi"),
        expression = Expression(
            signs = [None],
            terms = [Term(
                numerator = [Factor(
                    value = Number(whole = "3", decimal = "141592653"),
                    exponent = None
                )],
                denominator = []
            )]
        )
    ),
    #~ LIBRARY -> E
    Definition(
        group = "Number",
        variable = Variable(representation = "e"),
        expression = Expression(
            signs = [None],
            terms = [Term(
                numerator = [Limit(
                    variable = Variable(representation = "n"),
                    approach = Expression(
                        terms = [Term(
                            numerator = [Factor(
                                value = Infinite(),
                                exponent = None
                            )],
                            denominator = []
                        )],
                        signs = [None]
                    ),
                    direction = None,
                    nest = Nest(expression = Expression(
                        terms = [
                            Term(
                                numerator = [Factor(
                                    value = Number(whole = "1", decimal = None),
                                    exponent = None
                                )],
                                denominator = []
                            ),
                            Term(
                                numerator = [Factor(
                                    value = Number(whole = "1", decimal = None),
                                    exponent = None
                                )],
                                denominator = [Factor(
                                    value = Variable(representation = "n"),
                                    exponent = None
                                )]
                            )
                        ],
                        signs = [None, "+"]
                    )),
                    exponent = Expression(
                        terms = [Term(
                            numerator = [Factor(
                                value = Variable(representation = "n"),
                                exponent = None
                            )],
                            denominator = []
                        )],
                        signs = [None]
                    )
                )],
                denominator = []
            )]
        )
    )
])