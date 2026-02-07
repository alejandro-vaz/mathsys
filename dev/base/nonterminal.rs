//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Map
};

//> HEAD -> LOCAL
use super::start::Start;
use super::level1::{Level1, Declaration, Definition, Annotation, Node, Equation, Use};
use super::level2::{Level2, Expression};
use super::level3::{Level3, Term};
use super::level4::{Level4, Factor, Limit};
use super::level5::{Level5, Absolute, Casts, Infinite, Nest, Rational, Tensor, Undefined, Variable, Whole};


//^
//^ NONTERMINAL
//^

//> NONTERMINAL -> TRAIT
#[derive(Debug, Clone)]
pub enum NonTerminal {
    Start(Start),
    Level1(Level1),
    Level2(Level2),
    Level3(Level3),
    Level4(Level4),
    Level5(Level5),
    Internal(Vec<NonTerminal>)
}

//> NONTERMINAL -> SYMBOLS
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum Object {
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
} impl Object {
    pub fn summon(&self, items: Vec<NonTerminal>) -> NonTerminal {return match self {
        Object::Start => Start::summon(items),
        Object::Level1 => if let NonTerminal::Level1(element) = &items[0] {items.into_iter().next().unwrap()} else {panic!("{items:?}")},
        Object::Level2 => if let NonTerminal::Level2(element) = &items[0] {items.into_iter().next().unwrap()} else {panic!("{items:?}")},
        Object::Level3 => if let NonTerminal::Level3(element) = &items[0] {items.into_iter().next().unwrap()} else {panic!("{items:?}")},
        Object::Level4 => if let NonTerminal::Level4(element) = &items[0] {items.into_iter().next().unwrap()} else {panic!("{items:?}")},
        Object::Level5 => if let NonTerminal::Level5(element) = &items[0] {items.into_iter().next().unwrap()} else {panic!("{items:?}")},
        Object::Declaration => Declaration::summon(items),
        Object::Definition => Definition::summon(items),
        Object::Annotation => Annotation::summon(items),
        Object::Node => Node::summon(items),
        Object::Equation => Equation::summon(items),
        Object::Use => Use::summon(items),
        Object::Expression => Expression::summon(items),
        Object::Term => Term::summon(items),
        Object::Factor => Factor::summon(items),
        Object::Limit => Limit::summon(items),
        Object::Infinite => Infinite::summon(items),
        Object::Variable => Variable::summon(items),
        Object::Nest => Nest::summon(items),
        Object::Tensor => Tensor::summon(items),
        Object::Whole => Whole::summon(items),
        Object::Absolute => Absolute::summon(items),
        Object::Undefined => Undefined::summon(items),
        Object::Rational => Rational::summon(items),
        Object::Casts => Casts::summon(items)
    }}
    pub fn score(&self) -> usize {return match self {
        Object::Declaration => 1,
        other => 0
    }}
}

//> NONTERMINAL -> SPAWN
pub trait Spawn {
    fn summon(items: Vec<NonTerminal>) -> NonTerminal;
}

//> NONTERMINAL -> BACKENDS
pub trait Backends {}