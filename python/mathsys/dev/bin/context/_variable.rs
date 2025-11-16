//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::runtime::Id;


//^
//^ VARIABLE
//^

//> VARIABLE -> STRUCT
#[derive(Clone)]
pub struct _Variable {
    pub name: crate::String
}

//> VARIABLE -> IMPLEMENTATION
impl Id for _Variable {const ID: &'static str = "_Variable";} 
impl Value for _Variable {
    fn id(&self) -> &'static str {return Self::ID}
    fn ctrlcv(&self) -> crate::Box<dyn Value> {self.genlocale(0); return crate::Box::new(self.clone())}
    fn equiv(&self, to: crate::Box<dyn Value>) -> bool {self.genlocale(1); return match to.id() {
        "_Infinity" => to.equiv(self.ctrlcv()),
        "_Nexists" => to.equiv(self.ctrlcv()),
        "_Number" => to.equiv(self.ctrlcv()),
        "_Undefined" => to.equiv(self.ctrlcv()),
        "_Variable" => {
            let value = crate::runtime::downcast::<crate::_Variable>(&*to);
            &self.name == &value.name
        },
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn summation(&mut self, mut to: crate::Box<dyn Value>, inverse: bool, selfinverse: bool) -> crate::Box<dyn Value> {
        self.genlocale(2);
        return match to.id() {
            "_Infinity" => to.summation(self.ctrlcv(), false, inverse),
            "_Nexists" => to.summation(self.ctrlcv(), false, inverse),
            "_Number" => to.summation(self.ctrlcv(), false, inverse),
            "_Undefined" => to.summation(self.ctrlcv(), false, inverse),
            "_Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
            other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
        }
    }
    fn locale(&self, code: u8) -> () {match code {
        0 => crate::stdout::trace(&crate::format!(
            "Setting mutable value for variable \"{}\"",
            &self.name
        )),
        1 => crate::stdout::trace(&crate::format!(
            "Setting immutable value for variable \"{}\"",
            &self.name
        )),
        2 => crate::stdout::trace(&crate::format!(
            "Obtaining value of variable \"{}\"",
            &self.name
        )),
        3 => crate::stdout::alert(&crate::format!(
            "Value of variable \"{}\" is not defined",
            &self.name
        )),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
} impl _Variable {
    pub fn set(&self, value: crate::Box<dyn Value>, mutable: bool, context: &mut crate::runtime::Context) -> () {
        if mutable {self.locale(0)} else {self.locale(1)};
        for (key, data) in &context.immutable {
            if key == &self.name {crate::stdout::crash(crate::stdout::Code::ImmutableModification)}
        }
        if mutable {
            for (key, data) in &mut context.mutable {if *key == self.name {*data = value; return}}
            context.mutable.push((self.name.clone(), value));
        } else {
            context.immutable.push((self.name.clone(), value));
        }
    }
    pub fn get(&self, context: &crate::runtime::Context) -> crate::Box<dyn Value> {
        self.locale(2);
        for (key, value) in &context.immutable {if key == &self.name {return value.ctrlcv()}}
        for (key, value) in &context.mutable {if key == &self.name {return value.ctrlcv()}}
        self.locale(3);
        return (crate::_Undefined {}).ctrlcv();
    }
}