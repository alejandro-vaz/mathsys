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
#^  SYNTAX
#^

syntax = r"""
#> SYNTAX -> START
Start -> (_NEWLINES? Level1 _SPACES?)* _NEWLINES? _EOF

#> SYNTAX -> 1ºLEVEL
Declaration -> (TYPE _SPACES)? Variable _SPACES? _EQUALITY _SPACES? Level2
Definition -> (TYPE _SPACES)? Variable _SPACES? _BINDING _SPACES? Level2
Annotation -> TYPE _SPACES Variable (_SPACES? _COMMA _SPACES? Variable)*
Node -> Level2
Equation -> Level2 _SPACES? _EQUALITY _SPACES? Level2
Use -> _USE _SPACES MODULE

#> SYNTAX -> 2ºLEVEL
Expression -> (SIGN _SPACES?)* Level3 ((_SPACES? SIGN)+ _SPACES? Level3)*

#> SYNTAX -> 3ºLEVEL
Term -> Level4 ((_SPACES? OPERATOR)? _SPACES? Level4)*

#> SYNTAX -> 4ºLEVEL
Factor -> Level5 (_EXPONENTIATION _SPACES? Level2 _SPACES? _EXPONENTIATION)?
Limit -> _LIM _SPACES Variable _SPACES? _TO _SPACES? Level2 SIGN? _SPACES _OF _SPACES Nest (_EXPONENTIATION _SPACES? Level2 _SPACES? _EXPONENTIATION)?

#> SYNTAX -> 5ºLEVEL
Infinite -> _INF
Variable -> IDENTIFIER
Nest -> _OPEN _SPACES? Level2? _SPACES? _CLOSE
Tensor -> _ENTER _SPACES? (Level2 (_SPACES? _COMMA _SPACES? Level2)* _SPACES?)? _EXIT
Whole -> NUMBER
Absolute -> _PIPE _SPACES? Level2 _SPACES? _PIPE
Undefined -> _UNDEFINED
Rational -> RATIONAL
Casts -> Level5 TYPE

#> SYNTAX -> LEVELS
Level1 -> Declaration | Definition | Annotation | Node | Equation | Use
Level2 -> expression
Level3 -> Term
Level4 -> Factor | Limit
Level5 -> Infinite | Variable | Nest | Tensor | Whole | Absolute | Undefined | Rational | Casts
"""