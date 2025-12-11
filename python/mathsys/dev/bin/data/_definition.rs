//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ DEFINITION
//^

//> DEFINITION -> STRUCT
#[derive(Clone)]
pub struct _Definition {
    pub group: u8,
    pub variable: u32,
    pub expression: u32
}

//> DEFINITION -> EVALUATE
impl _Definition {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let expression = context.get(self.expression, memory);
    let Object::Variable(variable) = context.get(self.variable, memory) else {crate::stdout::crash(crate::stdout::Code::UnexpectedValue)};
    //~ EVALUATE -> OPERATIONS
    self.space("Setting mutable variable", id);
    variable.set(expression, false, context, self.group);
    return Object::Nexists(crate::Nexists {});
}}

//> DEFINITION -> REPRESENTATION
impl crate::Display for _Definition {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Definition {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> DEFINITION -> COMMON
impl Tip for _Definition {} impl _Definition {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Definition")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "group = {}, variable = {}, expression = {}",
        self.group, self.variable, self.expression
    )}
}