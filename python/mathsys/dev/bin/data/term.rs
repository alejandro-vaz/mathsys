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
pub struct Term {
    numerator: crate::Box<[u32]>,
    denominator: crate::Box<[u32]>
}

//> TERM -> IMPLEMENTATION
impl crate::converter::Class for Term {
    fn name(&self) -> &'static str {"Term"}
    fn locale(&self, code: u8) -> () {match code {
        0 => crate::stdout::alert("To be developed, returning undefined meanwhile"),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        self.locale(0);
        return crate::Box::new(crate::_Undefined {})
    }
} impl Term {
    pub fn new(numerator: &[u32], denominator: &[u32]) -> Self {return Term {
        numerator: numerator.into(),
        denominator: denominator.into()
    }}
}