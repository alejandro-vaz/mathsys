//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;


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
        0 => crate::stdout::debug(&crate::format!(
            "There {} {} term{}",
            if self.terms.len() == 1 {"is"} else {"are"},
            self.terms.len(),
            if self.terms.len() == 1 {""} else {"s"}
        )),
        1 => crate::stdout::space(&crate::format!(
            "[{}] Adding term to sum",
            self.name()
        )),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn Value> {
        self.locale(0);
        let mut current = crate::Box::new(crate::_Nexists {}) as crate::Box<dyn Value>;
        for (index, term) in self.terms.iter().enumerate() {
            context.process(self.terms[index]);
            self.locale(1);
            let next = context.read(self.terms[index]);
            let negative = self.signs[index] % 2 == 0;
            current = current.summation(next, negative, false);
        }
        return current;
    }
} impl Expression {
    pub fn new(terms: &[u32], signs: &[u8]) -> Self {return Expression {
        terms: terms.into(),
        signs: signs.into()
    }}
}