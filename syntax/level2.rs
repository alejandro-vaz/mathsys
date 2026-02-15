//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    take, dispatch
};

//> HEAD -> LOCAL
use super::{
    level3::Level3,
    super::{
        backends::traits::{Backends, Spawn},
        solver::nonterminal::{NonTerminal, Item}
    }
};


//^
//^ 2ºLEVEL
//^

//> 2ºLEVEL -> NAMESPACE
#[dispatch(Backends)]
#[derive(Debug, Clone)]
pub enum Level2 {
    Expression
}

//> 2ºLEVEL -> EXPRESSION
#[derive(Debug, Clone)]
pub struct Expression {
    pub terms: Vec<(Vec<bool>, Level3)>
} impl Backends for Expression {
    fn latex(&self) -> String {return self.terms.iter().map(|term| term.0.iter().map(|each| if *each {'+'} else {'-'}).collect::<String>() + &term.1.latex()).collect::<String>()}
} impl Spawn for Expression {fn summon(items: Vec<Item>) -> NonTerminal {
    let mut terms = Vec::new();
    let mut current = Vec::new();
    for item in items {match item {
        Item::Token(token) => current.push(token.value == "+"),
        Item::NonTerminal(NonTerminal::Level3(level3)) => terms.push((take(&mut current), level3)),
        other => panic!()
    }};
    return NonTerminal::Level2(Level2::Expression(Self {
        terms: terms
    }));
}}