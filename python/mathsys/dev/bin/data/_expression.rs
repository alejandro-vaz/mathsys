//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ EXPRESSION
//^

//> EXPRESSION -> STRUCT
#[derive(Clone)]
pub struct _Expression {
    pub signs: Box<[u8]>,
    pub terms: Box<[u32]>
}

//> EXPRESSION -> EVALUATE
impl _Expression {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    for &term in &self.terms {context.process(term, memory)}
    self.space("Summing up all terms", id);
    let mut current = Object::Nexists(crate::Nexists {});
    for (index, term) in self.terms.iter().enumerate() {
        let next = context.read(*term);
        let value = if self.signs[index] % 2 == 0 {next.negate()} else {next};
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