//^
//^ HEAD
//^

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;

//> HEAD -> CRATE
use crate::grammar::{
    symbol::Symbol,
    rule::Rule,
    constants::DERIVATION_LENGTH
};


//^
//^ STATE
//^

//> STATE -> STRUCT
#[derive(PartialEq, Eq, Hash)]
pub struct State {
    pub rule: Rule,
    pub variant: &'static Array<Symbol, DERIVATION_LENGTH>,
    pub slot: usize,
    pub starting: usize
}