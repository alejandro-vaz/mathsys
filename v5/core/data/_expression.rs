//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Runtime;
use crate::tip::Tip;
use crate::group::Group;
use crate::stdout::{crash, Code};


//^
//^ EXPRESSION
//^

//> EXPRESSION -> STRUCT
#[derive(Clone)]
pub struct _Expression {
    pub signs: Vec<u8>,
    pub terms: Vec<u32>
}

//> EXPRESSION -> EVALUATE
impl _Expression {pub fn evaluate(&self, runtime: &mut Runtime, id: u32, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let mut terms = Vec::with_capacity(self.terms.len());
    for &index in &self.terms {terms.push(runtime.get(index, memory))}
    //~ EVALUATE -> OPERATIONS
    self.space("Summing up all terms", id);
    let mut current = Object::Nexists(crate::Nexists::new());
    for index in 0..terms.len() {
        let next = &terms[index];
        let value = if self.signs[index] % 2 == 0 {next.negate()} else {next.clone()};
        current = current.summation(&value);
    }
    return current;
}}

//> EXPRESSION -> REPRESENTATION
impl crate::Display for _Expression {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Expression {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> EXPRESSION -> COMMON
impl Tip for _Expression {} impl _Expression {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Expression")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "signs = {:?}, terms = {:?}",
        self.signs, self.terms
    )}
}