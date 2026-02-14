//^ 
//^ HEAD
//^

//> HEAD -> LOCAL
use super::super::solver::nonterminal::{Spawn, NonTerminal, Item};
use super::start::Start;
use super::level2::Level2;
use super::level5::{Variable, Level5};


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> NAMESPACE
#[derive(Debug, Clone)]
pub enum Level1 {
    Declaration(Declaration),
    Definition(Definition),
    Annotation(Annotation),
    Node(Node),
    Equation(Equation),
    Use(Use)
}

//> 1ºLEVEL -> DECLARATION
#[derive(Debug, Clone)]
pub struct Declaration {
    variable: Variable,
    value: Level2
} impl Spawn for Declaration {fn summon(items: Vec<Item>) -> NonTerminal {
    let mut variable = None;
    let mut value = None;
    for item in items {match item {
        Item::NonTerminal(NonTerminal::Level5(Level5::Variable(var))) => variable = Some(var),
        Item::NonTerminal(NonTerminal::Level2(level2)) => value = Some(level2),
        other => panic!()
    }}
    return NonTerminal::Level1(Level1::Declaration(Self {
        variable: variable.unwrap(),
        value: value.unwrap()
    }))
}}

//> 1ºLEVEL -> DEFINITION
#[derive(Debug, Clone)]
pub struct Definition {
    variable: Variable,
    value: Level2
} impl Spawn for Definition {fn summon(items: Vec<Item>) -> NonTerminal {
    let mut variable = None;
    let mut value = None;
    for item in items {match item {
        Item::NonTerminal(NonTerminal::Level5(Level5::Variable(var))) => variable = Some(var),
        Item::NonTerminal(NonTerminal::Level2(level2)) => value = Some(level2),
        other => panic!()
    }}
    return NonTerminal::Level1(Level1::Definition(Self {
        variable: variable.unwrap(),
        value: value.unwrap()
    }));
}}

//> 1ºLEVEL -> ANNOTATION
#[derive(Debug, Clone)]
pub struct Annotation {
    variables: Vec<Variable>
} impl Spawn for Annotation {fn summon(items: Vec<Item>) -> NonTerminal {
    let mut variables = Vec::new();
    for item in items {match item {
        Item::NonTerminal(NonTerminal::Level5(Level5::Variable(variable))) => variables.push(variable),
        other => panic!()
    }}
    return NonTerminal::Level1(Level1::Annotation(Self {
        variables: variables
    }));
}}

//> 1ºLEVEL -> NODE
#[derive(Debug, Clone)]
pub struct Node {
    value: Level2
} impl Spawn for Node {fn summon(items: Vec<Item>) -> NonTerminal {
    return NonTerminal::Level1(Level1::Node(Self {
        value: if let Item::NonTerminal(NonTerminal::Level2(level2)) = items.into_iter().next().unwrap() {level2} else {panic!()}
    }));
}}

//> 1ºLEVEL -> EQUATION
#[derive(Debug, Clone)]
pub struct Equation {
    left: Level2,
    right: Level2
} impl Spawn for Equation {fn summon(items: Vec<Item>) -> NonTerminal {
    let mut iterator = items.into_iter();
    return NonTerminal::Level1(Level1::Equation(Self {
        left: if let Item::NonTerminal(NonTerminal::Level2(level2)) = iterator.next().unwrap() {level2} else {panic!()},
        right: if let Item::NonTerminal(NonTerminal::Level2(level2)) = iterator.next().unwrap() {level2} else {panic!()},
    }));
}}

//> 1ºLEVEL -> USE
#[derive(Debug, Clone)]
pub struct Use {
    module: String,
    start: Option<Start>
} impl Spawn for Use {fn summon(items: Vec<Item>) -> NonTerminal {
    return NonTerminal::Level1(Level1::Use(Self {
        module: if let Item::Token(token) = items.into_iter().next().unwrap() {token.value.to_string()} else {panic!()},
        start: None
    }));
}}