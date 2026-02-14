//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::super::{
    runtime::traits::Spawn,
    tokenizer::tokenizer::BindedToken,
    syntax::{
        start::Start,
        level1::{Level1, Declaration, Definition, Node, Equation, Use},
        level2::{Level2, Expression},
        level3::{Level3, Term},
        level4::{Level4, Factor, Limit},
        level5::{Level5, Absolute, Infinite, Nest, Rational, Tensor, Undefined, Variable, Whole}
    }
};


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
    Level5(Level5)
}

//> NONTERMINAL -> OBJECT
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
    Rational
} impl Object {
    pub fn summon<'parsing>(&self, items: Vec<Item>) -> NonTerminal {return match self {
        Object::Start => Start::summon(items),
        Object::Level1 => if let Item::NonTerminal(NonTerminal::Level1(element)) = &items[0] {items.into_iter().next().unwrap().getnt()} else {panic!("{items:?}")},
        Object::Level2 => if let Item::NonTerminal(NonTerminal::Level2(element)) = &items[0] {items.into_iter().next().unwrap().getnt()} else {panic!("{items:?}")},
        Object::Level3 => if let Item::NonTerminal(NonTerminal::Level3(element)) = &items[0] {items.into_iter().next().unwrap().getnt()} else {panic!("{items:?}")},
        Object::Level4 => if let Item::NonTerminal(NonTerminal::Level4(element)) = &items[0] {items.into_iter().next().unwrap().getnt()} else {panic!("{items:?}")},
        Object::Level5 => if let Item::NonTerminal(NonTerminal::Level5(element)) = &items[0] {items.into_iter().next().unwrap().getnt()} else {panic!("{items:?}")},
        Object::Declaration => Declaration::summon(items),
        Object::Definition => Definition::summon(items),
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
        Object::Rational => Rational::summon(items)
    }}
    pub fn score(&self) -> usize {return match self {
        Object::Declaration => 1,
        other => 0
    }}
}


//^
//^ TEMPORAL
//^

//> TEMPORAL -> ITEM
#[derive(Clone, Debug)]
pub enum Item<'resolving> {
    NonTerminal(NonTerminal),
    Token(BindedToken<'resolving>)
} impl<'resolving> Item<'resolving> {fn getnt(self) -> NonTerminal {match self {
    Item::NonTerminal(item) => return item,
    other => panic!("{other:?}")
}}}

//> TEMPORAL -> PARTITION
#[derive(Clone)]
pub enum Partition<'resolving> {
    NonTerminal(NonTerminal),
    Internal(Vec<Item<'resolving>>),
    Token(BindedToken<'resolving>)
}