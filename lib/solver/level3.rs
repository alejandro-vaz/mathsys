//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> SUPER
use super::{
    level4::Level4,
    types::{
        Spawn,
        Item,
        NonTerminal
    },
    context::Context
};

//> HEAD -> CRATE
use crate::{
    tokenizer::token::Token,
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
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> ENUM
#[enum_dispatch(LaTeX)]
#[derive(Clone, EnumAsInner)]
pub enum Level3<'valid> {
    Term(Term<'valid>)
}

//> 3ºLEVEL -> TERM
#[derive(Clone)]
pub struct Term<'valid> {
    pub numerator: Vec<Level4<'valid>>,
    pub denominator: Vec<Level4<'valid>>
} impl<'valid> Spawn<'valid> for Term<'valid> {
    fn spawn(
        children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<Same>, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {
        let mut numerator = Vec::new();
        let mut denominator = Vec::new();
        let mut location = true;
        for child in children {match child {
            Item::Token(Token {value, ..}) => location = value == "*",
            Item::NonTerminal(NonTerminal::Level4(level4)) => (if location {&mut numerator} else {&mut denominator}).push(level4),
            _ => unreachable!()
        }};
        return Some(NonTerminal::Level3(Level3::Term(Self {
            numerator: numerator,
            denominator: denominator
        })));
    }
}