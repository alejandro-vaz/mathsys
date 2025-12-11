//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::class::Class;


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
    pub fn run(&mut self) -> &Vec<Class> {
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
                object
            ));
            self.memory.push(object);
        };
        return &self.memory;
    }
    fn Annotation(&mut self) -> Class {return Class::_Annotation(crate::_Annotation {
        group: self.take8(),
        variables: self.list32()
    })}
    fn Comment(&mut self) -> Class {return Class::_Comment(crate::_Comment {
        text: self.listchar()
    })}
    fn Declaration(&mut self) -> Class {return Class::_Declaration(crate::_Declaration {
        group: self.take8(),
        variable: self.take32(),
        expression: self.take32()
    })}
    fn Definition(&mut self) -> Class {return Class::_Definition(crate::_Definition {
        group: self.take8(),
        variable: self.take32(),
        expression: self.take32()
    })}
    fn Equation(&mut self) -> Class {return Class::_Equation(crate::_Equation {
        leftexpression: self.take32(),
        rightexpression: self.take32()
    })}
    fn Expression(&mut self) -> Class {return Class::_Expression(crate::_Expression {
        signs: self.list8(),
        terms: self.list32()
    })}
    fn Factor(&mut self) -> Class {return Class::_Factor(crate::_Factor {
        value: self.take32(),
        exponent: self.take32()
    })}
    fn Infinite(&mut self) -> Class {return Class::_Infinite(crate::_Infinite {})}
    fn Limit(&mut self) -> Class {return Class::_Limit(crate::_Limit {
        variable: self.take32(),
        approach: self.take32(),
        direction: self.take8(),
        nest: self.take32(),
        exponent: self.take32()
    })}
    fn Nest(&mut self) -> Class {return Class::_Nest(crate::_Nest {
        expression: self.take32()
    })}
    fn Node(&mut self) -> Class {return Class::_Node(crate::_Node {
        expression: self.take32()
    })}
    fn Number(&mut self) -> Class {return Class::_Number(crate::_Number {
        value: self.take32(),
        shift: self.take8()
    })}
    fn Start(&mut self) -> Class {return Class::_Start(crate::_Start {
        statements: self.list32()
    })}
    fn Tensor(&mut self) -> Class {return Class::_Tensor(crate::_Tensor {
        values: self.list32()
    })}
    fn Term(&mut self) -> Class {return Class::_Term(crate::_Term {
        numerator: self.list32(),
        denominator: self.list32()
    })}
    fn Use(&mut self) -> Class {return Class::_Use(crate::_Use {
        name: self.listchar(),
        start: self.take32()
    })}
    fn Variable(&mut self) -> Class {return Class::_Variable(crate::_Variable {
        representation: self.listchar()
    })}
}

//> REPARSER -> METHODS
impl Reparser {
    pub fn new() -> Self {return Reparser { 
        locus: 0,
        memory: Vec::<Class>::new()
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
    fn list8(&mut self) -> Vec<u8> {
        let mut values = Vec::<u8>::new();
        loop {match self.take8() {
            0 => break,
            other => values.push(other)
        }};
        return values
    }
    fn list32(&mut self) -> Vec<u32> {
        let mut values = Vec::<u32>::new();
        loop {match self.take32() {
            0 => break,
            other => values.push(other)
        }};
        return values
    }
    fn listchar(&mut self) -> String {
        let mut values = String::new();
        loop {match self.take8() {
            0 => break,
            other => values.push(other as char)
        }}
        return values
    }
    fn check(&self, distance: usize) -> () {
        if self.locus + distance > crate::SETTINGS.ir.len() {
            crate::stdout::crash(crate::stdout::Code::MalformedIR);
        }
    }
}