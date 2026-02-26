//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    dispatch
};

//> HEAD -> LOCAL
use super::{
    super::{
        tokenizer::token::BindedToken,
        syntax::{
            Start,
            level1::Level1,
            level2::Level2,
            level3::Level3,
            level4::Level4,
            level5::Level5,
        }
    }
};


//^
//^ NONTERMINAL
//^

//> NONTERMINAL -> TRAIT
#[dispatch(Backends, Contextualize)]
#[derive(Debug, Clone)]
pub(crate) enum NonTerminal {
    Start,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5
}

//> NONTERMINAL -> ITEM
#[derive(Clone, Debug)]
pub(crate) enum Item<'resolving> {
    NonTerminal(NonTerminal),
    Token(BindedToken<'resolving>)
}

//> NONTERMINAL -> PARTITION
#[derive(Clone)]
pub(super) enum Partition<'resolving> {
    NonTerminal(NonTerminal),
    Internal(Vec<Item<'resolving>>),
    Token(BindedToken<'resolving>)
}