//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::nonterminal::{Spawn, NonTerminal, Item};
use super::level2::Level2;
use super::level5::{Level5, Variable, Nest};


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
    value: Level5,
    exponent: Option<Level2>
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