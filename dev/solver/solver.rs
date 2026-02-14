//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    FastMap, FastSet, SmallVec
};

//> HEAD -> LOCAL
use super::super::issues::Issue;
use super::super::tokenizer::tokenizer::{ORDER, Responsibility};
use super::nonterminal::{Partition, Object, NonTerminal, Item};
use super::super::parser::parser::{Backpointer, MINPOINTERS, Part};
use super::super::syntax::start::Start;


//^
//^ SOLVER
//^

//> SOLVER -> STRUCT
pub struct Solver {} impl Solver {
    pub fn new() -> Self {return Solver {}}
    pub fn run<'resolving>(&self, pool: &FastMap<Backpointer<'resolving>, FastSet<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>) -> Result<Start, Issue> {
        let mut memory = FastMap::new();
        let Partition::NonTerminal(NonTerminal::Start(start)) = self.best(pool, pool.iter().map(|item| item.0).find(|backpointer| if let Part::NonTerminal(Object::Start) = backpointer.symbol {true} else {false}).ok_or(Issue::SyntaxError)?, &mut memory).1.ok_or(Issue::SyntaxError)? else {return Err(Issue::SyntaxError)};
        return Ok(start);
    }
    fn best<'resolving, 'active>(&self, pool: &'active FastMap<Backpointer<'resolving>, FastSet<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>, node: &'active Backpointer<'resolving>, memory: &mut FastMap<Backpointer<'resolving>, (usize, Option<Partition<'resolving>>)>) -> (usize, Option<Partition<'resolving>>) {
        if let Some(result) = memory.get(node) {return result.clone()};
        if let Part::Token(token) = node.symbol.clone() {
            let result = (0, Some(Partition::Token(token)));
            memory.insert(node.clone(), result.clone());
            return result;
        };
        let mut bcore = 0;
        let mut btree = None;
        if let Some(derivations) = pool.get(node) {
            for derivation in derivations {
                let mut score = match node.symbol.clone() {
                    Part::NonTerminal(object) => object.score(),
                    Part::Internal(code) => 0,
                    other => unreachable!()
                };
                let mut children = Vec::new();
                for child in derivation {
                    let (ccore, ctree) = self.best(pool, child, memory);
                    score += ccore;
                    if let Some(tree) = ctree {match tree {
                        Partition::Internal(items) => children.extend(items),
                        Partition::NonTerminal(item) => children.push(Item::NonTerminal(item)),
                        Partition::Token(token) => if let Responsibility::Total = ORDER.get(&token.kind).unwrap().1 {children.push(Item::Token(token))},
                    }};
                }
                if score > bcore || btree.is_none() {
                    bcore = score;
                    btree = Some(match node.symbol.clone() {
                        Part::NonTerminal(object) => Partition::NonTerminal(object.summon(children)),
                        Part::Internal(code) => Partition::Internal(children),
                        other => unreachable!()
                    });
                }
            }
        };
        let result = (bcore, btree);
        memory.insert(node.clone(), result.clone());
        return result;
    }
}