//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> CRATE
use crate::{
    tokenizer::token::Token,
    grammar::rule::Rule
};


//^
//^ PARSED
//^

//> PARSED -> ENUM
#[enum_dispatch]
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Parsed<'valid> {
    Rule,
    Token(Token<'valid>)
} 

//> PARSED -> FROM RULE
impl<'valid> From<Rule> for Parsed<'valid> {
    fn from(value: Rule) -> Self {return Parsed::Rule(value)}
}