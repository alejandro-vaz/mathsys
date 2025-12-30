//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Object,
    Group,
    crash,
    Code,
    fmt,
    Undefined,
    Value
};


//^
//^ TENSOR
//^

//> TENSOR -> STRUCT
#[derive(Clone)]
pub struct Tensor {
    pub values: Vec<Object>
} impl Tensor {pub fn new(values: Vec<Object>) -> Object {
    let values0 = values;
    return Object::Tensor(Tensor {
        values: values0
    });
}}

//> TENSOR -> CASTING
impl Tensor {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => crash(Code::FailedCast),
    Group::Natural => crash(Code::FailedCast),
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => crash(Code::FailedCast),
    Group::Tensor => self.into(),
    Group::Undefined => Undefined::new(),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => crash(Code::FailedCast)
}}}

//> TENSOR -> EQUIVALENCY
impl Tensor {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.into()),
    Object::Integer(item) => item.equivalency(&self.into()),
    Object::Natural(item) => item.equivalency(&self.into()),
    Object::Nexists(item) => item.equivalency(&self.into()),
    Object::Rational(item) => item.equivalency(&self.into()),
    Object::Tensor(item) => if self.values.len() == item.values.len() {
        let mut state = true;
        for index in 0..self.values.len() {state = state && self.values[index].equivalency(&item.values[index]); if !state {break}};
        state
    } else {false},
    Object::Undefined(item) => false,
    Object::Variable(item) => false,
    Object::Whole(item) => false
}}}

//> TENSOR -> SUMMATION
impl Tensor {
    pub fn absolute(&self) -> Object {Tensor::new(
        self.values.iter().map(|value| value.absolute()).collect()
    )}
    pub fn negate(&self) -> Object {Tensor::new(
        self.values.iter().map(|value| value.negate()).collect()
    )}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.into()),
        Object::Integer(item) => item.summation(&self.into()),
        Object::Natural(item) => item.summation(&self.into()),
        Object::Nexists(item) => item.summation(&self.into()),
        Object::Rational(item) => item.summation(&self.into()),
        Object::Tensor(item) => if self.values.len() == item.values.len() {
            let mut values = Vec::<Object>::new();
            for index in 0..self.values.len() {values.push(self.values[index].summation(&item.values[index]))};
            Tensor::new(values)
        } else {Undefined::new()},
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => crash(Code::Todo)
    }}
}

//> TENSOR -> MULTIPLICATION
impl Tensor {
    pub fn invert(&self) -> Object {return Undefined::new()}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.into()),
        Object::Integer(item) => item.multiplication(&self.into()),
        Object::Natural(item) => item.multiplication(&self.into()),
        Object::Nexists(item) => item.multiplication(&self.into()),
        Object::Rational(item) => item.multiplication(&self.into()),
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => Tensor::new(
            self.values.iter().map(|value| value.multiplication(&item.into())).collect()
        )
    }}
}

//> TENSOR -> REPRESENTATION
impl fmt::Display for Tensor {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for Tensor {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> TENSOR -> COMMON
impl Value for Tensor {} impl Tensor {
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "Tensor")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, 
        "{} > values = {:?}",
        self, self.values
    )}
}