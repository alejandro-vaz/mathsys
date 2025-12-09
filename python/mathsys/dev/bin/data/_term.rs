//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::reparser::Class;
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ TERM
//^

//> TERM -> STRUCT
#[derive(Clone)]
pub struct _Term {
    pub numerator: Box<[u32]>,
    pub denominator: Box<[u32]>
}

//> TERM -> IMPLEMENTATION
impl Display for _Term {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.name())}}
impl Debug for _Term {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "numerator = {:?}, denominator = {:?}",
    self.numerator, self.denominator
)}} impl Class for _Term {
    fn name(&self) -> &'static str {"_Term"}
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &Vec<Box<dyn Class>>) -> Box<dyn Value> {
        for &factor in &self.numerator {context.process(factor, memory)}
        for &factor in &self.denominator {context.process(factor, memory)}
        self.space("Calculating term", id);
        let mut numerator = Box::new(crate::Nexists {}) as Box<dyn Value>;
        for &factor in &self.numerator {
            let next = context.read(factor);
            numerator = numerator.multiplication(&next);
        }
        let mut denominator = Box::new(crate::Nexists {}) as Box<dyn Value>;
        for &factor in &self.denominator {
            let next = context.read(factor);
            denominator = denominator.multiplication(&next);
        }
        denominator = denominator.invert();
        return self.result(numerator.multiplication(&denominator))
    }
}