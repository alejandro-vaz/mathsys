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
pub enum NonTerminals {
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
} impl NonTerminals {pub fn byname(name: &str) -> NonTerminals {return match name {
    "Start" => NonTerminals::Start,
    "Level1" => NonTerminals::Level1,
    "Level2" => NonTerminals::Level2,
    "Level3" => NonTerminals::Level3,
    "Level4" => NonTerminals::Level4,
    "Level5" => NonTerminals::Level5,
    "Declaration" => NonTerminals::Declaration,
    "Definition" => NonTerminals::Definition,
    "Annotation" => NonTerminals::Annotation,
    "Node" => NonTerminals::Node,
    "Equation" => NonTerminals::Equation,
    "Use" => NonTerminals::Use,
    "Expression" => NonTerminals::Expression,
    "Term" => NonTerminals::Term,
    "Factor" => NonTerminals::Factor,
    "Limit" => NonTerminals::Limit,
    "Infinite" => NonTerminals::Infinite,
    "Variable" => NonTerminals::Variable,
    "Nest" => NonTerminals::Nest,
    "Tensor" => NonTerminals::Tensor,
    "Whole" => NonTerminals::Whole,
    "Absolute" => NonTerminals::Absolute,
    "Undefined" => NonTerminals::Undefined,
    "Rational" => NonTerminals::Rational,
    "Casts" => NonTerminals::Casts,
    other => panic!("{other}")
}}}