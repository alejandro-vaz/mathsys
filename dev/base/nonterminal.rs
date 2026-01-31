//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::start::Start;
use super::level1::Level1;
use super::level2::Level2;
use super::level3::Level3;
use super::level4::Level4;
use super::level5::Level5;


//^
//^ NONTERMINAL
//^

//> NONTERMINAL -> TRAIT
pub enum NonTerminal {
    Start(Start),
    Level1(Level1),
    Level2(Level2),
    Level3(Level3),
    Level4(Level4),
    Level5(Level5)
}

//> NONTERMINAL -> SYMBOLS
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum Objects {
    Start,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Declaration,
    Definition,
    Annotation,
    Node,
    Equation,
    Use,
    Expression,
    Term,
    Factor,
    Limit,
    Infinite,
    Variable,
    Nest,
    Tensor,
    Whole,
    Absolute,
    Undefined,
    Rational,
    Casts
}