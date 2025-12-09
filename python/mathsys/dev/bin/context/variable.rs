//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::runtime::Id;
use crate::Display;
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
impl Id for Variable {const ID: &'static str = "Variable";} 
impl Display for Variable {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "{}", self.id())}}
impl Debug for Variable {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
    "name = \"{}\"",
    self.name
)}} impl Value for Variable {
    fn id(&self) -> &'static str {return Self::ID}
    fn ctrlcv(&self) -> Box<dyn Value> {return Box::new(self.clone())}
    fn unequivalency(&self, to: &Box<dyn Value>) -> bool {self.genlocale0(to); return match to.id() {
        "Infinite" => return to.unequivalency(&self.ctrlcv()),
        "Nexists" => return to.unequivalency(&self.ctrlcv()),
        "Number" => return to.unequivalency(&self.ctrlcv()),
        "Tensor" => return to.unequivalency(&self.ctrlcv()),
        "Undefined" => return to.unequivalency(&self.ctrlcv()),
        "Variable" => !self.equivalency(to),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn equivalency(&self, to: &Box<dyn Value>) -> bool {self.genlocale1(to); return match to.id() {
        "Infinite" => return to.equivalency(&self.ctrlcv()),
        "Nexists" => return to.equivalency(&self.ctrlcv()),
        "Number" => return to.equivalency(&self.ctrlcv()),
        "Tensor" => return to.equivalency(&self.ctrlcv()),
        "Undefined" => return to.equivalency(&self.ctrlcv()),
        "Variable" => {
            let value = crate::runtime::downcast::<crate::Variable>(&**to);
            &self.name == &value.name
        },
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
    fn negate(&self) -> Box<dyn Value> {self.genlocale2(); return self.partial(Box::new(crate::Variable {
        name: self.name.clone()
    }))}
    fn summation(&self, to: &Box<dyn Value>) -> Box<dyn Value> {self.genlocale3(to); return self.partial(match to.id() {
        "Infinite" => return to.summation(&self.ctrlcv()),
        "Nexists" => return to.summation(&self.ctrlcv()),
        "Number" => return to.summation(&self.ctrlcv()),
        "Tensor" => return to.summation(&self.ctrlcv()),
        "Undefined" => return to.summation(&self.ctrlcv()),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
    fn invert(&self) -> Box<dyn Value> {self.genlocale4(); return self.partial(Box::new(crate::Variable {
        name: self.name.clone()
    }))}
    fn multiplication(&self, to: &Box<dyn Value>) -> Box<dyn Value> {self.genlocale5(to); return self.partial(match to.id() {
        "Infinite" => return to.multiplication(&self.ctrlcv()),
        "Nexists" => return to.multiplication(&self.ctrlcv()),
        "Number" => return to.multiplication(&self.ctrlcv()),
        "Tensor" => return to.multiplication(&self.ctrlcv()),
        "Undefined" => return to.multiplication(&self.ctrlcv()),
        "Variable" => crate::stdout::crash(crate::stdout::Code::UnexpectedValue),
        other => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
} impl Variable {
    fn locale0(&self, mutable: bool) -> () {crate::stdout::trace(format!("Setting {}mutable value for {}", if mutable {""} else {"im"}, self))}
    fn locale1(&self, class: u8) -> () {crate::stdout::trace(format!("Setting group of {} to {}", self, class))}
    fn locale2(&self) -> () {crate::stdout::trace(format!("Retrieving {} value", self))}
    fn locale3(&self) -> () {crate::stdout::alert(format!("Value of {} is not stored", self))}
    fn locale4(&self, class: u8, string: &str) -> () {crate::stdout::trace(format!("Checking if {} group code {} is compatible with {}", self, class, string))}
    fn locale5(&self) -> () {crate::stdout::trace(format!("Getting stored value of {}", self))}
    pub fn set(&self, value: Box<dyn Value>, mutable: bool, context: &mut crate::runtime::Context, constraint: u8) -> () {
        self.locale0(mutable);
        self.setGroup(constraint, context);
        if !self.compatible(constraint, value.id()) {crate::stdout::crash(crate::stdout::Code::RuntimeTypeMismatch)}
        for (key, data) in &context.immutable {if key == &self.name {crate::stdout::crash(crate::stdout::Code::ImmutableModification)}}
        self.partial(value.ctrlcv());
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
    pub fn get(&self, context: &crate::runtime::Context) -> Box<dyn Value> {
        self.locale2();
        for (key, value) in &context.immutable {if key == &self.name {return self.partial(value.ctrlcv())}}
        for (key, value) in &context.mutable {if key == &self.name {return self.partial(value.ctrlcv())}}
        self.locale3();
        return self.partial(Box::new(crate::Undefined {}));
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