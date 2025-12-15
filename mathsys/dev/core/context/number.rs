//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::value::Value;
use crate::object::Object;
use crate::group::Group;
use crate::stdout::{crash, Code};


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

//> NUMBER -> CASTING
impl Number {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::UnexpectedValue),
    Group::Nexists => crash(Code::UnexpectedValue),
    Group::Number => self.to(),
    Group::Tensor => crash(Code::UnexpectedValue),
    Group::Undefined => Object::Undefined(crate::Undefined {}),
    Group::Variable => crash(Code::UnexpectedValue)
}}}

//> NUMBER -> EQUIVALENCY
impl Number {
    pub fn unequivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => item.unequivalency(&self.to()),
        Object::Nexists(item) => item.unequivalency(&self.to()),
        Object::Number(item) => !self.equivalency(to),
        Object::Tensor(item) => true,
        Object::Undefined(item) => false,
        Object::Variable(item) => true
    }}
    pub fn equivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => item.equivalency(&self.to()),
        Object::Nexists(item) => item.equivalency(&self.to()),
        Object::Number(item) => self.value == item.value && self.shift == item.shift && self.negative == item.negative,
        Object::Tensor(item) => false,
        Object::Undefined(item) => false,
        Object::Variable(item) => false
    }}
}

//> NUMBER -> SUMMATION
impl Number {
    pub fn negate(&self) -> Object {return Object::Number(crate::Number {
        value: self.value,
        shift: self.shift,
        negative: !self.negative
    })}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.to()),
        Object::Nexists(item) => item.summation(&self.to()),
        Object::Number(item) => {
            let shift = crate::max(self.shift, item.shift);
            let negative = if self.value.pow((shift - self.shift) as u32) >= item.value.pow((shift - item.shift) as u32) {self.negative} else {item.negative};
            Object::Number(crate::Number {
                value: if self.negative == item.negative {
                    self.value*10u32.pow((shift - self.shift) as u32) + item.value*10u32.pow((shift - item.shift) as u32)
                } else {
                    if self.value.pow((shift - self.shift) as u32) >= item.value.pow((shift - item.shift) as u32) {
                        self.value*10u32.pow((shift - self.shift) as u32) - item.value*10u32.pow((shift - item.shift) as u32)
                    } else {
                        item.value*10u32.pow((shift - item.shift) as u32) - self.value*10u32.pow((shift - self.shift) as u32)
                    }
                },
                shift: shift,
                negative: negative
            })
        },
        Object::Tensor(item) => self.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::UnexpectedValue)
    }}
}

//> NUMBER -> MULTIPLICATION
impl Number {
    pub fn invert(&self) -> Object {return Object::Number(crate::Number {
        value: 10u32.pow(6 + self.shift as u32) / self.value,
        shift: 6,
        negative: self.negative
    })}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.to()),
        Object::Nexists(item) => item.multiplication(&self.to()),
        Object::Number(item) => Object::Number(crate::Number {
            value: self.value * item.value,
            shift: self.shift + item.shift,
            negative: self.negative ^ item.negative
        }),
        Object::Tensor(item) => item.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::UnexpectedValue)
    }}
}

//> NUMBER -> REPRESENTATION
impl crate::Display for Number {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for Number {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> NUMBER -> COMMON
impl Value for Number {} impl Number {
    pub fn to(&self) -> Object {return Object::Number(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Number")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "value = {}, shift = {}, negative = {}",
        self.value, self.shift, self.negative
    )}
}