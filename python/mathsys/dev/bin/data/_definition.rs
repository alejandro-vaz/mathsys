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
    context.process(self.variable, memory);
    context.process(self.expression, memory);
    self.space("Setting immutable variable", id);
    let expression = context.read(self.expression);
    match context.read(self.variable) {
        Object::Variable(item) => item.set(expression, false, context, self.group),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }
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