//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    dispatch
};

//> HEAD -> LOCAL
use super::{
    level4::Level4,
    super::{
        Settings,
        Issue,
        backends::{
            Backends, 
            Spawn
        },
        solver::{
            nonterminal::{
                NonTerminal, 
                Item
            },
            context::Context
        }
    }
};


//^
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> NAMESPACE
#[dispatch(Backends, Contextualize)]
#[derive(Debug, Clone)]
pub(crate) enum Level3 {
    Term
}

//> 3ºLEVEL -> TERM
#[derive(Debug, Clone)]
pub(crate) struct Term {
    pub(crate) numerator: Vec<Level4>,
    pub(crate) denominator: Vec<Level4>
} impl Backends for Term {
    fn latex(&self) -> String {
        let numerator = self.numerator.iter().map(|factor| factor.latex()).collect::<Vec<String>>().join(r"\cdot ");
        let denominator = self.denominator.iter().map(|factor| factor.latex()).collect::<Vec<String>>().join(r"\cdot ");
        return if denominator.is_empty() {numerator} else {format!(r"\frac{{{}}}{{{}}}", numerator, denominator)};
    }
} impl Spawn for Term {fn spawn(items: Vec<Item>, settings: &Settings, context: &mut Context) -> Result<NonTerminal, Issue> {
    let mut numerator = Vec::new();
    let mut denominator = Vec::new();
    let mut location = true;
    for item in items {match item {
        Item::Token(token) => location = token.value == "*",
        Item::NonTerminal(NonTerminal::Level4(level4)) => (if location {&mut numerator} else {&mut denominator}).push(level4),
        other => panic!()
    }}
    return Ok(NonTerminal::Level3(Level3::Term(Self {
        numerator: numerator,
        denominator: denominator
    })));
}}