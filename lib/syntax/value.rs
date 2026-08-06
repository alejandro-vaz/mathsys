//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> CRATE
use crate::latex::LaTeX;

//> HEAD -> SUPER
use super::expression::Expression;


//^
//^ VALUE
//^

//> VALUE -> ENUM
#[enum_dispatch(LaTeX)]
pub enum Value<'valid> {
    Infinite,
    Identifier(Identifier<'valid>),
    Nest(Nest<'valid>),
    Vector(Vector<'valid>),
    Number(Number<'valid>),
    Absolute(Absolute<'valid>),
    Undefined,
    Call(Call<'valid>)
}

//> VALUE -> INFINITE
pub struct Infinite;

//> VALUE -> IDENTIFIER
pub struct Identifier<'valid> {
    pub name: &'valid str
} 

//> VALUE -> NEST
pub struct Nest<'valid> {
    pub inside: Option<Expression<'valid>>
}

//> VALUE -> VECTOR
pub struct Vector<'valid> {
    pub expressions: Vec<Expression<'valid>>
} 

//> VALUE -> NUMBER
pub struct Number<'valid> {
    pub number: &'valid str
} 

//> VALUE -> ABSOLUTE
pub struct Absolute<'valid> {
    pub expression: Expression<'valid>
} 

//> VALUE -> UNDEFINED
pub struct Undefined; 

//> VALUE -> CALL
pub struct Call<'valid> {
    pub identifier: Identifier<'valid>,
    pub with: Vec<Expression<'valid>>
} 