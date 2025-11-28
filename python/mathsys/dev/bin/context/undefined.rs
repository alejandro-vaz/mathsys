//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::runtime::Id;
use crate::Display;
use crate::Debug;


//^
//^ UNDEFINED
//^

//> UNDEFINED -> STRUCT
#[derive(Clone)]
pub struct Undefined {}

//> UNDEFINED -> IMPLEMENTATION
impl Id for Undefined {const ID: &'static str = "Undefined";} 
impl Display for Undefined {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.id())}}
impl Debug for Undefined {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    ""
)}} impl Value for Undefined {
    fn id(&self) -> &'static str {return Self::ID}
    fn ctrlcv(&self) -> crate::Box<dyn Value> {return crate::Box::new(self.clone())}
    fn unequivalency(&self, to: &crate::Box<dyn Value>) -> bool {self.genlocale0(to); return match to.id() {
        "Infinite" => return to.unequivalency(&self.ctrlcv()),
        "Nexists" => return to.unequivalency(&self.ctrlcv()),
        "Number" => return to.unequivalency(&self.ctrlcv()),
        "Tensor" => return to.unequivalency(&self.ctrlcv()),
        "Undefined" => true,
        "Variable" => false,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn equivalency(&self, to: &crate::Box<dyn Value>) -> bool {self.genlocale1(to); return match to.id() {
        "Infinite" => return to.equivalency(&self.ctrlcv()),
        "Nexists" => return to.equivalency(&self.ctrlcv()),
        "Number" => return to.equivalency(&self.ctrlcv()),
        "Tensor" => return to.equivalency(&self.ctrlcv()),
        "Undefined" => false,
        "Variable" => false,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn negate(&self) -> crate::Box<dyn Value> {self.genlocale2(); return self.partial(crate::Box::new(crate::Undefined {}))}
    fn summation(&self, to: &crate::Box<dyn Value>) -> crate::Box<dyn Value> {self.genlocale3(to); return self.partial(match to.id() {
        "Infinite" => return to.summation(&self.ctrlcv()),
        "Nexists" => return to.summation(&self.ctrlcv()),
        "Number" => return to.summation(&self.ctrlcv()),
        "Tensor" => return to.summation(&self.ctrlcv()),
        "Undefined" => to.ctrlcv(),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
    fn invert(&self) -> crate::Box<dyn Value> {self.genlocale4(); return self.partial(crate::Box::new(crate::Undefined {}))}
    fn multiplication(&self, to: &crate::Box<dyn Value>) -> crate::Box<dyn Value> {self.genlocale5(to); return self.partial(match to.id() {
        "Infinite" => return to.multiplication(&self.ctrlcv()),
        "Nexists" => return to.multiplication(&self.ctrlcv()),
        "Number" => return to.multiplication(&self.ctrlcv()),
        "Tensor" => return to.multiplication(&self.ctrlcv()),
        "Undefined" => to.ctrlcv(),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
} impl Undefined {}