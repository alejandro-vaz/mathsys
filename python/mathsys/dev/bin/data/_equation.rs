//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ EQUATION
//^

//> EQUATION -> STRUCT
#[derive(Clone)]
pub struct _Equation {
    pub leftexpression: u32,
    pub rightexpression: u32
}

//> EQUATION -> IMPLEMENTATION
impl Display for _Equation {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Equation {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "leftexpression = {}, rightexpression = {}",
    self.leftexpression, self.rightexpression
)}} impl Class for _Equation {
    fn name(&self) -> &'static str {"_Equation"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &crate::Vec<crate::Box<dyn Class>>) -> crate::Box<dyn Value> {
        context.process(self.leftexpression, memory);
        context.process(self.rightexpression, memory);
        self.space("Checking if both sides are equal", id);
        let leftexpression = context.read(self.leftexpression);
        let rightexpression = context.read(self.rightexpression);
        let equal = leftexpression.equivalency(&rightexpression);
        let unequal = leftexpression.unequivalency(&rightexpression);
        let determinable = equal == !unequal;
        return crate::Box::new(crate::Nexists {});
    }
}