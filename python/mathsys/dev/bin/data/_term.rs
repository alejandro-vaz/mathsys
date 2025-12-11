//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ TERM
//^

//> TERM -> STRUCT
#[derive(Clone)]
pub struct _Term {
    pub numerator: Box<[u32]>,
    pub denominator: Box<[u32]>
}

//> TERM -> EVALUATE
impl _Term {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    for &factor in &self.numerator {context.process(factor, memory)}
    for &factor in &self.denominator {context.process(factor, memory)}
    self.space("Calculating term", id);
    let mut numerator = Object::Nexists(crate::Nexists {});
    for &factor in &self.numerator {
        let next = context.read(factor);
        numerator = numerator.multiplication(&next);
    }
    let mut denominator = Object::Nexists(crate::Nexists {});
    for &factor in &self.denominator {
        let next = context.read(factor);
        denominator = denominator.multiplication(&next);
    }
    denominator = denominator.invert();
    return numerator.multiplication(&denominator)
}}

//> TERM -> REPRESENTATION
impl crate::Display for _Term {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Term {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> TERM -> COMMON
impl Tip for _Term {} impl _Term {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Term")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "numerator = {:?}, denominator = {:?}",
        self.numerator, self.denominator
    )}
}