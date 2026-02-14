//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::super::solver::nonterminal::{Item, Spawn, NonTerminal};
use super::level1::Level1;


//^
//^ START
//^

//> START -> CLASS
#[derive(Debug, Clone)]
pub struct Start {
    stream: Vec<Level1>
} impl Spawn for Start {fn summon(items: Vec<Item>) -> NonTerminal {return NonTerminal::Start(Self {
    stream: items.into_iter().map(|element| if let Item::NonTerminal(NonTerminal::Level1(level1)) = element {level1} else {panic!("{element:?}")}).collect()
})}}