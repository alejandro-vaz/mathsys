//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer, Runtime, Class, Object, Nexists, Tip
};


//^
//^ TERM
//^

//> TERM -> STRUCT
#[derive(Clone, Debug)]
pub struct _Term {
    pub numerator: Vec<Pointer>,
    pub denominator: Vec<Pointer>
}

//> TERM -> EVALUATE
impl _Term {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let mut numerator = Vec::with_capacity(self.numerator.len());
    for &factor in &self.numerator {numerator.push(runtime.get(factor, memory))};
    let mut denominator = Vec::with_capacity(self.denominator.len());
    for &factor in &self.denominator {denominator.push(runtime.get(factor, memory))};
    //~ EVALUATE -> OPERATIONS
    self.section("Calculating numerator", id);
    let mut up = Nexists::new();
    for factor in numerator {up = up.multiplication(&factor)}
    self.section("Calculating denominator", id);
    let mut down = Nexists::new();
    for factor in denominator {down = down.multiplication(&factor)};
    self.section("Computing fraction", id);
    let inverse = down.invert();
    return up.multiplication(&inverse);
}}