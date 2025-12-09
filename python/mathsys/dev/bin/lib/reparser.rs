//^
//^ CLASS
//^

//> CLASS -> TRAIT
pub trait Class: crate::Display + crate::Debug {
    fn name(&self) -> &'static str;
    fn space(&self, message: &str, id: u32) -> () {crate::stdout::space(format!(
        "{{{}{}}} {}",
        id, self, message
    )); self.info()}
    fn info(&self) -> () {crate::stdout::debug(format!(
        "{:?}",
        self
    ))}
    fn result(&self, value: Box<dyn crate::runtime::Value>) -> Box<dyn crate::runtime::Value> {
        crate::stdout::class(format!(
            "{} > {:?}",
            value, value
        ));
        return value;
    }
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32, memory: &Vec<Box<dyn Class>>) -> Box<dyn crate::runtime::Value>;
}


//^
//^ REPARSER
//^

//> REPARSER -> STRUCT
pub struct Reparser {
    locus: usize,
    memory: Vec<Box <dyn Class>>
}

//> REPARSER -> IMPLEMENTATION
impl Reparser {
    pub fn run(&mut self) -> &Vec<Box<dyn Class>> {
        crate::stdout::space("{REPARSER} Processing IR");
        while self.locus < crate::SETTINGS.ir.len() {
            let object = match self.take8() {
                0x01 => self.Start(),
                0x02 => self.Declaration(),
                0x03 => self.Definition(),
                0x04 => self.Annotation(),
                0x05 => self.Node(),
                0x06 => self.Equation(),
                0x07 => self.Comment(),
                0x08 => self.Use(),
                0x09 => self.Expression(),
                0x0A => self.Term(),
                0x0B => self.Factor(),
                0x0C => self.Limit(),
                0x0D => self.Infinite(),
                0x0E => self.Variable(),
                0x0F => self.Nest(),
                0x10 => self.Tensor(),
                0x11 => self.Number(),
                other => crate::stdout::crash(crate::stdout::Code::UnknownIRObject)
            };
            crate::stdout::trace(format!(
                "Creating {} data structure",
                object.name()
            ));
            self.memory.push(object);
        };
        return &self.memory;
    }
    fn Annotation(&mut self) -> Box<dyn Class> {return Box::new(crate::_Annotation {
        group: self.take8(),
        variables: self.list32()
    })}
    fn Comment(&mut self) -> Box<dyn Class> {return Box::new(crate::_Comment {
        text: self.listchar()
    })}
    fn Declaration(&mut self) -> Box<dyn Class> {return Box::new(crate::_Declaration {
        group: self.take8(),
        variable: self.take32(),
        expression: self.take32()
    })}
    fn Definition(&mut self) -> Box<dyn Class> {return Box::new(crate::_Definition {
        group: self.take8(),
        variable: self.take32(),
        expression: self.take32()
    })}
    fn Equation(&mut self) -> Box<dyn Class> {return Box::new(crate::_Equation {
        leftexpression: self.take32(),
        rightexpression: self.take32()
    })}
    fn Expression(&mut self) -> Box<dyn Class> {return Box::new(crate::_Expression {
        signs: self.list8(),
        terms: self.list32()
    })}
    fn Factor(&mut self) -> Box<dyn Class> {return Box::new(crate::_Factor {
        value: self.take32(),
        exponent: self.take32()
    })}
    fn Infinite(&mut self) -> Box<dyn Class> {return Box::new(crate::_Infinite {})}
    fn Limit(&mut self) -> Box<dyn Class> {return Box::new(crate::_Limit {
        variable: self.take32(),
        approach: self.take32(),
        direction: self.take8(),
        nest: self.take32(),
        exponent: self.take32()
    })}
    fn Nest(&mut self) -> Box<dyn Class> {return Box::new(crate::_Nest {
        expression: self.take32()
    })}
    fn Node(&mut self) -> Box<dyn Class> {return Box::new(crate::_Node {
        expression: self.take32()
    })}
    fn Number(&mut self) -> Box<dyn Class> {return Box::new(crate::_Number {
        value: self.take32(),
        shift: self.take8()
    })}
    fn Start(&mut self) -> Box<dyn Class> {return Box::new(crate::_Start {
        statements: self.list32()
    })}
    fn Tensor(&mut self) -> Box<dyn Class> {return Box::new(crate::_Tensor {
        values: self.list32()
    })}
    fn Term(&mut self) -> Box<dyn Class> {return Box::new(crate::_Term {
        numerator: self.list32(),
        denominator: self.list32()
    })}
    fn Use(&mut self) -> Box<dyn Class> {return Box::new(crate::_Use {
        name: self.listchar(),
        start: self.take32()
    })}
    fn Variable(&mut self) -> Box<dyn Class> {return Box::new(crate::_Variable {
        representation: self.listchar()
    })}
}

//> REPARSER -> METHODS
impl Reparser {
    pub fn new() -> Self {return Reparser { 
        locus: 0,
        memory: Vec::<Box<dyn Class>>::new()
    }}
    fn take8(&mut self) -> u8 {
        self.check(1);
        let value = crate::SETTINGS.ir[self.locus];
        self.locus += 1;
        return value;
    }
    fn take32(&mut self) -> u32 {
        self.check(4);
        let value = &crate::SETTINGS.ir[self.locus..self.locus + 4];
        self.locus += 4;
        return u32::from_le_bytes([value[0], value[1], value[2], value[3]]);
    }
    fn list8(&mut self) -> Box<[u8]> {
        let mut values = Vec::<u8>::new();
        loop {
            let value = self.take8();
            if value == 0 {break} else {values.push(value)}
        }
        return values.into_boxed_slice();
    }
    fn list32(&mut self) -> Box<[u32]> {
        let mut values = Vec::<u32>::new();    
        loop {
            let value = self.take32();
            if value == 0 {break} else {values.push(value)};
        }
        return values.into_boxed_slice();
    }
    fn listchar(&mut self) -> Box<str> {
        let mut values = String::new();
        loop {
            let value = self.take8();
            if value == 0 {break} else {values.push(value as char)}
        }
        return values.into_boxed_str();
    }
    fn check(&self, distance: usize) -> () {
        if self.locus + distance > crate::SETTINGS.ir.len() {
            crate::stdout::crash(crate::stdout::Code::MalformedIR);
        }
    }
}