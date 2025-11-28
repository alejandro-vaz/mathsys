//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::runtime::Id;
use crate::Display;
use crate::Debug;


//^
//^ TENSOR
//^

//> TENSOR -> CONSTRUCT
#[derive(Clone)]
pub struct Tensor {}

//> TENSOR -> IMPLEMENTATION
impl Id for Tensor {const ID: &'static str = "Tensor";} 
impl Display for Tensor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.id())}}
impl Debug for Tensor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    ""
)}} impl Value for Tensor {
    fn id(&self) -> &'static str {return Self::ID}
    fn ctrlcv(&self) -> crate::Box<dyn Value> {return crate::Box::new(self.clone())}
    fn unequivalency(&self, to: &crate::Box<dyn Value>) -> bool {self.genlocale0(to); return match to.id() {
        "Infinite" => return to.unequivalency(&self.ctrlcv()),
        "Nexists" => return to.unequivalency(&self.ctrlcv()),
        "Number" => return to.unequivalency(&self.ctrlcv()),
        "Tensor" => false,
        "Undefined" => false,
        "Variable" => true,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn equivalency(&self, to: &crate::Box<dyn Value>) -> bool {self.genlocale1(to); return match to.id() {
        "Infinite" => return to.equivalency(&self.ctrlcv()),
        "Nexists" => return to.equivalency(&self.ctrlcv()),
        "Number" => return to.equivalency(&self.ctrlcv()),
        "Tensor" => true,
        "Undefined" => false,
        "Variable" => false,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn negate(&self) -> crate::Box<dyn Value> {self.genlocale2(); return self.partial(crate::Box::new(crate::Tensor {}))}
    fn summation(&self, to: &crate::Box<dyn Value>) -> crate::Box<dyn Value> {self.genlocale3(to); return self.partial(match to.id() {
        "Infinite" => return to.summation(&self.ctrlcv()),
        "Nexists" => return to.summation(&self.ctrlcv()),
        "Number" => return to.summation(&self.ctrlcv()),
        "Tensor" => self.ctrlcv(),
        "Undefined" => to.ctrlcv(),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
    fn invert(&self) -> crate::Box<dyn Value> {self.genlocale4(); return self.partial(crate::Box::new(crate::Undefined {}))}
    fn multiplication(&self, to: &crate::Box<dyn Value>) -> crate::Box<dyn Value> {self.genlocale5(to); return self.partial(match to.id() {
        "Infinite" => return to.multiplication(&self.ctrlcv()),
        "Nexists" => return to.multiplication(&self.ctrlcv()),
        "Number" => return to.multiplication(&self.ctrlcv()),
        "Tensor" => to.ctrlcv(),
        "Undefined" => to.ctrlcv(),
        "Variable " => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
} impl Tensor {
    fn locale1(&self) -> () {crate::stdout::trace(crate::format!("Getting modulus of {}", self))}
    pub fn modulus(&self) -> crate::Box<dyn Value> {self.locale1(); return self.partial(crate::Box::new(crate::Number {
        value: 0,
        shift: 0,
        negative: false
    }))}
}