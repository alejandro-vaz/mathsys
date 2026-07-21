//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;

//> HEAD -> CRATE
use crate::syntax::{
    Start,
    level1::Level1,
    level2::Level2,
    level3::Level3,
    level4::Level4,
    level5::Level5
};


//^
//^ NONTERMINAL
//^

//> NONTERMINAL -> ENUM
#[enum_dispatch]
#[derive(Clone, EnumAsInner, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NonTerminal<'valid> {
    Start(Start<'valid>),
    Level1(Level1<'valid>),
    Level2(Level2<'valid>),
    Level3(Level3<'valid>),
    Level4(Level4<'valid>),
    Level5(Level5<'valid>)
}