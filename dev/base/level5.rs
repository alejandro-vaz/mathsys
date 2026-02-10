//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::nonterminal::{Spawn, NonTerminal, Item};
use super::level2::Level2;


//^
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> NAMESPACE
#[derive(Debug, Clone)]
pub enum Level5 {
    Infinite(Infinite),
    Variable(Variable),
    Nest(Nest),
    Tensor(Tensor),
    Whole(Whole),
    Absolute(Absolute),
    Undefined(Undefined),
    Rational(Rational),
    Casts(Casts)
}

//> 5ºLEVEL -> INFINITE
#[derive(Debug, Clone)]
pub struct Infinite {} impl Spawn for Infinite {fn summon(items: Vec<Item>) -> NonTerminal {return NonTerminal::Level5(Level5::Infinite(Self {}))}}

//> 5ºLEVEL -> VARIABLE
#[derive(Debug, Clone)]
pub struct Variable {
    name: String
} impl Spawn for Variable {fn summon(items: Vec<Item>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Variable(Self {
        name: if let Item::Token(token) = items.into_iter().next().unwrap() {token.value.to_string()} else {panic!()}
    }));
}}

//> 5ºLEVEL -> NEST
#[derive(Debug, Clone)]
pub struct Nest {
    value: Option<Level2>
} impl Spawn for Nest {fn summon(items: Vec<Item>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Nest(Self {
        value: if let Some(Item::NonTerminal(NonTerminal::Level2(level2))) = items.into_iter().next() {Some(level2)} else {None}
    }));
}}

//> 5ºLEVEL -> TENSOR
#[derive(Debug, Clone)]
pub struct Tensor {
    values: Vec<Level2>
} impl Spawn for Tensor {fn summon(items: Vec<Item>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Tensor(Self {
        values: items.into_iter().map(|item| if let Item::NonTerminal(NonTerminal::Level2(level2)) = item {level2} else {panic!()}).collect()
    }))
}}

//> 5ºLEVEL -> WHOLE
#[derive(Debug, Clone)]
pub struct Whole {} impl Spawn for Whole {fn summon(items: Vec<Item>) -> NonTerminal {return NonTerminal::Level5(Level5::Whole(Self {}))}}

//> 5ºLEVEL -> ABSOLUTE
#[derive(Debug, Clone)]
pub struct Absolute {
    value: Level2
} impl Spawn for Absolute {fn summon(items: Vec<Item>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Absolute(Self {
        value: if let Item::NonTerminal(NonTerminal::Level2(level2)) = items.into_iter().next().unwrap() {level2} else {panic!()}
    }))
}}

//> 5ºLEVEL -> UNDEFINED
#[derive(Debug, Clone)]
pub struct Undefined {} impl Spawn for Undefined {fn summon(items: Vec<Item>) -> NonTerminal {return NonTerminal::Level5(Level5::Undefined(Self {}))}}

//> 5ºLEVEL -> RATIONAL
#[derive(Debug, Clone)]
pub struct Rational {} impl Spawn for Rational {fn summon(items: Vec<Item>) -> NonTerminal {return NonTerminal::Level5(Level5::Rational(Self {}))}}

//> 5ºLEVEL -> CASTS
#[derive(Debug, Clone)]
pub struct Casts {
    element: Box<Level5>
} impl Spawn for Casts {fn summon(items: Vec<Item>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Casts(Self {
        element: if let Item::NonTerminal(NonTerminal::Level5(level5)) = items.into_iter().next().unwrap() {Box::new(level5)} else {panic!()}
    }))
}}