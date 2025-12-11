//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::reparser::Class;
use crate::runtime::Object;
use crate::Display;
use crate::Debug;


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

//> DEFINITION -> IMPLEMENTATION
impl Display for _Definition {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Definition {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "group = {}, variable = {}, expression = {}",
    self.group, self.variable, self.expression
)}} impl Class for _Definition {
    fn name(&self) -> &'static str {"_Definition"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &Vec<Box<dyn Class>>) -> Object {
        context.process(self.variable, memory);
        context.process(self.expression, memory);
        self.space("Setting immutable variable", id);
        let expression = self.result(context.read(self.expression));
        match context.read(self.variable) {
            Object::Variable(item) => item.set(expression, false, context, self.group),
            other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        }
        return Object::Nexists(crate::Nexists {});
    }
}