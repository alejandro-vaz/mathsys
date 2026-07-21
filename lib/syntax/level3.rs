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
use super::level4::Level4;


//^
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> ENUM
#[enum_dispatch(LaTeX)]
#[derive(Clone, EnumAsInner, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level3<'valid> {
    Term(Term<'valid>)
}

//> 3ºLEVEL -> TERM
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Term<'valid> {
    pub numerator: Vec<Level4<'valid>>,
    pub denominator: Vec<Level4<'valid>>
} 