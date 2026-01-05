#^
#^  HEAD
#^

#> HEAD -> TOKENS
from __future__ import annotations
from tokenizer import IDENTIFIER, MODULE, NUMBER, OPERATOR, RATIONAL, SIGN, TYPE, _BINDING, _CLOSE, _COMMA, _ENTER, _EQUALITY, _EXIT, _EXPONENTIATION, _INF, _LIM, _NEWLINES, _OF, _OPEN, _PIPE, _SPACES, _TO, _UNDEFINED, _USE, Token
from dataclasses import dataclass


#^
#^  NONTERMINAL
#^

#> NONTERMINAL -> REFERENCE
class NonTerminal: pass

#> NONTERMINAL -> START
class Start(NonTerminal): pass

#> NONTERMINAL -> 1ºLEVEL
class Declaration(NonTerminal): pass
class Definition(NonTerminal): pass
class Annotation(NonTerminal): pass
class Node(NonTerminal): pass
class Equation(NonTerminal): pass
class Use(NonTerminal): pass

#> NONTERMINAL -> 2ºLEVEL
class Expression(NonTerminal): pass

#> NONTERMINAL -> 3ºLEVEL
class Term(NonTerminal): pass

#> NONTERMINAL -> 4ºLEVEL
class Factor(NonTerminal): pass
class Limit(NonTerminal): pass

#> NONTERMINAL -> 5ºLEVEL
class Infinite(NonTerminal): pass
class Variable(NonTerminal): pass
class Nest(NonTerminal): pass
class Tensor(NonTerminal): pass
class Whole(NonTerminal): pass
class Absolute(NonTerminal): pass
class Undefined(NonTerminal): pass
class Rational(NonTerminal): pass
class Casts(NonTerminal): pass

#> NONTERMINAL -> LEVELS
class Level1(NonTerminal): pass
class Level2(NonTerminal): pass
class Level3(NonTerminal): pass
class Level4(NonTerminal): pass
class Level5(NonTerminal): pass


#^
#^  MODIFIERS
#^

#> MODIFIERS -> BRANCH
@dataclass(frozen = True)
class Branch:
    options: tuple[type[NonTerminal], ...]

#> MODIFIERS -> OPTIONAL
@dataclass(frozen = True)
class Optional:
    element: type[NonTerminal | Token] | Sequence | More

#> MODIFIERS -> SEQUENCE
@dataclass(frozen = True)
class Sequence:
    items: tuple[type[Token | NonTerminal] | Optional | More, ...]

#> MODIFIERS -> MORE
@dataclass(frozen = True)
class More:
    of: Sequence


#^
#^  SYNTAX
#^

#> SYNTAX -> DEFINITION
syntax = {
    #~ SYNTAX -> START
    Start: (Optional(_NEWLINES), Optional(Sequence(
        (Level1, Optional(_SPACES), Optional(More(Sequence(
            (_NEWLINES, Level1, Optional(_SPACES))
        ))), Optional(_NEWLINES))
    ))),
    #~ SYNTAX -> 1ºLEVEL
    Declaration: (Optional(Sequence((TYPE, _SPACES))), Variable, Optional(_SPACES), _EQUALITY, Optional(_SPACES), Level2),
    Definition: (Optional(Sequence((TYPE, _SPACES))), Variable, Optional(_SPACES), _BINDING, Optional(_SPACES), Level2),
    Annotation: (TYPE, _SPACES, Variable, Optional(More(Sequence((Optional(_SPACES), _COMMA, Optional(_SPACES), Variable))))),
    Node: (Level2,),
    Equation: (Level2, Optional(_SPACES), _EQUALITY, Optional(_SPACES), Level2),
    Use: (_USE, _SPACES, MODULE),
    #~ SYNTAX -> 2ºLEVEL
    Expression: (Optional(More(Sequence((SIGN, Optional(_SPACES))))), Level3, Optional(More(Sequence((More(Sequence((Optional(_SPACES), SIGN))), Optional(_SPACES), Level3))))),
    #~ SYNTAX -> 3ºLEVEL
    Term: (Level4, Optional(More(Sequence((Optional(Sequence((Optional(_SPACES), OPERATOR))), Optional(_SPACES), Level4))))),
    #~ SYNTAX -> 4ºLEVEL
    Factor: (Level5, Optional(Sequence((_EXPONENTIATION, Optional(_SPACES), Level2, Optional(_SPACES), _EXPONENTIATION)))),
    Limit: (_LIM, _SPACES, Variable, Optional(_SPACES), _TO, Optional(_SPACES), Level2, Optional(SIGN), _SPACES, _OF, _SPACES, Nest, Optional(Sequence((_EXPONENTIATION, Optional(_SPACES), Level2, Optional(_SPACES), _EXPONENTIATION)))),
    #~ SYNTAX -> 5ºLEVEL
    Infinite: (_INF,),
    Variable: (IDENTIFIER,),
    Nest: (_OPEN, Optional(_SPACES), Optional(Level2), Optional(_SPACES), _CLOSE),
    Tensor: (_ENTER, Optional(_SPACES), Optional(Sequence(
        (Level2, Optional(More(Sequence(
            (Optional(_SPACES), _COMMA, Optional(_SPACES), Level2)
        ))), Optional(_SPACES))
    )), _EXIT),
    Whole: (NUMBER,),
    Absolute: (_PIPE, Optional(_SPACES), Level2, Optional(_SPACES), _PIPE),
    Undefined: (_UNDEFINED,),
    Rational: (RATIONAL,),
    Casts: (Level5, TYPE),
    #~ SYNTAX -> LEVELS
    Level1: (Branch((Declaration, Definition, Annotation, Node, Equation, Use)),),
    Level2: (Expression,),
    Level3: (Term,),
    Level4: (Branch((Factor, Limit)),),
    Level5: (Branch((Infinite, Variable, Nest, Tensor, Whole, Absolute, Undefined, Rational, Casts)),)
}