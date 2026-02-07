//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::nonterminal::{Spawn, NonTerminal};
use super::level2::Level2;


//^
//^ 4ºLEVEL
//^

//> 4ºLEVEL -> NAMESPACE
#[derive(Debug, Clone)]
pub enum Level4 {
    Factor(Factor),
    Limit(Limit)
}

//> 4ºLEVEL -> FACTOR
#[derive(Debug, Clone)]
pub struct Factor {
    exponent: Option<Level2>
} impl Spawn for Factor {fn summon(items: Vec<NonTerminal>) -> NonTerminal {return NonTerminal::Level4(Level4::Factor(Self {
    exponent: None
}))}}

//> 4ºLEVEL -> LIMIT
#[derive(Debug, Clone)]
pub struct Limit {
    exponent: Option<Level2>
} impl Spawn for Limit {fn summon(items: Vec<NonTerminal>) -> NonTerminal {return NonTerminal::Level4(Level4::Limit(Self {
    exponent: None
}))}}