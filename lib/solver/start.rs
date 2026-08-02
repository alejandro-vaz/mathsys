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

//> HEAD -> CRATE
use crate::syntax::Start;


//^
//^ START
//^

//> START -> SPAWN
impl<'valid> Spawn<'valid> for Start<'valid> {
    fn spawn(
        children: Vec<Item<'_, 'valid>>, 
        _context: &mut Context<'valid>, 
        _filename: &'valid str
    ) -> NonTerminal<'valid> {return NonTerminal::Start(Self {
        stream: children.into_iter().map(|item| {
            item.into_non_terminal().unwrap().into_level1().unwrap()
        }).collect()
    })}
}