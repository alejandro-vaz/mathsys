//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::nonterminal::{Spawn, NonTerminal};
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
} impl Spawn for Term {fn summon(items: Vec<NonTerminal>) -> NonTerminal {return NonTerminal::Level3(Level3::Term(Self {
    numerator: Vec::new(),
    denominator: Vec::new()
}))}}