//^
//^ HEAD
//^

//> HEAD -> MODULES
pub(super) mod latex;

//> HEAD -> PRELUDE
use crate::prelude::{
    dispatch
};

//> HEAD -> LOCAL
use self::super::{
    solver::{
        nonterminal::{
            Item, 
            NonTerminal
        },
        context::Context
    },
    syntax::{
        Start,
        level1::{
            Definition, 
            Function, 
            Level1, 
            Node, 
            Equation, 
            Use
        },
        level2::{
            Level2, 
            Expression
        },
        level3::{
            Level3, 
            Term
        },
        level4::{
            Limit, 
            Level4, 
            Factor
        },
        level5::{
            Absolute, 
            Level5, 
            Infinite, 
            Variable, 
            Nest, 
            Tensor, 
            Whole, 
            Undefined, 
            Rational,
            Call
        }
    }
};


//^
//^ TRAITS
//^

//> TRAITS -> SPAWN
pub(super) trait Spawn: Backends {
    fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal;
}

//> BACKENDS -> TRAIT
#[dispatch]
pub(super) trait Backends {
    fn latex(&self) -> String;
}