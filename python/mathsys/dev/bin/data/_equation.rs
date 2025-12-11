//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ EQUATION
//^

//> EQUATION -> STRUCT
#[derive(Clone)]
pub struct _Equation {
    pub leftexpression: u32,
    pub rightexpression: u32
}

//> EQUATION -> EVALUATE
impl _Equation {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    context.process(self.leftexpression, memory);
    context.process(self.rightexpression, memory);
    self.space("Checking if both sides are equal", id);
    let leftexpression = context.read(self.leftexpression);
    let rightexpression = context.read(self.rightexpression);
    let equal = leftexpression.equivalency(&rightexpression);
    let unequal = leftexpression.unequivalency(&rightexpression);
    let determinable = equal == !unequal;
    return Object::Nexists(crate::Nexists {});
}}

//> EQUATION -> REPRESENTATION
impl crate::Display for _Equation {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Equation {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> EQUATION -> COMMON
impl Tip for _Equation {} impl _Equation {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Equation")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "leftexpression = {}, rightexpression = {}",
        self.leftexpression, self.rightexpression
    )}
}