//^
//^ EXPRESSION
//^

//> EXPRESSION -> STRUCT
pub struct Expression {
    terms: crate::Box<[u32]>,
    signs: crate::Box<[u8]>
}

//> EXPRESSION -> IMPLEMENTATION
impl crate::converter::Class for Expression {
    fn name(&self) -> &'static str {"Expression"}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        self.locale(0);
        self.locale(1);
        return crate::Box::new(crate::_Undefined {});
    }
} impl Expression {
    pub fn new(terms: &[u32], signs: &[u8]) -> Self {return Expression {
        terms: terms.into(),
        signs: signs.into()
    }}
    fn locale(&self, code: u8) -> () {
        match code {
            0 => {crate::stdout::debug("To be developed, nothing here yet")},
            1 => {crate::stdout::trace("Returning an undefined placeholder")},
            _ => {}
        }
    }
}