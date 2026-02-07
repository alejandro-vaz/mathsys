//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::nonterminal::{Spawn, NonTerminal};
use super::level3::Level3;


//^
//^ 2ºLEVEL
//^

//> 2ºLEVEL -> NAMESPACE
#[derive(Debug, Clone)]
pub enum Level2 {
    Expression(Expression)
}

//> 2ºLEVEL -> EXPRESSION
#[derive(Debug, Clone)]
pub struct Expression {
    terms: Vec<(Vec<bool>, Level3)>
} impl Spawn for Expression {fn summon(items: Vec<NonTerminal>) -> NonTerminal {return NonTerminal::Level2(Level2::Expression(Self {
    terms: Vec::new()
}))}}