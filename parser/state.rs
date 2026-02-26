//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Arena
};

//> HEAD -> LOCAL
use super::grammar::{
    Rule,
    Symbol,
    GRAMMAR,
    Part
};


//^
//^ STATE
//^

//> STATE -> STRUCT
#[derive(Clone, Hash, PartialEq, Eq)]
pub(super) struct State {
    pub(super) rule: Rule,
    pub(super) variant: &'static Vec<Symbol>,
    pub(super) slot: u8,
    pub(super) starting: usize
} impl State {
    #[inline(always)]
    pub(super) fn new<'arena>(arena: &'arena Arena<State>, rule: Rule, variant: &'static Vec<Symbol>, slot: u8, starting: usize) -> &'arena State {arena.alloc(Self {
        rule: rule,
        variant: variant,
        slot: slot,
        starting: starting
    })}
    #[inline(always)]
    pub(super) fn at(&self) -> Option<&'static Symbol> {self.variant.get(self.slot as usize)}
    #[inline(always)]
    pub(super) fn next<'all>(&self, arena: &'all Arena<State>) -> &'all State {return State::new(arena, self.rule, self.variant, self.slot + 1, self.starting)}
}

//> STATE -> BACKPOINTER
#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct Backpointer<'parsing> {
    pub(crate) symbol: Part<'parsing>,
    start: usize,
    end: usize
} impl<'parsing> Backpointer<'parsing> {
    #[inline(always)]
    pub(super) fn new<'land>(land: &'land Arena<Backpointer<'parsing>>, symbol: Part<'parsing>, start: usize, end: usize) -> &'land Backpointer<'parsing> {
        return land.alloc(Self {
            symbol: symbol,
            start: start,
            end: end
        })
    }
}