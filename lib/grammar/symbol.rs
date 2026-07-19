//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> CRATE
use crate::tokenizer::kind::Kind;

//> HEAD -> SUPER
use super::{
    object::Object,
    rule::Rule
};


//^
//^ SYMBOL
//^

//> SYMBOL -> STRUCT
#[enum_dispatch]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Symbol {
    Object,
    #[allow(nonstandard_style)]
    usize,
    Kind
} 

//> SYMBOL -> FROM STR
impl From<&str> for Symbol {
    fn from(value: &str) -> Self {return if let Some(internal) = value.strip_prefix('$') {
        Symbol::usize(internal.parse().unwrap())
    } else if let Ok(kind) = value.parse::<Kind>() {
        Symbol::Kind(kind)
    } else {
        Symbol::Object(value.parse().unwrap())
    }}
} 

//> SYMBOL -> FROM RULE
impl From<Rule> for Symbol {
    fn from(value: Rule) -> Self {return match value {
        Rule::Object(object) => Symbol::Object(object),
        Rule::usize(internal) => Symbol::usize(internal)
    }}
}