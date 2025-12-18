//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::runtime::Id;
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
impl Id for Infinite {const ID: &'static str = "Infinite";} 
impl Display for Infinite {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.id())}}
impl Debug for Infinite {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "negative = {}",
    self.negative
)}} impl Value for Infinite {
    fn id(&self) -> &'static str {return Self::ID}
    fn ctrlcv(&self) -> crate::Box<dyn Value> {return crate::Box::new(self.clone())}
    fn unequivalency(&self, to: &crate::Box<dyn Value>) -> bool {self.genlocale0(to); return match to.id() {
        "Infinite" => !self.equivalency(to),
        "Nexists" => true,
        "Number" => true,
        "Tensor" => true,
        "Undefined" => false,
        "Variable" => true,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn equivalency(&self, to: &crate::Box<dyn Value>) -> bool {self.genlocale1(to); return match to.id() {
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
    fn negate(&self) -> crate::Box<dyn Value> {self.genlocale2(); return self.partial(crate::Box::new(crate::Infinite {
        negative: !self.negative
    }))}
    fn summation(&self, to: &crate::Box<dyn Value>) -> crate::Box<dyn Value> {self.genlocale3(to); return self.partial(match to.id() {
        "Infinite" => {
            let value = crate::runtime::downcast::<crate::Infinite>(&**to);
            if self.negative != value.negative {crate::Box::new(crate::Undefined {})} else {self.ctrlcv()}
        },
        "Nexists" => self.ctrlcv(),
        "Number" => self.ctrlcv(),
        "Tensor" => self.ctrlcv(),
        "Undefined" => to.ctrlcv(),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
    fn invert(&self) -> crate::Box<dyn Value> {self.genlocale4(); return self.partial(crate::Box::new(crate::Number {
        value: 0,
        shift: 0,
        negative: false
    }))}
    fn multiplication(&self, to: &crate::Box<dyn Value>) -> crate::Box<dyn Value> {self.genlocale5(to); return self.partial(match to.id() {
        "Infinite" => {
            let value = crate::runtime::downcast::<crate::Infinite>(&**to);
            crate::Box::new(crate::Infinite {
                negative: self.negative != value.negative
            })
        },
        "Nexists" => self.ctrlcv(),
        "Number" => {
            let value = crate::runtime::downcast::<crate::Number>(&**to);
            if value.value == 0 {crate::Box::new(crate::Undefined {})} else {crate::Box::new(crate::Infinite {
                negative: self.negative != value.negative
            })}
        },
        "Tensor" => self.ctrlcv(),
        "Undefined" => to.ctrlcv(),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
} impl Infinite {
    fn locale1(&self) -> () {crate::stdout::trace(crate::format!("Taking absolute value of {}", self))}
    pub fn absolute(&mut self) -> () {self.locale1(); self.negative = false; self.partial(self.ctrlcv());}
}