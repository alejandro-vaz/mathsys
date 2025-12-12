//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::value::Value;
use crate::object::Object;


//^
//^ VARIABLE
//^

//> VARIABLE -> STRUCT
#[derive(Clone)]
pub struct Variable {
    pub name: String
}

//> VARIABLE -> EQUIVALENCY
impl Variable {
    pub fn unequivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => item.unequivalency(&self.to()),
        Object::Nexists(item) => item.unequivalency(&self.to()),
        Object::Number(item) => item.unequivalency(&self.to()),
        Object::Tensor(item) => item.unequivalency(&self.to()),
        Object::Undefined(item) => item.unequivalency(&self.to()),
        Object::Variable(item) => !self.equivalency(to)
    }}
    pub fn equivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => item.equivalency(&self.to()),
        Object::Nexists(item) => item.equivalency(&self.to()),
        Object::Number(item) => item.equivalency(&self.to()),
        Object::Tensor(item) => item.equivalency(&self.to()),
        Object::Undefined(item) => item.equivalency(&self.to()),
        Object::Variable(item) => &self.name == &item.name
    }}
}

//> VARIABLE -> SUMMATION
impl Variable {
    pub fn negate(&self) -> Object {return self.to()}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.to()),
        Object::Nexists(item) => item.summation(&self.to()),
        Object::Number(item) => item.summation(&self.to()),
        Object::Tensor(item) => item.summation(&self.to()),
        Object::Undefined(item) => item.summation(&self.to()),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
}

//> VARIABLE -> MULTIPLICATION
impl Variable {
    pub fn invert(&self) -> Object {return self.to()}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.to()),
        Object::Nexists(item) => item.multiplication(&self.to()),
        Object::Number(item) => item.multiplication(&self.to()),
        Object::Tensor(item) => item.multiplication(&self.to()),
        Object::Undefined(item) => item.multiplication(&self.to()),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
}

//> VARIABLE -> CUSTOM
impl Variable {
    fn locale0(&self, mutable: bool) -> () {crate::stdout::trace(format!("Setting {}mutable value for {}", if mutable {""} else {"im"}, self))}
    fn locale1(&self, class: u8) -> () {crate::stdout::trace(format!("Setting group of {} to {}", self, class))}
    fn locale2(&self) -> () {crate::stdout::trace(format!("Retrieving {} value", self))}
    fn locale3(&self) -> () {crate::stdout::alert(format!("Value of {} is not stored", self))}
    fn locale4<Type: crate::Display>(&self, class: u8, string: &Type) -> () {crate::stdout::trace(format!("Checking if {} group code {} is compatible with {}", self, class, string))}
    fn locale5(&self) -> () {crate::stdout::trace(format!("Getting stored value of {}", self))}
    pub fn set(&self, value: Object, mutable: bool, context: &mut crate::runtime::Context, constraint: u8) -> () {
        self.locale0(mutable);
        self.setGroup(constraint, context);
        if !self.compatible(constraint, &value) {crate::stdout::crash(crate::stdout::Code::RuntimeTypeMismatch)}
        for (key, data) in &context.immutable {if key == &self.name {crate::stdout::crash(crate::stdout::Code::ImmutableModification)}}
        if mutable {
            for (key, data) in &mut context.mutable {if *key == self.name {*data = value; return}}
            context.mutable.insert(self.name.clone(), value);
        } else {
            context.immutable.insert(self.name.clone(), value);
        }
    }
    pub fn setGroup(&self, code: u8, context: &mut crate::runtime::Context) -> () {
        let current = self.getGroup(context);
        if code == 0 || current == code {return}
        self.locale1(code);
        if current != 0 {crate::stdout::crash(crate::stdout::Code::DoubleAnnotation)}
        context.types.insert(self.name.clone(), code);
    }
    pub fn get(&self, context: &crate::runtime::Context) -> Object {
        self.locale2();
        for (key, value) in &context.immutable {if key == &self.name {return value.clone()}}
        for (key, value) in &context.mutable {if key == &self.name {return value.clone()}}
        self.locale3();
        return Object::Undefined(crate::Undefined {});
    }
    fn compatible(&self, code: u8, object: &Object) -> bool {
        self.locale4(code, object);
        if code == 0 {return true} else {
            return format!("{}", object) == match code {
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