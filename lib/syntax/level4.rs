//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;

//> HEAD -> CRATE
use crate::latex::LaTeX;

//> HEAD -> SUPER
use super::{
    level2::Level2,
    level5::{
        Variable,
        Level5,
        Nest
    }
};


//^
//^ 4ºLEVEL
//^

//> 4ºLEVEL -> ENUM
#[enum_dispatch(LaTeX)]
#[derive(Clone, EnumAsInner, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level4<'valid> {
    Factor(Factor<'valid>),
    Limit(Limit<'valid>)
}

//> 4ºLEVEL -> FACTOR
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Factor<'valid> {
    pub value: Level5<'valid>,
    pub exponent: Option<Level2<'valid>>
} 

//> 4ºLEVEL -> LIMIT
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Limit<'valid> {
    pub variable: Variable<'valid>,
    pub approach: Level2<'valid>,
    pub direction: Option<bool>,
    pub nest: Nest<'valid>,
    pub exponent: Option<Level2<'valid>>
} 