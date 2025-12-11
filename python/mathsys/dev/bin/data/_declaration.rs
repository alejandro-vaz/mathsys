//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::reparser::Class;
use crate::runtime::Object;
use crate::Display;
use crate::Debug;


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

//> DECLARATION -> IMPLEMENTATION
impl Display for _Declaration {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Declaration {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "group = {}, variable = {}, expression = {}",
    self.group, self.variable, self.expression
)}} impl Class for _Declaration {
    fn name(&self) -> &'static str {"_Declaration"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &Vec<Box<dyn Class>>) -> Object {
        context.process(self.variable, memory);
        context.process(self.expression, memory);
        self.space("Setting mutable variable", id);
        let expression = self.result(context.read(self.expression));
        match context.read(self.variable) {
            Object::Variable(item) => item.set(expression, true, context, self.group),
            other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        }
        return Object::Nexists(crate::Nexists {});
    }
}