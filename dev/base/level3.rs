//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::nonterminal::{Spawn, NonTerminal, Item};
use super::level4::Level4;


//^
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> NAMESPACE
#[derive(Debug, Clone)]
pub enum Level3 {
    Term(Term)
}

//> 3ºLEVEL -> TERM
#[derive(Debug, Clone)]
pub struct Term {
    numerator: Vec<Level4>,
    denominator: Vec<Level4>
} impl Spawn for Term {fn summon(items: Vec<Item>) -> NonTerminal {
    let mut numerator = Vec::new();
    let mut denominator = Vec::new();
    let mut location = true;
    for item in items {match item {
        Item::Token(token) => location = token.value == "*",
        Item::NonTerminal(NonTerminal::Level4(level4)) => (if location {&mut numerator} else {&mut denominator}).push(level4),
        other => panic!()
    }}
    return NonTerminal::Level3(Level3::Term(Self {
        numerator: numerator,
        denominator: denominator
    }))
}}