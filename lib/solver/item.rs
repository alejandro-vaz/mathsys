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
#[derive(Clone, EnumAsInner, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item<'valid> {
    NonTerminal(NonTerminal<'valid>),
    Token(Token<'valid>)
}