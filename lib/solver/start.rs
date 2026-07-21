//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    spawn::Spawn,
    item::Item,
    nonterminal::NonTerminal,
    context::Context
};

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemio::SystemIO
};

//> HEAD -> CRATE
use crate::{
    failure::Failure,
    syntax::Start
};


//^
//^ START
//^

//> START -> SPAWN
impl<'valid> Spawn<'valid> for Start<'valid> {
    fn spawn(
        children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {return NonTerminal::Start(Self {
        stream: children.into_iter().map(|item| {
            item.into_non_terminal().unwrap().into_level1().unwrap()
        }).collect()
    })}
}