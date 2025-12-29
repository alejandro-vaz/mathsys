//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Object,
    Group,
    crash,
    Code,
    Undefined,
    Value,
    fmt,
    Runtime
};


//^
//^ VARIABLE
//^

//> VARIABLE -> STRUCT
#[derive(Clone)]
pub struct Variable {
    pub name: String
} impl Variable {pub fn new(name: String) -> Object {
    let name0 = name;
    return Object::Variable(Variable {
        name: name0
    })
}}

//> VARIABLE -> CASTING
impl Variable {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => crash(Code::FailedCast),
    Group::Natural => crash(Code::FailedCast),
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => crash(Code::FailedCast),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Undefined::new(),
    Group::Variable => self.into(),
    Group::Whole => crash(Code::FailedCast)
}}}

//> VARIABLE -> EQUIVALENCY
impl Variable {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.into()),
    Object::Integer(item) => item.equivalency(&self.into()),
    Object::Natural(item) => item.equivalency(&self.into()),
    Object::Nexists(item) => item.equivalency(&self.into()),
    Object::Rational(item) => item.equivalency(&self.into()),
    Object::Tensor(item) => item.equivalency(&self.into()),
    Object::Undefined(item) => item.equivalency(&self.into()),
    Object::Variable(item) => self.name == item.name,
    Object::Whole(item) => false
}}}

//> VARIABLE -> SUMMATION
impl Variable {
    pub fn absolute(&self) -> Object {crash(Code::NoVariableOperation)}
    pub fn negate(&self) -> Object {crash(Code::NoVariableOperation)}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.into()),
        Object::Integer(item) => item.summation(&self.into()),
        Object::Natural(item) => item.summation(&self.into()),
        Object::Nexists(item) => item.summation(&self.into()),
        Object::Rational(item) => item.summation(&self.into()),
        Object::Tensor(item) => item.summation(&self.into()),
        Object::Undefined(item) => item.summation(&self.into()),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => crash(Code::NoVariableOperation)
    }}
}

//> VARIABLE -> MULTIPLICATION
impl Variable {
    pub fn invert(&self) -> Object {crash(Code::NoVariableOperation)}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.into()),
        Object::Integer(item) => item.multiplication(&self.into()),
        Object::Natural(item) => item.multiplication(&self.into()),
        Object::Nexists(item) => item.multiplication(&self.into()),
        Object::Rational(item) => item.multiplication(&self.into()),
        Object::Tensor(item) => item.multiplication(&self.into()),
        Object::Undefined(item) => item.multiplication(&self.into()),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => crash(Code::NoVariableOperation)
    }}
}

//> VARIABLE -> CUSTOM
impl Variable {
    pub fn get(&self, runtime: &Runtime) -> Object {return match runtime.immutable.get(&self.name) {
        Some(thing) => thing.clone(),
        None => match runtime.mutable.get(&self.name) {
            Some(thing) => thing.clone(),
            None => Undefined::new()
        }
    }}
    pub fn set(&self, value: Object, mutable: bool, runtime: &mut Runtime, group: Group) -> () {
        self.setGroup(group, runtime);
        let object = if value.is(group) {value} else {value.cast(group)};
        if let Some(thing) = runtime.immutable.get(&self.name) {crash(Code::ImmutableModification)} 
        if mutable {
            runtime.mutable.insert(self.name.clone(), object);
        } else {
            runtime.immutable.insert(self.name.clone(), object);
        }
    }
    fn setGroup(&self, code: Group, runtime: &mut Runtime) -> () {
        let current = self.getGroup(runtime);
        if code == Group::Undefined || current == code {return}
        if current != Group::Undefined {crash(Code::DoubleGroupAnnotation)}
        runtime.types.insert(self.name.clone(), code);
    }
    fn getGroup(&self, runtime: &Runtime) -> Group {return match runtime.types.get(&self.name) {
        Some(thing) => thing.clone(),
        None => Group::Undefined
    }}
}

//> VARIABLE -> REPRESENTATION
impl fmt::Display for Variable {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for Variable {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> VARIABLE -> COMMON
impl Value for Variable {} impl Variable {
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "Variable")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "{} > name = \"{}\"",
        self, self.name
    )}
}