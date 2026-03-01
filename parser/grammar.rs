//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Map, 
    LazyLock
};

//> HEAD -> LOCAL
use super::{
    super::{
        syntax::object::Object,
        tokenizer::token::{
            Kind,
            BindedToken
        }
    },
    extensor::Extensor
};


//^
//^ EBNF
//^

//> EBNF -> SYNTAX
pub(super) static GRAMMAR: LazyLock<Map<Rule, Vec<Vec<Symbol>>>> = LazyLock::new(|| Extensor::run("
//> EBNF -> START
Start -> (NEWLINES? Level1 (NEWLINES Level1 SPACES?)*)? NEWLINES? ENDOFFILE

//> EBNF -> 1ºLEVEL
Definition -> Variable DEFINITION Level2
Function -> Variable OPEN (Variable (COMMA Variable)* SPACES?)? CLOSE DEFINITION Level2
Node -> Level2
Equation -> Level2 EQUALITY Level2
Use -> USE MODULE

//> EBNF -> 2ºLEVEL
Expression -> (SIGN SPACES?)* Level3 ((SIGN)+ Level3)*

//> EBNF -> 3ºLEVEL
Term -> Level4 ((OPERATOR)? Level4)*

//> EBNF -> 4ºLEVEL
Factor -> Level5 (EXPONENTIATION Level2 EXPONENTIATION)?
Limit -> LIMIT Variable TO Level2 SIGN? OF Nest (EXPONENTIATION Level2 EXPONENTIATION)?

//> EBNF -> 5ºLEVEL
Infinite -> INFINITE
Variable -> IDENTIFIER
Nest -> OPEN Level2? CLOSE
Tensor -> ENTER (Level2 (COMMA Level2)* SPACES?)? EXIT
Whole -> NUMBER
Absolute -> PIPE Level2 PIPE
Undefined -> UNDEFINED
Rational -> RATIONAL
Call -> Variable OPEN (Level2 (COMMA Level2)* SPACES?)? CLOSE

//> EBNF -> LEVELS
Level1 -> Definition | Function | Node | Equation | Use
Level2 -> Expression
Level3 -> Term
Level4 -> Factor | Limit
Level5 -> Infinite | Variable | Nest | Tensor | Whole | Absolute | Undefined | Rational | Call
"));


//^
//^ GRAMMAR
//^

//> GRAMMAR -> RULE
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub(super) enum Rule {
    NonTerminal(Object),
    Internal(u8)
} impl From<&str> for Rule {fn from(value: &str) -> Self {return if let Some(internal) = value.strip_prefix("$") {
    Rule::Internal(internal.parse().unwrap())
} else {
    Rule::NonTerminal(value.parse().unwrap())
}}} impl Into<Symbol> for Rule {fn into(self) -> Symbol {return match self {
    Rule::NonTerminal(object) => Symbol::NonTerminal(object),
    Rule::Internal(code) => Symbol::Internal(code)
}}} impl<'always> Into<Part<'always>> for Rule {fn into(self) -> Part<'always> {return match self {
    Rule::Internal(code) => Part::Internal(code),
    Rule::NonTerminal(object) => Part::NonTerminal(object)
}}}

//> GRAMMAR -> SYMBOL
#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub(super) enum Symbol {
    NonTerminal(Object),
    Internal(u8),
    Kind(Kind)
} impl From<&str> for Symbol {fn from(value: &str) -> Self {return if let Ok(kind) = value.parse::<Kind>() {Self::Kind(kind)} else {Rule::from(value).into()}}}

//> GRAMMAR -> PART
#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) enum Part<'parsing> {
    NonTerminal(Object),
    Internal(u8),
    Token(BindedToken<'parsing>)
}