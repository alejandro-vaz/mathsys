//^
//^ HEAD
//^

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
    pub(super) variant: u8,
    pub(super) slot: u8,
    pub(super) starting: u32
} impl State {
    #[inline(always)]
    pub(super) fn new(rule: Rule, variant: u8, slot: u8, starting: u32) -> Self {return Self {
        rule: rule,
        variant: variant,
        slot: slot,
        starting: starting
    }}
    #[inline(always)]
    pub(super) fn at(&self) -> Option<Symbol> {
        GRAMMAR[&self.rule].get(self.variant as usize)?.get(self.slot as usize).cloned()
    }
    #[inline(always)]
    pub(super) fn next(&self) -> State {return State {
        rule: self.rule.clone(),
        variant: self.variant,
        slot: self.slot + 1,
        starting: self.starting
    }}
}

//> STATE -> BACKPOINTER
#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct Backpointer<'parsing> {
    pub(crate) symbol: Part<'parsing>,
    start: u32,
    end: u32
} impl<'parsing> Backpointer<'parsing> {
    #[inline(always)]
    pub(super) fn new(symbol: Part<'parsing>, start: u32, end: u32) -> Self {return Self {
        symbol: symbol,
        start: start,
        end: end
    }}
}