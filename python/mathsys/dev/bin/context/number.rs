//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::runtime::Id;
use crate::Display;
use crate::Debug;


//^
//^ NUMBER
//^

//> NUMBER -> STRUCT
#[derive(Clone)]
pub struct Number {
    pub value: u32,
    pub shift: u8,
    pub negative: bool
}

//> NUMBER -> IMPLEMENTATION
impl Id for Number {const ID: &'static str = "Number";} 
impl Display for Number {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.id())}}
impl Debug for Number {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "value = {}, shift = {}, negative = {}",
    self.value, self.shift, self.negative
)}} impl Value for Number {
    fn id(&self) -> &'static str {return Self::ID}
    fn ctrlcv(&self) -> Box<dyn Value> {return Box::new(self.clone())}
    fn unequivalency(&self, to: &Box<dyn Value>) -> bool {self.genlocale0(to); return match to.id() {
        "Infinite" => return to.unequivalency(&self.ctrlcv()),
        "Nexists" => return to.unequivalency(&self.ctrlcv()),
        "Number" => !self.equivalency(to),
        "Tensor" => true,
        "Undefined" => false,
        "Variable" => true,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn equivalency(&self, to: &Box<dyn Value>) -> bool {self.genlocale1(to); return match to.id() {
        "Infinite" => return to.equivalency(&self.ctrlcv()),
        "Nexists" => return to.equivalency(&self.ctrlcv()),
        "Number" => {
            let value = crate::runtime::downcast::<crate::Number>(&**to);
            self.value == value.value && self.shift == value.shift && self.negative == value.negative
        },
        "Tensor" => false,
        "Undefined" => false,
        "Variable" => false,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn negate(&self) -> Box<dyn Value> {self.genlocale2(); return self.partial(Box::new(crate::Number {
        value: self.value,
        shift: self.shift,
        negative: !self.negative
    }))}
    fn summation(&self, to: &Box<dyn Value>) -> Box<dyn Value> {self.genlocale3(to); return self.partial(match to.id() {
        "Infinite" => return to.summation(&self.ctrlcv()),
        "Nexists" => return to.summation(&self.ctrlcv()),
        "Number" => {
            let value = crate::runtime::downcast::<crate::Number>(&**to);
            let shift = crate::max(self.shift, value.shift);
            let negative = if self.value >= value.value {self.negative} else {value.negative};
            Box::new(crate::Number {
                value: if self.negative == value.negative {
                    self.value*10u32.pow((shift - self.shift) as u32) + value.value*10u32.pow((shift - value.shift) as u32)
                } else {
                    if self.value >= value.value {
                        self.value*10u32.pow((shift - self.shift) as u32) - value.value*10u32.pow((shift - value.shift) as u32)
                    } else {
                        value.value*10u32.pow((shift - value.shift) as u32) - self.value*10u32.pow((shift - self.shift) as u32)
                    }
                },
                shift: shift,
                negative: negative
            })
        },
        "Tensor" => self.ctrlcv(),
        "Undefined" => to.ctrlcv(),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
    fn invert(&self) -> Box<dyn Value> {self.genlocale4(); return self.partial(Box::new(crate::Number {
        value: 10u32.pow(6 + self.shift as u32) / self.value,
        shift: 6,
        negative: self.negative
    }))}
    fn multiplication(&self, to: &Box<dyn Value>) -> Box<dyn Value> {self.genlocale5(to); return self.partial(match to.id() {
        "Infinite" => return to.multiplication(&self.ctrlcv()),
        "Nexists" => return to.multiplication(&self.ctrlcv()),
        "Number" => {
            let value = crate::runtime::downcast::<crate::Number>(&**to);
            Box::new(crate::Number {
                value: self.value * value.value,
                shift: self.shift + value.shift,
                negative: self.negative ^ value.negative
            })
        },
        "Tensor" => to.ctrlcv(),
        "Undefined" => to.ctrlcv(),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
} impl Number {}