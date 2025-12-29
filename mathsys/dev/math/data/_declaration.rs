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
    crash,
    Code,
    Undefined,
    fmt,
    Tip
};


//^
//^ DECLARATION
//^

//> DECLARATION -> STRUCT
#[derive(Clone)]
pub struct _Declaration {
    pub group: Group,
    pub variable: Pointer,
    pub expression: Pointer
}

//> DECLARATION -> EVALUATE
impl _Declaration {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> RETRIEVAL
    let expression = runtime.get(self.expression, memory);
    let Object::Variable(variable) = runtime.get(self.variable, memory) else {crash(Code::FailedNamedRetrieval)};
    //~ EVALUATE -> OPERATIONS
    self.section("Setting mutable variable", id);
    variable.set(expression, true, runtime, Group::from(self.group));
    return Undefined::new();
}}

//> DECLARATION -> REPRESENTATION
impl fmt::Display for _Declaration {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for _Declaration {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> DECLARATION -> COMMON
impl Tip for _Declaration {} impl _Declaration {
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "_Declaration")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "group = {}, variable = {}, expression = {}",
        self.group, self.variable, self.expression
    )}
}