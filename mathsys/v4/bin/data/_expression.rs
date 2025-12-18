//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::converter::Class;
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ EXPRESSION
//^

//> EXPRESSION -> STRUCT
#[derive(Clone)]
pub struct _Expression {
    pub signs: crate::Box<[u8]>,
    pub terms: crate::Box<[u32]>
}

//> EXPRESSION -> IMPLEMENTATION
impl Display for _Expression {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Expression {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "signs = {:?}, terms = {:?}",
    self.signs, self.terms
)}} impl Class for _Expression {
    fn name(&self) -> &'static str {"_Expression"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &crate::Vec<crate::Box<dyn Class>>) -> crate::Box<dyn Value> {
        for &term in &self.terms {context.process(term, memory)}
        self.space("Summing up all terms", id);
        let mut current = crate::Box::new(crate::Nexists {}) as crate::Box<dyn Value>;
        for (index, term) in self.terms.iter().enumerate() {
            let next = context.read(*term);
            let value = if self.signs[index] % 2 == 0 {next.negate()} else {next};
            current = current.summation(&value);
        }
        return self.result(current);
    }
}