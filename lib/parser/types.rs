//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::{
    extensor::types::{
        Rule,
        Symbol,
        LENGTH,
        Object
    },
    tokenizer::token::Token
};

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;


//^
//^ TYPES
//^

//> TYPES -> PARSED
#[enum_dispatch]
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Parsed<'valid> {
    Object,
    #[allow(nonstandard_style)]
    usize,
    Token(Token<'valid>)
} impl<'valid> From<Rule> for Parsed<'valid> {
    fn from(value: Rule) -> Self {return match value {
        Rule::Object(object) => Parsed::Object(object),
        Rule::usize(internal) => Parsed::usize(internal)
    }}
}

//> TYPES -> STATE
#[derive(PartialEq, Eq, Hash)]
pub struct State<'grammar> {
    pub rule: Rule,
    pub variant: &'grammar Array<Symbol, LENGTH>,
    pub slot: usize,
    pub starting: usize
}

//> TYPES -> POINTER
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Pointer<'valid> {
    pub parsed: Parsed<'valid>,
    pub start: usize,
    pub end: usize
}