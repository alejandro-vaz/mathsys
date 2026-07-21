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
    Start,
    level5::Variable
};


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> ENUM
#[enum_dispatch(LaTeX)]
#[derive(Clone, EnumAsInner, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level1<'valid> {
    Definition(Definition<'valid>),
    Function(Function<'valid>),
    Node(Node<'valid>),
    Equation(Equation<'valid>),
    Use(Use<'valid>)
}

//> 1ºLEVEL -> DEFINITION
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Definition<'valid> {
    pub variable: Variable<'valid>,
    pub value: Level2<'valid>
} 

//> 1ºLEVEL -> FUNCTION
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Function<'valid> {
    pub variable: Variable<'valid>,
    pub arguments: Vec<Variable<'valid>>,
    pub expression: Level2<'valid>
} 

//> 1ºLEVEL -> NODE
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<'valid> {
    pub value: Level2<'valid>
} 

//> 1ºLEVEL -> EQUATION
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Equation<'valid> {
    pub left: Level2<'valid>,
    pub right: Level2<'valid>
} 

//> 1ºLEVEL -> USE
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Use<'valid> {
    pub _module: &'valid str,
    pub _start: Start<'valid>
} 