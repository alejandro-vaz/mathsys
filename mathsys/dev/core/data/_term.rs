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
//^ TERM
//^

//> TERM -> STRUCT
#[derive(Clone)]
pub struct _Term {
    pub numerator: Vec<u32>,
    pub denominator: Vec<u32>
}

//> TERM -> EVALUATE
impl _Term {pub fn evaluate(&self, runtime: &mut Runtime, id: u32, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let mut numerator = Vec::with_capacity(self.numerator.len());
    for &factor in &self.numerator {numerator.push(runtime.get(factor, memory))};
    let mut denominator = Vec::with_capacity(self.denominator.len());
    for &factor in &self.denominator {denominator.push(runtime.get(factor, memory))};
    //~ EVALUATE -> OPERATIONS
    self.space("Calculating numerator", id);
    let mut up = Object::Nexists(crate::Nexists::new());
    for factor in numerator {up = up.multiplication(&factor)};
    self.space("Calculating denominator", id);
    let mut down = Object::Nexists(crate::Nexists::new());
    for factor in denominator {down = down.multiplication(&factor)};
    self.space("Computing fraction", id);
    let inverse = down.invert();
    return up.multiplication(&inverse);
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