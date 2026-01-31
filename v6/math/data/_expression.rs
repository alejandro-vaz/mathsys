//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Sign, Pointer, Runtime, Class, Object, Nexists, Tip
};


//^
//^ EXPRESSION
//^

//> EXPRESSION -> STRUCT
#[derive(Clone, Debug)]
pub struct _Expression {
    pub signs: Vec<Vec<Sign>>,
    pub terms: Vec<Pointer>
}

//> EXPRESSION -> EVALUATE
impl _Expression {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let terms: Vec<Object> = self.terms.iter().map(|term| runtime.get(*term, memory)).collect();
    //~ EVALUATE -> OPERATIONS
    self.section("Summing up all terms", id);
    let mut current = Nexists::new();
    for index in 0..terms.len() {
        let next = &terms[index];
        let mut sign = Sign::Positive;
        self.signs[index].iter().for_each(|following| sign = (sign == *following).into());
        let value = if sign.into() {next} else {&next.negate()};
        current = current.summation(value);
    }
    return current;
}}