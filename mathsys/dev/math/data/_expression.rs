//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Sign,
    Pointer,
    Runtime,
    Class,
    Object,
    Nexists,
    fmt,
    Tip
};


//^
//^ EXPRESSION
//^

//> EXPRESSION -> STRUCT
#[derive(Clone)]
pub struct _Expression {
    pub signs: Vec<Option<Sign>>,
    pub terms: Vec<Pointer>
}

//> EXPRESSION -> EVALUATE
impl _Expression {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let mut terms = Vec::with_capacity(self.terms.len());
    for &index in &self.terms {terms.push(runtime.get(index, memory))}
    //~ EVALUATE -> OPERATIONS
    self.section("Summing up all terms", id);
    let mut current = Nexists::new();
    for index in 0..terms.len() {
        let next = &terms[index];
        let value = if let Some(item) = self.signs[index] {if item.into() {&next.negate()} else {next}} else {next};
        current = current.summation(value);
    }
    return current;
}}

//> EXPRESSION -> REPRESENTATION
impl fmt::Display for _Expression {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Expression {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> EXPRESSION -> COMMON
impl Tip for _Expression {} impl _Expression {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Expression")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "signs = {:?}, terms = {:?}",
        self.signs, self.terms
    )}
}