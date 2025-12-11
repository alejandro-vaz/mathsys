//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::object::Object;
use crate::runtime::Context;
use crate::tip::Tip;


//^
//^ ANNOTATION
//^

//> ANNOTATION -> STRUCT
#[derive(Clone)]
pub struct _Annotation {
    pub group: u8,
    pub variables: Box<[u32]>
}

//> ANNOTATION -> EVALUATE
impl _Annotation {pub fn evaluate(&self, context: &mut Context, id: u32, memory: &Vec<Class>) -> Object {
    for &variable in &self.variables {context.process(variable, memory)}
    self.space("Setting class of variables", id);
    for &variable in &self.variables {match context.read(variable) {
        Object::Variable(item) => item.setGroup(self.group, context),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    return Object::Nexists(crate::Nexists {})
}}

//> ANNOTATION -> REPRESENTATION
impl crate::Display for _Annotation {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for _Annotation {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> ANNOTATION -> COMMON
impl Tip for _Annotation {} impl _Annotation {
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "_Annotation")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "group = {}, variables = {:?}",
        self.group, self.variables
    )}
}