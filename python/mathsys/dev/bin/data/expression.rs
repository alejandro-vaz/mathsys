//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;


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
    fn locale(&self, code: u8) -> () {match code {
        0 => {crate::stdout::alert("To be developed")},
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value> {
        self.locale(0);
        return crate::Box::new(crate::_Undefined {});
    }
} impl Expression {
    pub fn new(terms: &[u32], signs: &[u8]) -> Self {return Expression {
        terms: terms.into(),
        signs: signs.into()
    }}
}