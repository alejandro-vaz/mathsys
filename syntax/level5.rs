//^
//^ HEAD
//^

use core::panic;

//> HEAD -> PRELUDE
use crate::prelude::{
    dispatch
};

//> HEAD -> LOCAL
use super::{
    level2::Level2,
    super::{
        backends::{
            Backends, 
            Spawn,
            latex::augmentVariables
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
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> NAMESPACE
#[dispatch(Backends)]
#[derive(Debug, Clone)]
pub(crate) enum Level5 {
    Infinite,
    Variable,
    Nest,
    Tensor,
    Whole,
    Absolute,
    Undefined,
    Rational,
    Call
}

//> 5ºLEVEL -> INFINITE
#[derive(Debug, Clone)]
pub(crate) struct Infinite {} impl Backends for Infinite {
    fn latex(&self) -> String {return r"\infty ".to_string()}
} impl Spawn for Infinite {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Infinite(Self {}));
}}

//> 5ºLEVEL -> VARIABLE
#[derive(Debug, Clone)]
pub(crate) struct Variable {
    pub(crate) name: String
} impl Backends for Variable {
    fn latex(&self) -> String {return augmentVariables(&self.name)}
} impl Spawn for Variable {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Variable(Self {
        name: if let Item::Token(token) = items.into_iter().next().unwrap() {token.value.to_string()} else {panic!()}
    }));
}}

//> 5ºLEVEL -> NEST
#[derive(Debug, Clone)]
pub(crate) struct Nest {
    pub(crate) value: Option<Level2>
} impl Backends for Nest {
    fn latex(&self) -> String {
        let inside = if let Some(level2) = &self.value {&level2.latex()} else {""};
        return format!(r"\left( {inside}\right) ")
    }
} impl Spawn for Nest {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Nest(Self {
        value: if let Some(Item::NonTerminal(NonTerminal::Level2(level2))) = items.into_iter().next() {Some(level2)} else {None}
    }));
}}

//> 5ºLEVEL -> TENSOR
#[derive(Debug, Clone)]
pub(crate) struct Tensor {
    pub(crate) values: Vec<Level2>
} impl Backends for Tensor {
    fn latex(&self) -> String {
        let inside = if self.values.len() == 0 {r"\; "} else {&self.values.iter().map(|value| value.latex()).collect::<Vec<String>>().join(r"\\ ")};
        return format!(r"\begin{{bmatrix}}{inside}\end{{bmatrix}}");
    }
} impl Spawn for Tensor {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Tensor(Self {
        values: items.into_iter().map(|item| if let Item::NonTerminal(NonTerminal::Level2(level2)) = item {level2} else {panic!()}).collect()
    }));
}}

//> 5ºLEVEL -> WHOLE
#[derive(Debug, Clone)]
pub(crate) struct Whole {
    pub(crate) number: String
} impl Backends for Whole {
    fn latex(&self) -> String {return self.number.clone()}
} impl Spawn for Whole {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Whole(Self {
        number: if let Item::Token(token) = items.into_iter().next().unwrap() {token.value.to_string()} else {panic!()}
    }));
}}

//> 5ºLEVEL -> ABSOLUTE
#[derive(Debug, Clone)]
pub(crate) struct Absolute {
    pub(crate) value: Level2
} impl Backends for Absolute {
    fn latex(&self) -> String {return format!(r"\left| {}\right| ", self.value.latex())}
} impl Spawn for Absolute {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Absolute(Self {
        value: if let Item::NonTerminal(NonTerminal::Level2(level2)) = items.into_iter().next().unwrap() {level2} else {panic!()}
    }));
}}

//> 5ºLEVEL -> UNDEFINED
#[derive(Debug, Clone)]
pub(crate) struct Undefined {} impl Backends for Undefined {
    fn latex(&self) -> String {return r"\left. ?\right. ".to_string()}
} impl Spawn for Undefined {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Undefined(Self {}));
}}

//> 5ºLEVEL -> RATIONAL
#[derive(Debug, Clone)]
pub(crate) struct Rational {
    pub(crate) number: String
} impl Backends for Rational {
    fn latex(&self) -> String {return self.number.clone()}
} impl Spawn for Rational {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    return NonTerminal::Level5(Level5::Rational(Self {
        number: if let Item::Token(token) = items.into_iter().next().unwrap() {token.value.to_string()} else {panic!()}
    }));
}}

//> 5ºLEVEL -> CALL
#[derive(Debug, Clone)]
pub(crate) struct Call {
    pub(crate) to: Variable,
    pub(crate) with: Vec<Level2>
} impl Backends for Call {
    fn latex(&self) -> String {return format!("{}({})", self.to.latex(), self.with.iter().map(|argument| argument.latex()).collect::<Vec<String>>().join(","))}
} impl Spawn for Call {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    let mut iterator = items.into_iter();
    return NonTerminal::Level5(Level5::Call(Self {
        to: if let Item::NonTerminal(NonTerminal::Level5(Level5::Variable(variable))) = iterator.next().unwrap() {variable} else {panic!()},
        with: iterator.map(|element| if let Item::NonTerminal(NonTerminal::Level2(level2)) = element {level2} else {panic!()}).collect()
    }))
}}