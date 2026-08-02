//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::failure::Failure;

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
        children: Vec<Item<'_, 'valid>>, 
        context: &mut Context<'valid>, 
        filename: &'valid str
    ) -> NonTerminal<'valid>;
}