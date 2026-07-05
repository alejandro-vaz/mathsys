//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> SUPER
use super::{
    level2::Level2,
    level5::{
        Level5,
        Variable,
        Nest
    },
    types::{
        Spawn,
        Item,
        NonTerminal
    },
    context::Context
};

//> HEAD -> CRATE
use crate::{
    latex::LaTeX,
    Interpreter,
    Resolver
};

//> HEAD -> LIBUTILS
use libutils::report::{
    Report,
    Same
};

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ 4ºLEVEL
//^

//> 4ºLEVEL -> ENUM
#[enum_dispatch(LaTeX)]
#[derive(Clone, EnumAsInner)]
pub enum Level4<'valid> {
    Factor(Factor<'valid>),
    Limit(Limit<'valid>)
}

//> 4ºLEVEL -> FACTOR
#[derive(Clone)]
pub struct Factor<'valid> {
    pub value: Level5<'valid>,
    pub exponent: Option<Level2<'valid>>
} impl<'valid> Spawn<'valid> for Factor<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<Same>, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Level4(Level4::Factor(Self {
        value: children.remove(0).into_non_terminal().ok().unwrap().into_level5().ok().unwrap(),
        exponent: children.pop().map(|item| item.into_non_terminal().ok().unwrap().into_level2().ok().unwrap())
    })))}
}

//> 4ºLEVEL -> LIMIT
#[derive(Clone)]
pub struct Limit<'valid> {
    pub variable: Variable<'valid>,
    pub approach: Level2<'valid>,
    pub direction: Option<bool>,
    pub nest: Nest<'valid>,
    pub exponent: Option<Level2<'valid>>
} impl<'valid> Spawn<'valid> for Limit<'valid> {
    fn spawn(
        children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<Same>, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {
        let mut iterator = children.into_iter();
        let variable = iterator.next().unwrap().into_non_terminal().ok().unwrap().into_level5().ok().unwrap().into_variable().ok().unwrap();
        let approach = iterator.next().unwrap().into_non_terminal().ok().unwrap().into_level2().ok().unwrap();
        let mut next = iterator.next();
        let direction = if let Some(Item::Token(token)) = next {next = iterator.next(); Some(token.value == "+")} else {None};
        let Some(Item::NonTerminal(NonTerminal::Level5(Level5::Nest(nest)))) = next else {panic!()};
        let exponent = if let Some(Item::NonTerminal(NonTerminal::Level2(level2))) = iterator.next() {Some(level2)} else {None};
        return Some(NonTerminal::Level4(Level4::Limit(Self {
            variable: variable,
            approach: approach,
            direction: direction,
            nest: nest,
            exponent: exponent
        })));
    }
}