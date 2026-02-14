//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    dispatch
};

//> HEAD -> LOCAL
use super::super::{
    solver::nonterminal::{Item, NonTerminal},
    syntax::{
        level1::{Declaration, Definition, Level1, Node, Equation, Use},
        level2::{Level2, Expression},
        level3::{Level3, Term},
        level4::{Limit, Level4, Factor},
        level5::{Absolute, Level5, Infinite, Variable, Nest, Tensor, Whole, Undefined, Rational}
    }
};


//^
//^ TRAITS
//^

//> TRAITS -> SPAWN
pub trait Spawn: Backends {
    fn summon(items: Vec<Item>) -> NonTerminal;
}

//> BACKENDS -> TRAIT
#[dispatch]
pub trait Backends {
    fn latex(&self) -> String;
}