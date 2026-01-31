//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::value::Value;
use crate::object::Object;
use crate::group::Group;
use crate::stdout::{crash, Code};


//^
//^ VARIABLE
//^

//> VARIABLE -> STRUCT
#[derive(Clone)]
pub struct Variable {
    pub name: String
} impl Variable {pub fn new(name: String) -> Self {return Variable {
    name: name
}}}

//> VARIABLE -> CASTING
impl Variable {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => crash(Code::FailedCast),
    Group::Natural => crash(Code::FailedCast),
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => crash(Code::FailedCast),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Object::Undefined(crate::Undefined::new()),
    Group::Variable => self.to(),
    Group::Whole => crash(Code::FailedCast)
}}}

//> VARIABLE -> EQUIVALENCY
impl Variable {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.to()),
    Object::Integer(item) => item.equivalency(&self.to()),
    Object::Natural(item) => item.equivalency(&self.to()),
    Object::Nexists(item) => item.equivalency(&self.to()),
    Object::Rational(item) => item.equivalency(&self.to()),
    Object::Tensor(item) => item.equivalency(&self.to()),
    Object::Undefined(item) => item.equivalency(&self.to()),
    Object::Variable(item) => &self.name == &item.name,
    Object::Whole(item) => false
}}}

//> VARIABLE -> SUMMATION
impl Variable {
    pub fn absolute(&self) -> Object {crash(Code::NoVariableOperation)}
    pub fn negate(&self) -> Object {crash(Code::NoVariableOperation)}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.to()),
        Object::Integer(item) => item.summation(&self.to()),
        Object::Natural(item) => item.summation(&self.to()),
        Object::Nexists(item) => item.summation(&self.to()),
        Object::Rational(item) => item.summation(&self.to()),
        Object::Tensor(item) => item.summation(&self.to()),
        Object::Undefined(item) => item.summation(&self.to()),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => crash(Code::NoVariableOperation)
    }}
}

//> VARIABLE -> MULTIPLICATION
impl Variable {
    pub fn invert(&self) -> Object {crash(Code::NoVariableOperation)}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.to()),
        Object::Integer(item) => item.multiplication(&self.to()),
        Object::Natural(item) => item.multiplication(&self.to()),
        Object::Nexists(item) => item.multiplication(&self.to()),
        Object::Rational(item) => item.multiplication(&self.to()),
        Object::Tensor(item) => item.multiplication(&self.to()),
        Object::Undefined(item) => item.multiplication(&self.to()),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => crash(Code::NoVariableOperation)
    }}
}

//> VARIABLE -> CUSTOM
impl Variable {
    pub fn get(&self, runtime: &crate::runtime::Runtime) -> Object {
        for (key, value) in &runtime.immutable {if key == &self.name {return value.clone()}}
        for (key, value) in &runtime.mutable {if key == &self.name {return value.clone()}}
        return Object::Undefined(crate::Undefined::new());
    }
    pub fn set(&self, value: Object, mutable: bool, runtime: &mut crate::runtime::Runtime, group: Group) -> () {
        self.setGroup(group, runtime);
        let object = if value.is(group) {value} else {value.cast(group)};
        for (key, data) in &runtime.immutable {if key == &self.name {crash(Code::ImmutableModification)}}
        if mutable {
            for (key, data) in &mut runtime.mutable {if *key == self.name {*data = object; return}}
            runtime.mutable.insert(self.name.clone(), object);
        } else {
            runtime.immutable.insert(self.name.clone(), object);
        }
    }
    fn setGroup(&self, code: Group, runtime: &mut crate::runtime::Runtime) -> () {
        let current = self.getGroup(runtime);
        if code == Group::Undefined || current == code {return}
        if current != Group::Undefined {crash(Code::DoubleGroupAnnotation)}
        runtime.types.insert(self.name.clone(), code);
    }
    fn getGroup(&self, runtime: &crate::runtime::Runtime) -> Group {
        for (key, value) in &runtime.types {if key == &self.name {return *value}}
        return Group::Undefined;
    }
}

//> VARIABLE -> REPRESENTATION
impl crate::Display for Variable {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for Variable {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> VARIABLE -> COMMON
impl Value for Variable {} impl Variable {
    pub fn to(&self) -> Object {return Object::Variable(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Variable")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "name = \"{}\"",
        self.name
    )}
}