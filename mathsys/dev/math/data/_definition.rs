//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Group,
    Pointer,
    Runtime,
    Class,
    Object,
    Undefined,
    fmt,
    Tip,
    crash,
    Code
};


//^
//^ DEFINITION
//^

//> DEFINITION -> STRUCT
#[derive(Clone)]
pub struct _Definition {
    pub group: Group,
    pub variable: Pointer,
    pub expression: Pointer
}

//> DEFINITION -> EVALUATE
impl _Definition {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let expression = runtime.get(self.expression, memory);
    let Object::Variable(variable) = runtime.get(self.variable, memory) else {crash(Code::FailedNamedRetrieval)};
    //~ EVALUATE -> OPERATIONS
    self.section("Setting immutable variable", id);
    variable.set(expression, false, runtime, Group::from(self.group));
    return Undefined::new();
}}

//> DEFINITION -> REPRESENTATION
impl fmt::Display for _Definition {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Definition {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> DEFINITION -> COMMON
impl Tip for _Definition {} impl _Definition {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Definition")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "group = {}, variable = {}, expression = {}",
        self.group, self.variable, self.expression
    )}
}