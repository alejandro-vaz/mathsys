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
use super::level2::Level2;


//^
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> ENUM
#[enum_dispatch(LaTeX)]
#[derive(Clone, EnumAsInner, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level5<'valid> {
    Infinite,
    Variable(Variable<'valid>),
    Nest(Nest<'valid>),
    Vector(Vector<'valid>),
    Number(Number<'valid>),
    Absolute(Absolute<'valid>),
    Undefined,
    Call(Call<'valid>)
}

//> 5ºLEVEL -> INFINITE
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Infinite;

//> 5ºLEVEL -> VARIABLE
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Variable<'valid> {
    pub name: &'valid str
} 

//> 5ºLEVEL -> NEST
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Nest<'valid> {
    pub value: Option<Level2<'valid>>
} 

//> 5ºLEVEL -> VECTOR
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector<'valid> {
    pub values: Vec<Level2<'valid>>
} 

//> 5ºLEVEL -> NUMBER
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number<'valid> {
    pub number: &'valid str
} 

//> 5ºLEVEL -> ABSOLUTE
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Absolute<'valid> {
    pub value: Level2<'valid>
} 

//> 5ºLEVEL -> UNDEFINED
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Undefined; 

//> 5ºLEVEL -> CALL
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Call<'valid> {
    pub to: Variable<'valid>,
    pub with: Vec<Level2<'valid>>
} 