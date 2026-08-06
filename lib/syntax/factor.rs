//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> CRATE
use crate::latex::LaTeX;

//> HEAD -> SUPER
use super::{
    expression::Expression,
    value::{
        Identifier,
        Value,
        Nest
    }
};


//^
//^ FACTOR
//^

//> FACTOR -> ENUM
#[enum_dispatch(LaTeX)]
pub enum Factor<'valid> {
    Raised(Raised<'valid>),
    Limit(Limit<'valid>)
}

//> FACTOR -> RAISED
pub struct Raised<'valid> {
    pub value: Value<'valid>,
    pub exponent: Option<Expression<'valid>>
} 

//> FACTOR -> LIMIT
pub struct Limit<'valid> {
    pub identifier: Identifier<'valid>,
    pub expression: Expression<'valid>,
    pub direction: Option<bool>,
    pub nest: Nest<'valid>,
    pub exponent: Option<Expression<'valid>>
} 