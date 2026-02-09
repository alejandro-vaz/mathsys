//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    FastMap, FastSet, SmallVec
};

//> HEAD -> LOCAL
use super::issues::{Issue, SyntaxError};
use super::grammar::Symbol;
use super::nonterminal::{Partition, Object};
use super::recognizer::{Backpointer, MINPOINTERS};


//^
//^ BUILDER
//^

//> BUILDER -> STRUCT
pub struct Builder {
    memory: FastMap<Backpointer, (usize, Option<Partition>)>
} impl Builder {
    pub fn new() -> Self {return Builder {
        memory: FastMap::new()
    }}
    pub fn run(&mut self, pool: &FastMap<Backpointer, FastSet<SmallVec<[Backpointer; MINPOINTERS]>>>, length: u32) -> Result<(), Issue> {
        self.memory.clear();
        let backpointer = pool.iter().map(|item| item.0).find(|backpointer| if let Symbol::NonTerminal(Object::Start) = backpointer.symbol {true} else {false}).ok_or_else(SyntaxError)?;
        self.best(pool, backpointer);
        return Ok(());
    }
    fn best(&mut self, pool: &FastMap<Backpointer, FastSet<SmallVec<[Backpointer; MINPOINTERS]>>>, node: &Backpointer) -> (usize, Option<Partition>) {
        if let Some(result) = self.memory.get(node) {return result.clone()};
        if let Symbol::Kind(kind) = node.symbol {
            let result = (0, None);
            self.memory.insert(node.clone(), result.clone());
            return result;
        };
        let mut bcore = 0;
        let mut btree = None;
        if let Some(derivations) = pool.get(node) {
            for derivation in derivations {
                let mut score = match node.symbol {
                    Symbol::NonTerminal(object) => object.score(),
                    Symbol::Internal(code) => 0,
                    Symbol::Kind(kind) => unreachable!()
                };
                let mut children = Vec::new();
                for child in derivation {
                    let (ccore, ctree) = self.best(pool, child);
                    score += ccore;
                    if let Some(tree) = ctree {
                        match tree {
                            Partition::Internal(items) => children.extend(items),
                            Partition::NonTerminal(items) => children.push(items),
                        }
                    };
                }
                if score > bcore || btree.is_none() {
                    bcore = score;
                    btree = Some(match node.symbol {
                        Symbol::NonTerminal(object) => Partition::NonTerminal(object.summon(children)),
                        Symbol::Internal(code) => Partition::Internal(children),
                        Symbol::Kind(kind) => unreachable!()
                    });
                }
            }
        };
        let result = (bcore, btree);
        self.memory.insert(node.clone(), result.clone());
        return result;
    }
}