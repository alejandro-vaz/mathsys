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
pub struct Variable {
    pub name: crate::String
}

//> VARIABLE -> IMPLEMENTATION
impl Id for Variable {const ID: &'static str = "Variable";} 
impl Value for Variable {
    fn id(&self) -> &'static str {return Self::ID}
    fn info(&self) -> () {crate::stdout::debug(&crate::format!(
        "{} > name = {}", 
        self.id(), 
        self.name
    ))}
    fn ctrlcv(&self) -> crate::Box<dyn Value> {self.genlocale(0); return crate::Box::new(self.clone())}
    fn equiv(&self, mut to: crate::Box<dyn Value>) -> bool {self.genlocale(1); return match to.id() {
        "Infinite" => to.equiv(self.ctrlcv()),
        "Nexists" => to.equiv(self.ctrlcv()),
        "Number" => to.equiv(self.ctrlcv()),
        "Undefined" => to.equiv(self.ctrlcv()),
        "Variable" => {
            let value = crate::runtime::mutcast::<crate::Variable>(&mut *to);
            &self.name == &value.name
        },
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn summation(&mut self, mut to: crate::Box<dyn Value>, inverse: bool, selfinverse: bool) -> crate::Box<dyn Value> {
        self.genlocale(2);
        return match to.id() {
            "Infinite" => to.summation(self.ctrlcv(), false, inverse),
            "Nexists" => to.summation(self.ctrlcv(), false, inverse),
            "Number" => to.summation(self.ctrlcv(), false, inverse),
            "Undefined" => to.summation(self.ctrlcv(), false, inverse),
            "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
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
} impl Variable {
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
        return crate::Box::new(crate::Undefined {});
    }
}