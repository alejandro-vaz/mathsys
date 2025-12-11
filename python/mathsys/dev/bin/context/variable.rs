//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::Display;
use crate::runtime::Object;
use crate::Debug;


//^
//^ VARIABLE
//^

//> VARIABLE -> STRUCT
#[derive(Clone)]
pub struct Variable {
    pub name: String
}

//> VARIABLE -> IMPLEMENTATION
impl Variable {
    pub fn unequivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => return item.unequivalency(&self.to()),
        Object::Nexists(item) => return item.unequivalency(&self.to()),
        Object::Number(item) => return item.unequivalency(&self.to()),
        Object::Tensor(item) => return item.unequivalency(&self.to()),
        Object::Undefined(item) => return item.unequivalency(&self.to()),
        Object::Variable(item) => return !self.equivalency(to)
    }}
    pub fn equivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => return item.equivalency(&self.to()),
        Object::Nexists(item) => return item.equivalency(&self.to()),
        Object::Number(item) => return item.equivalency(&self.to()),
        Object::Tensor(item) => return item.equivalency(&self.to()),
        Object::Undefined(item) => return item.equivalency(&self.to()),
        Object::Variable(item) => &self.name == &item.name
    }}
    pub fn negate(&self) -> Object {return self.partial(self.to())}
    pub fn summation(&self, to: &Object) -> Object {return self.partial(match to {
        Object::Infinite(item) => return item.summation(&self.to()),
        Object::Nexists(item) => return item.summation(&self.to()),
        Object::Number(item) => return item.summation(&self.to()),
        Object::Tensor(item) => return item.summation(&self.to()),
        Object::Undefined(item) => return item.summation(&self.to()),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
    pub fn invert(&self) -> Object {return self.partial(self.to())}
    pub fn multiplication(&self, to: &Object) -> Object {return self.partial(match to {
        Object::Infinite(item) => return item.multiplication(&self.to()),
        Object::Nexists(item) => return item.multiplication(&self.to()),
        Object::Number(item) => return item.multiplication(&self.to()),
        Object::Tensor(item) => return item.multiplication(&self.to()),
        Object::Undefined(item) => return item.multiplication(&self.to()),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
} impl Variable {
    fn locale0(&self, mutable: bool) -> () {crate::stdout::trace(format!("Setting {}mutable value for {}", if mutable {""} else {"im"}, self))}
    fn locale1(&self, class: u8) -> () {crate::stdout::trace(format!("Setting group of {} to {}", self, class))}
    fn locale2(&self) -> () {crate::stdout::trace(format!("Retrieving {} value", self))}
    fn locale3(&self) -> () {crate::stdout::alert(format!("Value of {} is not stored", self))}
    fn locale4(&self, class: u8, string: &str) -> () {crate::stdout::trace(format!("Checking if {} group code {} is compatible with {}", self, class, string))}
    fn locale5(&self) -> () {crate::stdout::trace(format!("Getting stored value of {}", self))}
    pub fn set(&self, value: Object, mutable: bool, context: &mut crate::runtime::Context, constraint: u8) -> () {
        //self.locale0(mutable);
        //self.setGroup(constraint, context);
        //if !self.compatible(constraint, value.id()) {crate::stdout::crash(crate::stdout::Code::RuntimeTypeMismatch)}
        //for (key, data) in &context.immutable {if key == &self.name {crate::stdout::crash(crate::stdout::Code::ImmutableModification)}}
        //self.partial(value.ctrlcv());
        //if mutable {
        //    for (key, data) in &mut context.mutable {if *key == self.name {*data = value; return}}
        //    context.mutable.insert(self.name.clone(), value);
        //} else {
        //    context.immutable.insert(self.name.clone(), value);
        //}
    }
    pub fn setGroup(&self, code: u8, context: &mut crate::runtime::Context) -> () {
        //let current = self.getGroup(context);
        //if code == 0 || current == code {return}
        //self.locale1(code);
        //if current != 0 {crate::stdout::crash(crate::stdout::Code::DoubleAnnotation)}
        //context.types.insert(self.name.clone(), code);
    }
    pub fn get(&self, context: &crate::runtime::Context) -> Object {
        self.locale2();
        for (key, value) in &context.immutable {if key == &self.name {return self.partial(value.clone())}}
        for (key, value) in &context.mutable {if key == &self.name {return self.partial(value.clone())}}
        self.locale3();
        return self.partial(Object::Undefined(crate::Undefined {}));
    }
    fn compatible(&self, code: u8, output: &str) -> bool {
        self.locale4(code, output);
        if code == 0 {return true} else {
            return output == match code {
                1 => "Infinite",
                2 => "Nexists",
                3 => "Number",
                4 => "Tensor",
                5 => "Undefined",
                6 => "Variable",
                other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
            }
        }
    }
    fn getGroup(&self, context: &crate::runtime::Context) -> u8 {
        self.locale5();
        for (key, value) in &context.types {if key == &self.name {return *value}}
        return 0;
    }
}

//> VARIABLE -> COMMON
impl Display for Variable {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl Debug for Variable {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 
impl Value for Variable {} impl Variable {
    pub fn to(&self) -> Object {return Object::Variable(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Variable")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "name = \"{}\"",
        self.name
    )}
}