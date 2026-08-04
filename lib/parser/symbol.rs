//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> SUPER
use super::rule::Rule;

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ SYMBOL
//^

//> SYMBOL -> STRUCT
#[enum_dispatch]
#[derive(Debug, PartialEq, Eq, EnumAsInner, Hash)]
pub enum Symbol {
    Rule,
    #[allow(nonstandard_style)]
    str(&'static str)
} 

//> SYMBOL -> FROM STR
impl From<&'static str> for Symbol {
    fn from(value: &'static str) -> Self {return if let Ok(rule) = value.try_into() {
        Self::Rule(rule)
    } else {
        Self::str(value)
    }
}}