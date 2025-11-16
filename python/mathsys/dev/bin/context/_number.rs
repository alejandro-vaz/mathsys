//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::runtime::Id;


//^
//^ NUMBER
//^

//> NUMBER -> STRUCT
#[derive(Clone)]
pub struct _Number {
    pub value: u32,
    pub shift: u8,
    pub negative: bool
}

//> NUMBER -> IMPLEMENTATION
impl Id for _Number {const ID: &'static str = "_Number";} 
impl Value for _Number {
    fn id(&self) -> &'static str {return Self::ID}
    fn ctrlcv(&self) -> crate::Box<dyn Value> {self.genlocale(0); return crate::Box::new(self.clone())}
    fn equiv(&self, to: crate::Box<dyn Value>) -> bool {self.genlocale(1); return match to.id() {
        "_Infinity" => to.equiv(self.ctrlcv()),
        "_Nexists" => to.equiv(self.ctrlcv()),
        "_Number" => {
            let value = crate::runtime::downcast::<crate::_Number>(&*to);
            self.value == value.value && self.shift == value.shift && self.negative == value.negative
        },
        "_Undefined" => false,
        "_Variable" => false,
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn summation(&mut self, mut to: crate::Box<dyn Value>, inverse: bool, selfinverse: bool) -> crate::Box<dyn Value> {
        self.genlocale(2);
        if selfinverse {self.negate()}; 
        return match to.id() {
            "_Infinity" => to.summation(self.ctrlcv(), false, inverse),
            "_Nexists" => to.summation(self.ctrlcv(), false, inverse),
            "_Number" => {
                let value = crate::runtime::mutcast::<crate::_Number>(&mut *to);
                if inverse {value.negate()}
                self.reduce(); value.reduce();
                let shift = crate::max(self.shift, value.shift);
                self.setShift(shift); value.setShift(shift);
                let total = {
                    if self.negative == value.negative {
                        self.value + value.value
                    } else {
                        if self.value >= value.value {self.value - value.value}
                        else {value.value - self.value}
                    }
                };
                let negative = {
                    if self.value >= value.value {self.negative}
                    else {value.negative}
                };
                return crate::Box::new(crate::_Number {
                    value: total,
                    shift: shift,
                    negative: negative
                });
            },
            "_Undefined" => to,
            "_Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
            other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        }
    }
    fn locale(&self, code: u8) -> () {match code {
        0 => crate::stdout::trace(&crate::format!(
            "Inverting sign of the number to be {}",
            if !self.negative {"positive"} else {"negative"}
        )),
        1 => crate::stdout::trace("Reducing the number minimum decimal places without losing precision"),
        2 => crate::stdout::trace("Adequating number to shift requirements"),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
} impl _Number {
    pub fn negate(&mut self) -> () {self.locale(0); self.negative = !self.negative}
    pub fn reduce(&mut self) -> () {self.locale(1); while self.value % 10 == 0 && self.shift != 0 {
        self.value = self.value / 10;
        self.shift -= 1;
    }}
    pub fn setShift(&mut self, shift: u8) -> () {
        self.locale(2);
        self.reduce();
        for each in 0..(shift - self.shift) {
            self.value = self.value * 10;
            self.shift += 1;
        }
    }
}