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
    super::{
        runtime::traits::{Backends, Spawn},
        solver::nonterminal::{NonTerminal, Item}
    }
};


//^
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> NAMESPACE
#[dispatch(Backends)]
#[derive(Debug, Clone)]
pub enum Level5 {
    Infinite,
    Variable,
    Nest,
    Tensor,
    Whole,
    Absolute,
    Undefined,
    Rational
}

//> 5ºLEVEL -> INFINITE
#[derive(Debug, Clone)]
pub struct Infinite {} impl Backends for Infinite {
    fn latex(&self) -> String {return r"\infty ".to_string()}
} impl Spawn for Infinite {fn summon(items: Vec<Item>) -> NonTerminal {return NonTerminal::Level5(Level5::Infinite(Self {}))}}

//> 5ºLEVEL -> VARIABLE
#[derive(Debug, Clone)]
pub struct Variable {
    name: String
} impl Backends for Variable {
    fn latex(&self) -> String {return self.name.clone()}
} impl Spawn for Variable {fn summon(items: Vec<Item>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Variable(Self {
        name: if let Item::Token(token) = items.into_iter().next().unwrap() {token.value.to_string()} else {panic!()}
    }));
}}

//> 5ºLEVEL -> NEST
#[derive(Debug, Clone)]
pub struct Nest {
    value: Option<Level2>
} impl Backends for Nest {
    fn latex(&self) -> String {
        let inside = if let Some(level2) = &self.value {&level2.latex()} else {""};
        return format!(r"\left( {inside}\right) ")
    }
} impl Spawn for Nest {fn summon(items: Vec<Item>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Nest(Self {
        value: if let Some(Item::NonTerminal(NonTerminal::Level2(level2))) = items.into_iter().next() {Some(level2)} else {None}
    }));
}}

//> 5ºLEVEL -> TENSOR
#[derive(Debug, Clone)]
pub struct Tensor {
    values: Vec<Level2>
} impl Backends for Tensor {
    fn latex(&self) -> String {
        let inside = if self.values.len() == 0 {r"\; "} else {&self.values.iter().map(|value| value.latex()).collect::<Vec<String>>().join(r"\\ ")};
        return format!(r"\begin{{bmatrix}}{inside}\end{{bmatrix}}");
    }
} impl Spawn for Tensor {fn summon(items: Vec<Item>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Tensor(Self {
        values: items.into_iter().map(|item| if let Item::NonTerminal(NonTerminal::Level2(level2)) = item {level2} else {panic!()}).collect()
    }))
}}

//> 5ºLEVEL -> WHOLE
#[derive(Debug, Clone)]
pub struct Whole {
    number: String
} impl Backends for Whole {
    fn latex(&self) -> String {return self.number.clone()}
} impl Spawn for Whole {fn summon(items: Vec<Item>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Whole(Self {
        number: if let Item::Token(token) = items.into_iter().next().unwrap() {token.value.to_string()} else {panic!()}
    }));
}}

//> 5ºLEVEL -> ABSOLUTE
#[derive(Debug, Clone)]
pub struct Absolute {
    value: Level2
} impl Backends for Absolute {
    fn latex(&self) -> String {return format!(r"\left| {}\right| ", self.value.latex())}
} impl Spawn for Absolute {fn summon(items: Vec<Item>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Absolute(Self {
        value: if let Item::NonTerminal(NonTerminal::Level2(level2)) = items.into_iter().next().unwrap() {level2} else {panic!()}
    }))
}}

//> 5ºLEVEL -> UNDEFINED
#[derive(Debug, Clone)]
pub struct Undefined {} impl Backends for Undefined {
    fn latex(&self) -> String {return r"\left. ?\right. ".to_string()}
} impl Spawn for Undefined {fn summon(items: Vec<Item>) -> NonTerminal {return NonTerminal::Level5(Level5::Undefined(Self {}))}}

//> 5ºLEVEL -> RATIONAL
#[derive(Debug, Clone)]
pub struct Rational {} impl Backends for Rational {
    fn latex(&self) -> String {return "".to_string()}
} impl Spawn for Rational {fn summon(items: Vec<Item>) -> NonTerminal {return NonTerminal::Level5(Level5::Rational(Self {}))}}