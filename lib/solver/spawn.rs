//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::failure::Failure;

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemio::SystemIO
};

//> HEAD -> SUPER
use super::{
    item::Item,
    nonterminal::NonTerminal,
    context::Context
};

//> HEAD -> CORE
use core::fmt::Debug;


//^
//^ SPAWN
//^

//> SPAWN -> TRAIT
pub trait Spawn<'valid>: Sized + Clone + Debug + Eq + PartialEq + Ord + PartialOrd {
    fn spawn(
        children: Vec<Item<'valid>>, 
        context: &mut Context<'valid>, 
        report: Report<"">, 
        systemio: &'valid SystemIO<Failure<'valid>>,
        resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        filename: &'valid str
    ) -> NonTerminal<'valid>;
}