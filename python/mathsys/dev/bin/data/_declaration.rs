//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ DECLARATION
//^

//> DECLARATION -> STRUCT
#[derive(Clone)]
pub struct _Declaration {
    pub group: u8,
    pub variable: u32,
    pub expression: u32
}

//> DECLARATION -> EVALUATE
impl _Declaration {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    context.process(self.variable, memory);
    context.process(self.expression, memory);
    self.space("Setting mutable variable", id);
    let expression = context.read(self.expression);
    match context.read(self.variable) {
        Object::Variable(item) => item.set(expression, true, context, self.group),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }
    return Object::Nexists(crate::Nexists {});
}}

//> DECLARATION -> REPRESENTATION
impl crate::Display for _Declaration {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Declaration {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> DECLARATION -> COMMON
impl Tip for _Declaration {} impl _Declaration {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Declaration")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "group = {}, variable = {}, expression = {}",
        self.group, self.variable, self.expression
    )}
}