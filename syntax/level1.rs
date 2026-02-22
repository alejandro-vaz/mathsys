//^ 
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    dispatch
};

//> HEAD -> LOCAL
use super::{
    Start,
    level2::Level2,
    level5::{
        Variable, 
        Level5
    },
    super::{
        Transformers,
        issues::Issue,
        Settings,
        solver::context::Context,
        backends::{
            Backends, 
            Spawn
        },
        solver::nonterminal::{
            NonTerminal, 
            Item
        },
        entry::File
    }
};


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> NAMESPACE
#[dispatch(Backends)]
#[derive(Debug, Clone)]
pub(crate) enum Level1 {
    Definition,
    Function,
    Node,
    Equation,
    Use
}

//> 1ºLEVEL -> DEFINITION
#[derive(Debug, Clone)]
pub(crate) struct Definition {
    pub(crate) variable: Variable,
    pub(crate) value: Level2
} impl Backends for Definition {
    fn latex(&self) -> String {return format!("{}:={}", self.variable.latex(), self.value.latex())}
} impl Spawn for Definition {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
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

//> 1ºLEVEL -> FUNCTION
#[derive(Debug, Clone)]
pub(crate) struct Function {
    pub(crate) variable: Variable,
    pub(crate) arguments: Vec<Variable>,
    pub(crate) value: Level2
} impl Backends for Function {
    fn latex(&self) -> String {return format!("{}({}):={}", self.variable.latex(), self.arguments.iter().map(|argument| argument.latex()).collect::<Vec<String>>().join(","), self.value.latex())}
} impl Spawn for Function {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    let mut iterator = items.into_iter();
    let Some(Item::NonTerminal(NonTerminal::Level5(Level5::Variable(variable)))) = iterator.next() else {panic!()};
    let Some(Item::NonTerminal(NonTerminal::Level2(value))) = iterator.next_back() else {panic!()};
    let arguments = iterator.map(|each| if let Item::NonTerminal(NonTerminal::Level5(Level5::Variable(argument))) = each {argument} else {panic!()}).collect::<Vec<Variable>>();
    if let Some(context) = context {
        context.registerFn(&variable);
    }
    return NonTerminal::Level1(Level1::Function(Self {
        variable: variable,
        arguments: arguments,
        value: value
    }))
}}

//> 1ºLEVEL -> NODE
#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub(crate) value: Level2
} impl Backends for Node {
    fn latex(&self) -> String {return self.value.latex()}
} impl Spawn for Node {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    return NonTerminal::Level1(Level1::Node(Self {
        value: if let Item::NonTerminal(NonTerminal::Level2(level2)) = items.into_iter().next().unwrap() {level2} else {panic!()}
    }));
}}

//> 1ºLEVEL -> EQUATION
#[derive(Debug, Clone)]
pub(crate) struct Equation {
    pub(crate) left: Level2,
    pub(crate) right: Level2
} impl Backends for Equation {
    fn latex(&self) -> String {return format!("{}={}", self.left.latex(), self.right.latex())}
} impl Spawn for Equation {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    let mut iterator = items.into_iter();
    return NonTerminal::Level1(Level1::Equation(Self {
        left: if let Item::NonTerminal(NonTerminal::Level2(level2)) = iterator.next().unwrap() {level2} else {panic!()},
        right: if let Item::NonTerminal(NonTerminal::Level2(level2)) = iterator.next().unwrap() {level2} else {panic!()},
    }));
}}

//> 1ºLEVEL -> USE
#[derive(Debug, Clone)]
pub(crate) struct Use {
    pub(crate) module: String,
    pub(crate) start: Option<Start>
} impl Backends for Use {
    fn latex(&self) -> String {
        let (start, end) = match &self.start {
            None => (r"\color{brown}", r"\color{black}"),
            Some(thing) => ("", "")
        };
        return format!(r"{start}\text{{use {}}}{end}", self.module);
    }
} impl Spawn for Use {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    return NonTerminal::Level1(Level1::Use(Self {
        module: if let Item::Token(token) = items.into_iter().next().unwrap() {token.value.trim_matches('\"').to_string()} else {panic!()},
        start: None
    }));
}} impl Use {pub fn load(&mut self, transformers: &Transformers, settings: &Settings) -> Result<(), Issue> {return Ok(if let Ok(content) = File(self.module.clone().into()).read() {
    let tokens = transformers.tokenizer.run(&content, settings)?;
    let pool = transformers.parser.run(&tokens, settings);
    let mut start = transformers.solver.run(&pool)?;
    start.modules(transformers, settings)?;
    self.start = Some(start);
})}}