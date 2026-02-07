//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::nonterminal::{NonTerminal, Spawn};
use super::level1::Level1;


//^
//^ START
//^

//> START -> CLASS
#[derive(Debug, Clone)]
pub struct Start {
    stream: Vec<Level1>
} impl Spawn for Start {fn summon(items: Vec<NonTerminal>) -> NonTerminal {return NonTerminal::Start(Self {
    stream: items.into_iter().map(|element| if let NonTerminal::Level1(level1) = element {level1} else {panic!()}).collect()
})}}