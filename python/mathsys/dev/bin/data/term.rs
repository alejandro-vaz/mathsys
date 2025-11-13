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
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        return crate::Box::new(crate::_Undefined {})
    }
    fn locale(&self, code: u8) -> () {match code {
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
} impl Term {pub fn new(numerator: &[u32], denominator: &[u32]) -> Self {return Term {
    numerator: numerator.into(),
    denominator: denominator.into()
}}}