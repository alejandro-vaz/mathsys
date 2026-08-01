//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    rule::Rule,
    symbol::Symbol,
    constants::DERIVATION_LENGTH,
    grammar::GRAMMAR
};

//> HEAD -> LIBUTILS
use libutils::stack_array::Array;

//> HEAD -> CRATE
use crate::tokenizer::token::Token;


//^
//^ PRODUCTION
//^

//> PRODUCTION -> STRUCT
#[derive(PartialEq, Eq, Hash)]
pub struct Production {
    pub rule: &'static Rule,
    pub derivation: &'static Array<Symbol, DERIVATION_LENGTH>,
    pub at: usize,
    pub lookahead: &'static str
}

//> PRODUCTION -> DEFAULT
impl Default for Production {
    fn default() -> Self {return Production {
        rule: &const {Rule::default()},
        derivation: &GRAMMAR[&const {Rule::default()}][0],
        at: 0,
        lookahead: Token::EndOfFile.as_ref()
    }}
}