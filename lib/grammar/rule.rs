//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> SUPER
use super::{
    object::Object,
    constants::TEMPORAL
};

//> HEAD -> STRUM
use strum::ParseError;


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

//> RULE -> TRY FROM STR
impl TryFrom<&str> for Rule {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        return if let Some(internal) = value.strip_prefix(TEMPORAL) {
            Ok(Rule::usize(internal.parse().unwrap()))
        } else {
            Ok(Rule::Object(value.parse::<Object>()?))
        }
    }
}