//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    dispatch
};

//> HEAD -> LOCAL
use super::{
    level2::Level2,
    level5::{Level5, Variable, Nest},
    super::{
        runtime::traits::{Backends, Spawn},
        solver::nonterminal::{NonTerminal, Item}
    }
};


//^
//^ 4ºLEVEL
//^

//> 4ºLEVEL -> NAMESPACE
#[dispatch(Backends)]
#[derive(Debug, Clone)]
pub enum Level4 {
    Factor,
    Limit
}

//> 4ºLEVEL -> FACTOR
#[derive(Debug, Clone)]
pub struct Factor {
    value: Level5,
    exponent: Option<Level2>
} impl Backends for Factor {
    fn latex(&self) -> String {
        let exponent = if let Some(level2) = &self.exponent {&format!("^{{{}}}", level2.latex())} else {""};
        return self.value.latex() + &exponent;
    }
} impl Spawn for Factor {fn summon(items: Vec<Item>) -> NonTerminal {
    let mut iterator = items.into_iter();
    return NonTerminal::Level4(Level4::Factor(Self {
        value: if let Item::NonTerminal(NonTerminal::Level5(level5)) = iterator.next().unwrap() {level5} else {panic!()},
        exponent: if let Some(Item::NonTerminal(NonTerminal::Level2(level2))) = iterator.next() {Some(level2)} else {None}
    }));
}}

//> 4ºLEVEL -> LIMIT
#[derive(Debug, Clone)]
pub struct Limit {
    variable: Variable,
    approach: Level2,
    direction: Option<bool>,
    nest: Nest,
    exponent: Option<Level2>
} impl Backends for Limit {
    fn latex(&self) -> String {
        let direction = if let Some(value) = self.direction {if value {"+"} else {"-"}} else {""};
        let exponent = if let Some(level2) = &self.exponent {&format!("^{{{}}}", level2.latex())} else {""};
        return format!(r"\lim_{{\substack{{{}\to {}{direction}}}}}{exponent}", self.variable.latex(), self.approach.latex());
    }
} impl Spawn for Limit {fn summon(items: Vec<Item>) -> NonTerminal {
    let mut iterator = items.into_iter();
    let Some(Item::NonTerminal(NonTerminal::Level5(Level5::Variable(variable)))) = iterator.next() else {panic!()};
    let Some(Item::NonTerminal(NonTerminal::Level2(approach))) = iterator.next() else {panic!()};
    let mut next = iterator.next();
    let direction = if let Some(Item::Token(token)) = next {next = iterator.next(); Some(token.value == "+")} else {None};
    let Some(Item::NonTerminal(NonTerminal::Level5(Level5::Nest(nest)))) = next else {panic!()};
    let exponent = if let Some(Item::NonTerminal(NonTerminal::Level2(level2))) = iterator.next() {Some(level2)} else {None};
    return NonTerminal::Level4(Level4::Limit(Self {
        variable: variable,
        approach: approach,
        direction: direction,
        nest: nest,
        exponent: exponent
    }));
}}