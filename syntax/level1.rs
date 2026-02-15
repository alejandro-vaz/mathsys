//^ 
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    dispatch
};

//> HEAD -> LOCAL
use super::{
    start::Start,
    level2::Level2,
    level5::{Variable, Level5},
    super::{
        issues::Issue,
        Settings,
        tokenizer::tokenizer::Tokenizer,
        parser::parser::Parser,
        solver::solver::Solver,
        backends::traits::{Backends, Spawn},
        solver::nonterminal::{NonTerminal, Item},
        entry::File
    }
};


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> NAMESPACE
#[dispatch(Backends)]
#[derive(Debug, Clone)]
pub enum Level1 {
    Declaration,
    Definition,
    Node,
    Equation,
    Use
}

//> 1ºLEVEL -> DECLARATION
#[derive(Debug, Clone)]
pub struct Declaration {
    variable: Variable,
    value: Level2
} impl Backends for Declaration {
    fn latex(&self) -> String {return format!("{}={}", self.variable.latex(), self.value.latex())}
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
} impl Backends for Definition {
    fn latex(&self) -> String {return format!(r"{}\equiv {}", self.variable.latex(), self.value.latex())}
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

//> 1ºLEVEL -> NODE
#[derive(Debug, Clone)]
pub struct Node {
    value: Level2
} impl Backends for Node {
    fn latex(&self) -> String {return self.value.latex()}
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
} impl Backends for Equation {
    fn latex(&self) -> String {return format!("{}={}", self.left.latex(), self.right.latex())}
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
} impl Backends for Use {
    fn latex(&self) -> String {
        let (start, end) = match &self.start {
            None => (r"\color{brown}", r"\color{black}"),
            Some(thing) => ("", "")
        };
        return format!(r"{start}\text{{use {}}}{end}", self.module);
    }
} impl Spawn for Use {fn summon(items: Vec<Item>) -> NonTerminal {
    return NonTerminal::Level1(Level1::Use(Self {
        module: if let Item::Token(token) = items.into_iter().next().unwrap() {token.value.trim_matches('\"').to_string()} else {panic!()},
        start: None
    }));
}} impl Use {pub fn load(&mut self, tokenizer: &mut Tokenizer, parser: &mut Parser, solver: &mut Solver, settings: &Settings) -> Result<(), Issue> {
    let Ok(content) = File {
        name: self.module.clone().into()
    }.read() else {return Ok(())};
    let tokens = tokenizer.run(&content, settings)?;
    let pool = parser.run(&tokens, settings);
    let mut start = solver.run(&pool)?;
    start.modules(tokenizer, parser, solver, settings)?;
    self.start = Some(start);
    return Ok(());
}}