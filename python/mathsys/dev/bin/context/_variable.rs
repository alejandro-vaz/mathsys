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
    fn id(&self) -> &'static str {crate::ALLOCATOR.tempSpace(|| {crate::stdout::trace(&crate::format!(
        "Selected element is of type {}",
        Self::ID
    ))}); return Self::ID}
    fn ctrlcv(&self) -> crate::Box<dyn Value> {return crate::Box::new(self.clone())}
    fn locale(&self, code: u8) -> () {match code {
        0 => {crate::ALLOCATOR.tempSpace(|| {crate::stdout::trace(&crate::format!(
            "Setting mutable value for variable \"{}\"",
            &self.name
        ))})},
        1 => {crate::ALLOCATOR.tempSpace(|| {crate::stdout::trace(&crate::format!(
            "Setting immutable value for variable \"{}\"",
            &self.name
        ))})},
        2 => {crate::ALLOCATOR.tempSpace(|| {crate::stdout::trace(&crate::format!(
            "Obtaining value of variable \"{}\"",
            &self.name
        ))})},
        3 => {crate::ALLOCATOR.tempSpace(|| {crate::stdout::alert(&crate::format!(
            "Value of variable \"{}\" is not defined",
            &self.name
        ))})},
        _ => {crate::stdout::crash(crate::stdout::Code::LocaleNotFound)}
    }}
    fn equiv(&self, to: crate::Box<dyn Value>) -> bool {self.id(); return match to.id() {
        "_Infinity" => to.equiv(self.ctrlcv()),
        "_Nexists" => to.equiv(self.ctrlcv()),
        "_Number" => to.equiv(self.ctrlcv()),
        "_Undefined" => to.equiv(self.ctrlcv()),
        "_Variable" => {
            let value = crate::runtime::downcast::<crate::_Variable>(&*to);
            &self.name == &value.name
        },
        _ => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
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
    pub fn get<'a>(&self, context: &'a crate::runtime::Context) -> &'a dyn Value {
        self.locale(2);
        for (key, value) in &context.immutable {if key == &self.name {return &**value}}
        for (key, value) in &context.mutable {if key == &self.name {return &**value}}
        self.locale(3);
        return &crate::_Undefined {};
    }
}