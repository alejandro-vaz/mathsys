//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::tokenizer::token::Token;

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> SUPER
use super::nonterminal::NonTerminal;

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ ITEM
//^

//> ITEM -> ENUM
#[enum_dispatch]
#[derive(EnumAsInner, Debug)]
pub enum Item<'parsing, 'valid> {
    NonTerminal(NonTerminal<'valid>),
    Token(&'parsing Token<'valid>)
}