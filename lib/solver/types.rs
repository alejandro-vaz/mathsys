//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::{
    tokenizer::token::Token,
    latex::LaTeX,
    Interpreter,
    Resolver
};

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> SUPER
use super::{
    start::Start,
    level1::Level1,
    level2::Level2,
    level3::Level3,
    level4::Level4,
    level5::Level5,
    context::Context
};

//> HEAD -> LIBUTILS
use libutils::report::{
    Same,
    Report
};

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ NONTERMINAL
//^

//> NONTERMINAL -> ENUM
#[enum_dispatch]
#[derive(Clone, EnumAsInner, Debug)]
pub enum NonTerminal<'valid> {
    Start(Start<'valid>),
    Level1(Level1<'valid>),
    Level2(Level2<'valid>),
    Level3(Level3<'valid>),
    Level4(Level4<'valid>),
    Level5(Level5<'valid>)
}

//> NONTERMINAL -> ITEM
#[enum_dispatch]
#[derive(Clone, EnumAsInner, Debug)]
pub enum Item<'valid> {
    NonTerminal(NonTerminal<'valid>),
    Token(Token<'valid>)
}

//> NONTERMINAL -> SUBTREE
#[enum_dispatch]
#[derive(Clone, EnumAsInner)]
pub enum Subtree<'valid> {
    NonTerminal(NonTerminal<'valid>),
    Vec(Vec<Item<'valid>>),
    Token(Token<'valid>)
}

//> NONTERMINAL -> SPAWN
pub trait Spawn<'valid>: Sized + LaTeX {
    fn spawn(
        children: Vec<Item<'valid>>, 
        context: &mut Context<'valid>, 
        report: Report<Same>, 
        interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        filename: &'valid str
    ) -> Option<NonTerminal<'valid>>;
}