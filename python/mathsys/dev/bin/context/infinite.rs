//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::Display;
use crate::Debug;


//^
//^ INFINITE
//^

//> INFINITE -> STRUCT
#[derive(Clone)]
pub struct Infinite {
    pub negative: bool
}

//> INFINITE -> IMPLEMENTATION
impl Display for Infinite {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.id())}}
impl Debug for Infinite {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "negative = {}",
    self.negative
)}} impl Value for Infinite {
    fn id(&self) -> &'static str {return "Infinite"}
    fn ctrlcv(&self) -> Box<dyn Value> {return Box::new(self.clone())}
    fn unequivalency(&self, to: &Box<dyn Value>) -> bool {self.genlocale0(to); return match to.id() {
        "Infinite" => !self.equivalency(to),
        "Nexists" => true,
        "Number" => true,
        "Tensor" => true,
        "Undefined" => false,
        "Variable" => true,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn equivalency(&self, to: &Box<dyn Value>) -> bool {self.genlocale1(to); return match to.id() {
        "Infinite" => {
            let value = crate::runtime::downcast::<crate::Infinite>(&**to);
            self.negative == value.negative
        },
        "Nexists" => false,
        "Number" => false,
        "Tensor" => false,
        "Undefined" => false,
        "Variable" => false,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn negate(&self) -> Box<dyn Value> {self.genlocale2(); return self.partial(Box::new(crate::Infinite {
        negative: !self.negative
    }))}
    fn summation(&self, to: &Box<dyn Value>) -> Box<dyn Value> {self.genlocale3(to); return self.partial(match to.id() {
        "Infinite" => {
            let value = crate::runtime::downcast::<crate::Infinite>(&**to);
            if self.negative != value.negative {Box::new(crate::Undefined {})} else {self.ctrlcv()}
        },
        "Nexists" => self.ctrlcv(),
        "Number" => self.ctrlcv(),
        "Tensor" => self.ctrlcv(),
        "Undefined" => to.ctrlcv(),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
    fn invert(&self) -> Box<dyn Value> {self.genlocale4(); return self.partial(Box::new(crate::Number {
        value: 0,
        shift: 0,
        negative: false
    }))}
    fn multiplication(&self, to: &Box<dyn Value>) -> Box<dyn Value> {self.genlocale5(to); return self.partial(match to.id() {
        "Infinite" => {
            let value = crate::runtime::downcast::<crate::Infinite>(&**to);
            Box::new(crate::Infinite {
                negative: self.negative != value.negative
            })
        },
        "Nexists" => self.ctrlcv(),
        "Number" => {
            let value = crate::runtime::downcast::<crate::Number>(&**to);
            if value.value == 0 {Box::new(crate::Undefined {})} else {Box::new(crate::Infinite {
                negative: self.negative != value.negative
            })}
        },
        "Tensor" => self.ctrlcv(),
        "Undefined" => to.ctrlcv(),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
} impl Infinite {}