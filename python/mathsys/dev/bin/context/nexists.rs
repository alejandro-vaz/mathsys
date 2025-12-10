//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ NEXISTS
//^

//> NEXISTS -> CONSTRUCT
#[derive(Clone)]
pub struct Nexists {}

//> NEXISTS -> IMPLEMENTATION
impl Display for Nexists {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.id())}}
impl Debug for Nexists {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    ""
)}} impl Value for Nexists {
    fn id(&self) -> &'static str {return "Nexists"}
    fn ctrlcv(&self) -> Box<dyn Value> {return Box::new(self.clone())}
    fn unequivalency(&self, to: &Box<dyn Value>) -> bool {self.genlocale0(to); return match to.id() {
        "Infinite" => return to.unequivalency(&self.ctrlcv()),
        "Nexists" => true,
        "Number" => true,
        "Tensor" => true,
        "Undefined" => false,
        "Variable" => true,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn equivalency(&self, to: &Box<dyn Value>) -> bool {self.genlocale1(to); return match to.id() {
        "Infinite" => return to.equivalency(&self.ctrlcv()),
        "Nexists" => false,
        "Number" => false,
        "Tensor" => false,
        "Undefined" => false,
        "Variable" => false,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn negate(&self) -> Box<dyn Value> {self.genlocale2(); return self.partial(Box::new(crate::Nexists {}))}
    fn summation(&self, to: &Box<dyn Value>) -> Box<dyn Value> {self.genlocale3(to); return self.partial(match to.id() {
        "Infinite" => return to.summation(&self.ctrlcv()),
        "Nexists" => to.ctrlcv(),
        "Number" => to.ctrlcv(),
        "Tensor" => to.ctrlcv(),
        "Undefined" => to.ctrlcv(),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
    fn invert(&self) -> Box<dyn Value> {self.genlocale4(); return self.partial(Box::new(crate::Nexists {}))}
    fn multiplication(&self, to: &Box<dyn Value>) -> Box<dyn Value> {self.genlocale5(to); return self.partial(match to.id() {
        "Infinite" => return to.multiplication(&self.ctrlcv()),
        "Nexists" => to.ctrlcv(),
        "Number" => to.ctrlcv(),
        "Tensor" => to.ctrlcv(),
        "Undefined" => to.ctrlcv(),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
} impl Nexists {}