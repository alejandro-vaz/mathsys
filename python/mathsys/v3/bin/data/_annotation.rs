//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ ANNOTATION
//^

//> ANNOTATION -> STRUCT
pub struct _Annotation {
    pub object: u8,
    pub variable: u32
}

//> ANNOTATION -> IMPLEMENTATION
impl Class for _Annotation {
    fn name(&self) -> &'static str {"_Annotation"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > object = {}, variable = {}",
        self.name(),
        self.object,
        self.variable
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        self.space("Processing", id);
        context.process(self.variable);
        self.space("Setting class of variable", id);
        let mut value = context.read(self.variable);
        let instance = crate::runtime::mutcast::<crate::Variable>(&mut *value);
        instance.setType(self.object, context);
        return crate::Box::new(crate::Nexists {});
    }
}