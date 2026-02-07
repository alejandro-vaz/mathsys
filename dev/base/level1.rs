//^ 
//^ HEAD
//^

//> HEAD -> LOCAL
use super::nonterminal::{Spawn, NonTerminal};
use super::start::Start;


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
pub struct Declaration {} impl Spawn for Declaration {fn summon(items: Vec<NonTerminal>) -> NonTerminal {return NonTerminal::Level1(Level1::Declaration(Self {}))}}

//> 1ºLEVEL -> DEFINITION
#[derive(Debug, Clone)]
pub struct Definition {} impl Spawn for Definition {fn summon(items: Vec<NonTerminal>) -> NonTerminal {return NonTerminal::Level1(Level1::Definition(Self {}))}}

//> 1ºLEVEL -> ANNOTATION
#[derive(Debug, Clone)]
pub struct Annotation {} impl Spawn for Annotation {fn summon(items: Vec<NonTerminal>) -> NonTerminal {return NonTerminal::Level1(Level1::Annotation(Self {}))}}

//> 1ºLEVEL -> NODE
#[derive(Debug, Clone)]
pub struct Node {} impl Spawn for Node {fn summon(items: Vec<NonTerminal>) -> NonTerminal {return NonTerminal::Level1(Level1::Node(Self {}))}}

//> 1ºLEVEL -> EQUATION
#[derive(Debug, Clone)]
pub struct Equation {} impl Spawn for Equation {fn summon(items: Vec<NonTerminal>) -> NonTerminal {return NonTerminal::Level1(Level1::Equation(Self {}))}}

//> 1ºLEVEL -> USE
#[derive(Debug, Clone)]
pub struct Use {
    start: Option<Start>
} impl Spawn for Use {fn summon(items: Vec<NonTerminal>) -> NonTerminal {return NonTerminal::Level1(Level1::Use(Self {
    start: None
}))}}