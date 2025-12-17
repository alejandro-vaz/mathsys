//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;
use crate::Settings;
use crate::stdout::{space, crash, Code, trace};


//^
//^ REPARSER
//^

//> REPARSER -> STRUCT
pub struct Reparser {
    locus: usize,
    memory: Vec<Class>
}

//> REPARSER -> IMPLEMENTATION
impl Reparser {
    pub fn run(&mut self, settings: &Settings) -> &Vec<Class> {
        space("{REPARSER} Processing IR");
        while self.locus < settings.ir.len() {
            let object = match self.take8(settings) {
                0x01 => self.Start(settings),
                0x02 => self.Declaration(settings),
                0x03 => self.Definition(settings),
                0x04 => self.Annotation(settings),
                0x05 => self.Node(settings),
                0x06 => self.Equation(settings),
                0x07 => self.Comment(settings),
                0x08 => self.Use(settings),
                0x09 => self.Expression(settings),
                0x0A => self.Term(settings),
                0x0B => self.Factor(settings),
                0x0C => self.Limit(settings),
                0x0D => self.Infinite(settings),
                0x0E => self.Variable(settings),
                0x0F => self.Nest(settings),
                0x10 => self.Tensor(settings),
                0x11 => self.Natural(settings),
                other => crash(Code::UnknownIRObject)
            };
            trace(format!(
                "Creating {} data structure",
                object
            ));
            self.memory.push(object);
        };
        return &self.memory;
    }
    fn Annotation(&mut self, settings: &Settings) -> Class {return Class::_Annotation(crate::_Annotation {
        group: self.take8(settings),
        variables: self.list32(settings)
    })}
    fn Comment(&mut self, settings: &Settings) -> Class {return Class::_Comment(crate::_Comment {
        text: self.listchar(settings)
    })}
    fn Declaration(&mut self, settings: &Settings) -> Class {return Class::_Declaration(crate::_Declaration {
        group: self.take8(settings),
        variable: self.take32(settings),
        expression: self.take32(settings)
    })}
    fn Definition(&mut self, settings: &Settings) -> Class {return Class::_Definition(crate::_Definition {
        group: self.take8(settings),
        variable: self.take32(settings),
        expression: self.take32(settings)
    })}
    fn Equation(&mut self, settings: &Settings) -> Class {return Class::_Equation(crate::_Equation {
        leftexpression: self.take32(settings),
        rightexpression: self.take32(settings)
    })}
    fn Expression(&mut self, settings: &Settings) -> Class {return Class::_Expression(crate::_Expression {
        signs: self.list8(settings),
        terms: self.list32(settings)
    })}
    fn Factor(&mut self, settings: &Settings) -> Class {return Class::_Factor(crate::_Factor {
        value: self.take32(settings),
        exponent: self.take32(settings)
    })}
    fn Infinite(&mut self, settings: &Settings) -> Class {return Class::_Infinite(crate::_Infinite {})}
    fn Limit(&mut self, settings: &Settings) -> Class {return Class::_Limit(crate::_Limit {
        variable: self.take32(settings),
        approach: self.take32(settings),
        direction: self.take8(settings),
        nest: self.take32(settings),
        exponent: self.take32(settings)
    })}
    fn Natural(&mut self, settings: &Settings) -> Class {return Class::_Natural(crate::_Natural {
        value: self.take32(settings)
    })}
    fn Nest(&mut self, settings: &Settings) -> Class {return Class::_Nest(crate::_Nest {
        expression: self.take32(settings)
    })}
    fn Node(&mut self, settings: &Settings) -> Class {return Class::_Node(crate::_Node {
        expression: self.take32(settings)
    })}
    fn Start(&mut self, settings: &Settings) -> Class {return Class::_Start(crate::_Start {
        statements: self.list32(settings)
    })}
    fn Tensor(&mut self, settings: &Settings) -> Class {return Class::_Tensor(crate::_Tensor {
        values: self.list32(settings)
    })}
    fn Term(&mut self, settings: &Settings) -> Class {return Class::_Term(crate::_Term {
        numerator: self.list32(settings),
        denominator: self.list32(settings)
    })}
    fn Use(&mut self, settings: &Settings) -> Class {return Class::_Use(crate::_Use {
        name: self.listchar(settings),
        start: self.take32(settings)
    })}
    fn Variable(&mut self, settings: &Settings) -> Class {return Class::_Variable(crate::_Variable {
        representation: self.listchar(settings)
    })}
}

//> REPARSER -> METHODS
impl Reparser {
    pub fn new() -> Self {return Reparser { 
        locus: 0,
        memory: Vec::<Class>::new()
    }}
    fn take8(&mut self, settings: &Settings) -> u8 {
        self.check(1, settings);
        let value = settings.ir[self.locus];
        self.locus += 1;
        return value;
    }
    fn take32(&mut self, settings: &Settings) -> u32 {
        self.check(4, settings);
        let value = &settings.ir[self.locus..self.locus + 4];
        self.locus += 4;
        return u32::from_le_bytes([value[0], value[1], value[2], value[3]]);
    }
    fn list8(&mut self, settings: &Settings) -> Vec<u8> {
        let mut values = Vec::<u8>::new();
        loop {match self.take8(settings) {
            0 => break,
            other => values.push(other)
        }};
        return values
    }
    fn list32(&mut self, settings: &Settings) -> Vec<u32> {
        let mut values = Vec::<u32>::new();
        loop {match self.take32(settings) {
            0 => break,
            other => values.push(other)
        }};
        return values
    }
    fn listchar(&mut self, settings: &Settings) -> String {
        let mut values = String::new();
        loop {match self.take8(settings) {
            0 => break,
            other => values.push(other as char)
        }}
        return values
    }
    fn check(&self, distance: usize, settings: &Settings) -> () {
        if self.locus + distance > settings.ir.len() {
            crash(Code::MalformedIR);
        }
    }
}