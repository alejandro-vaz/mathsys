//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::{
    level1::Level1,
    super::{
        backends::traits::{Spawn, Backends},
        solver::nonterminal::{Item, NonTerminal}
    }
};


//^
//^ START
//^

//> START -> CLASS
#[derive(Debug, Clone)]
pub struct Start {
    stream: Vec<Level1>
} impl Backends for Start {
    fn latex(&self) -> String {
        let (start, end) = match self.stream.len() {
            0 => ("", ""),
            1 => (r"\(", r"\)"),
            other => (r"\[", r"\]")
        };
        return format!("{start}{}{end}", self.stream.iter().map(|element| element.latex()).collect::<Vec<String>>().join(r"\\ "));
    }
} impl Spawn for Start {fn summon(items: Vec<Item>) -> NonTerminal {return NonTerminal::Start(Self {
    stream: items.into_iter().map(|element| if let Item::NonTerminal(NonTerminal::Level1(level1)) = element {level1} else {panic!("{element:?}")}).collect()
})}}