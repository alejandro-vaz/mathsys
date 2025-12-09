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
    fn ctrlcv(&self) -> Box<dyn Value> {return Box::new(self.clone())}
    fn unequivalency(&self, to: &Box<dyn Value>) -> bool {self.genlocale0(to); return match to.id() {
        "Infinite" => return to.unequivalency(&self.ctrlcv()),
        "Nexists" => return to.unequivalency(&self.ctrlcv()),
        "Number" => return to.unequivalency(&self.ctrlcv()),
        "Tensor" => false,
        "Undefined" => false,
        "Variable" => true,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn equivalency(&self, to: &Box<dyn Value>) -> bool {self.genlocale1(to); return match to.id() {
        "Infinite" => return to.equivalency(&self.ctrlcv()),
        "Nexists" => return to.equivalency(&self.ctrlcv()),
        "Number" => return to.equivalency(&self.ctrlcv()),
        "Tensor" => true,
        "Undefined" => false,
        "Variable" => false,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn negate(&self) -> Box<dyn Value> {self.genlocale2(); return self.partial(Box::new(crate::Tensor {}))}
    fn summation(&self, to: &Box<dyn Value>) -> Box<dyn Value> {self.genlocale3(to); return self.partial(match to.id() {
        "Infinite" => return to.summation(&self.ctrlcv()),
        "Nexists" => return to.summation(&self.ctrlcv()),
        "Number" => return to.summation(&self.ctrlcv()),
        "Tensor" => self.ctrlcv(),
        "Undefined" => to.ctrlcv(),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
    fn invert(&self) -> Box<dyn Value> {self.genlocale4(); return self.partial(Box::new(crate::Undefined {}))}
    fn multiplication(&self, to: &Box<dyn Value>) -> Box<dyn Value> {self.genlocale5(to); return self.partial(match to.id() {
        "Infinite" => return to.multiplication(&self.ctrlcv()),
        "Nexists" => return to.multiplication(&self.ctrlcv()),
        "Number" => return to.multiplication(&self.ctrlcv()),
        "Tensor" => to.ctrlcv(),
        "Undefined" => to.ctrlcv(),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
} impl Tensor {}