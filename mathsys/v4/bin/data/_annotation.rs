//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ ANNOTATION
//^

//> ANNOTATION -> STRUCT
#[derive(Clone)]
pub struct _Annotation {
    pub group: u8,
    pub variables: crate::Box<[u32]>
}

//> ANNOTATION -> IMPLEMENTATION
impl Display for _Annotation {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Annotation {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "group = {}, variables = {:?}",
    self.group, self.variables
)}} impl Class for _Annotation {
    fn name(&self) -> &'static str {"_Annotation"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &crate::Vec<crate::Box<dyn Class>>) -> crate::Box<dyn Value> {
        for &variable in &self.variables {context.process(variable, memory)}
        self.space("Setting class of variables", id);
        for &variable in &self.variables {
            let mut value = context.read(variable);
            let instance = crate::runtime::mutcast::<crate::Variable>(&mut *value);
            instance.setGroup(self.group, context);
        }
        return crate::Box::new(crate::Nexists {})
    }
}