//^
//^ HEAD
//^

//> HEAD -> MODULES
pub(super) mod level1;
pub(super) mod level2;
pub(super) mod level3;
pub(super) mod level4;
pub(super) mod level5;

//> HEAD -> LOCAL
use self::{
    level1::Level1,
    super::{
        issues::Issue,
        Transformers,
        Settings,
        solver::context::Context,
        backends::{
            Spawn, 
            Backends
        },
        solver::nonterminal::{
            Item, 
            NonTerminal
        }
    }
};


//^
//^ START
//^

//> START -> CLASS
#[derive(Debug, Clone)]
pub struct Start {
    pub(super) stream: Vec<Level1>
} impl Backends for Start {
    fn latex(&self) -> String {
        let (start, end) = match self.stream.len() {
            0 => ("", ""),
            1 => (r"\(", r"\)"),
            other => (r"\[", r"\]")
        };
        return format!("{start}{}{end}", self.stream.iter().map(|element| element.latex()).collect::<Vec<String>>().join(r"\\ "));
    }
} impl Spawn for Start {fn spawn(items: Vec<Item>, context: Option<&mut Context>) -> NonTerminal {
    return NonTerminal::Start(Self {
        stream: items.into_iter().map(|element| if let Item::NonTerminal(NonTerminal::Level1(level1)) = element {level1} else {panic!("{element:?}")}).collect()
    })
}} impl Start {pub(super) fn modules(&mut self, transformers: &Transformers, settings: &Settings) -> Result<(), Issue> {return Ok(for statement in &mut self.stream {if let Level1::Use(element) = statement {element.load(transformers, settings)?}})}}