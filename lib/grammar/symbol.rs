//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> SUPER
use super::rule::Rule;


//^
//^ SYMBOL
//^

//> SYMBOL -> STRUCT
#[enum_dispatch]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Symbol {
    Rule,
    #[allow(nonstandard_style)]
    str(&'static str)
} 

//> SYMBOL -> FROM STR
impl From<&'static str> for Symbol {
    fn from(value: &'static str) -> Self {
        return if let Ok(rule) = value.try_into() {
            Self::Rule(rule)
        } else {
            Self::str(value)
        }
    }
}

//> SYMBOL -> FROM RULE
impl From<Rule> for Symbol {
    fn from(value: Rule) -> Self {return Symbol::Rule(value)}
}