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
pub struct _Expression {
    pub terms: crate::Box<[u32]>,
    pub signs: crate::Box<[u8]>
}

//> EXPRESSION -> IMPLEMENTATION
impl Class for _Expression {
    fn name(&self) -> &'static str {"_Expression"}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > terms = [{}], signs = [{}]",
        self.name(),
        self.terms.iter().map(|id| crate::format!("{}", id)).collect::<crate::Vec<_>>().join(", "),
        self.signs.iter().map(|id| crate::format!("{}", id)).collect::<crate::Vec<_>>().join(", ")
    ))}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn Value> {
        self.space("Processing", id);
        let mut current = crate::Box::new(crate::Nexists {}) as crate::Box<dyn Value>;
        for (index, term) in self.terms.iter().enumerate() {
            context.process(self.terms[index]);
            self.space("Adding term to sum", id);
            let next = context.read(self.terms[index]);
            let negative = self.signs[index] % 2 == 0;
            current = current.summation(next, negative, false);
        }
        return current;
    }
}