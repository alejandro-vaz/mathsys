//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    level1::Level1,
    types::{
        Spawn,
        Item,
        NonTerminal
    },
    context::Context
};

//> HEAD -> LIBUTILS
use libutils::active_reporting::Report;

//> HEAD -> CRATE
use crate::{
    Interpreter,
    Resolver
};


//^
//^ START
//^

//> START -> CLASS
#[derive(Clone, Debug)]
pub struct Start<'valid> {
    pub stream: Vec<Level1<'valid>>
} impl<'valid> Spawn<'valid> for Start<'valid> {
    fn spawn(
        children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
        _filename: &'valid str
    ) -> Option<NonTerminal<'valid>> {return Some(NonTerminal::Start(Self {
        stream: children.into_iter().map(|item| {
            item.into_non_terminal().unwrap().into_level1().unwrap()
        }).collect()
    }))}
}