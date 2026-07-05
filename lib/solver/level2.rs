//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> SUPER
use super::{
    level3::Level3,
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

//> HEAD -> CORE
use core::mem::take;

//> HEAD -> LIBUTILS
use libutils::report::{
    Report,
    Same
};

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ 2ºLEVEL
//^

//> 2ºLEVEL -> ENUM
#[enum_dispatch(LaTeX)]
#[derive(Clone, EnumAsInner)]
pub enum Level2<'valid> {
    Expression(Expression<'valid>)
}

//> 2ºLEVEL -> EXPRESSION
#[derive(Clone)]
pub struct Expression<'valid> {
    pub terms: Vec<(Vec<bool>, Level3<'valid>)>
} impl<'valid> Spawn<'valid> for Expression<'valid> {
    fn spawn(
        children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<Same>, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {
        let mut terms = Vec::new();
        let mut current = Vec::new();
        for child in children {match child {
            Item::Token(Token {value, ..}) => current.push(value == "+"),
            Item::NonTerminal(NonTerminal::Level3(level3)) => terms.push((take(&mut current), level3)),
            _ => unreachable!()
        }};
        return Some(NonTerminal::Level2(Level2::Expression(Self {
            terms: terms
        })))
    }
}