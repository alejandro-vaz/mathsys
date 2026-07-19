//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> SUPER
use super::object::Object;


//^
//^ RULE
//^

//> RULE -> STRUCT
#[enum_dispatch]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Rule {
    Object,
    #[allow(nonstandard_style)]
    usize
} 

//> RULE -> FROM STR
impl From<&str> for Rule {
    fn from(value: &str) -> Self {return if let Some(internal) = value.strip_prefix('$') {
        Rule::usize(internal.parse().unwrap())
    } else {
        Rule::Object(value.parse().unwrap())
    }}
}