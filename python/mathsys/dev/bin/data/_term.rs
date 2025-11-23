//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


//^
//^ TERM
//^

//> TERM -> STRUCT
pub struct _Term {
    pub numerator: crate::Box<[u32]>,
    pub denominator: crate::Box<[u32]>
}

//> TERM -> IMPLEMENTATION
impl Class for _Term {
    fn name(&self) -> &'static str {"_Term"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > numerator = [{}], denominator = [{}]",
        self.name(),
        self.numerator.iter().map(|id| crate::format!("{}", id)).collect::<crate::Vec<_>>().join(", "),
        self.denominator.iter().map(|id| crate::format!("{}", id)).collect::<crate::Vec<_>>().join(", ")
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        return crate::Box::new(crate::Undefined {})
    }
}