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
use super::level3::Level3;


//^
//^ 2ºLEVEL
//^

//> 2ºLEVEL -> ENUM
#[enum_dispatch(LaTeX)]
#[derive(Clone, EnumAsInner, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level2<'valid> {
    Expression(Expression<'valid>)
}

//> 2ºLEVEL -> EXPRESSION
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Expression<'valid> {
    pub terms: Vec<(Vec<bool>, Level3<'valid>)>
} 