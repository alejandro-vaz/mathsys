//^
//^ CLASS
//^

//> CLASS -> TRAIT
pub trait Class {
    fn name(&self) -> &'static str;
    fn space(&self, message: &str, id: u32) -> () {crate::stdout::space(&crate::format!(
        "{{{}{}}} {}",
        id,
        self.name(),
        message
    )); self.info()}
    fn info(&self) -> ();
    fn evaluate(&self, context: &mut crate::runtime::Context, id: u32) -> crate::Box<dyn crate::runtime::Value>;
}


//^
//^ CONVERTER
//^

//> CONVERTER -> STRUCT
pub struct Converter {
    locus: usize,
    memory: crate::Vec<crate::Box <dyn Class>>
}

//> CONVERTER -> IMPLEMENTATION
impl Converter {
    pub fn run(&mut self) -> &crate::Vec<crate::Box <dyn Class>> {
        crate::stdout::space("{CONVERTER} Processing IR");
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
            self.memory.push(object);
        };
        return &self.memory;
    }
    fn Annotation(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Annotation data structure");
        return crate::Box::new(crate::_Annotation {
            object: self.take8(),
            variable: self.take32()
        })
    }
    fn Comment(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Comment data structure");
        return crate::Box::new(crate::_Comment {
            characters: (&*self.listchar()).into()
        })
    }
    fn Declaration(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Declaration data structure");
        return crate::Box::new(crate::_Declaration {
            object: self.take8(),
            variable: self.take32(),
            pointer: self.take32()
        });
    }
    fn Definition(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Definition data structure");
        return crate::Box::new(crate::_Definition {
            object: self.take8(),
            variable: self.take32(),
            pointer: self.take32()
        });
    }
    fn Equation(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Equation data structure");
        return crate::Box::new(crate::_Equation {
            left: self.take32(),
            right: self.take32()
        });
    }
    fn Expression(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Expression data structure");
        return crate::Box::new(crate::_Expression {
            terms: (&*self.list32()).into(),
            signs: (&*self.list8()).into()
        });
    }
    fn Factor(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Factor data structure");
        return crate::Box::new(crate::_Factor {
            pointer: self.take32(),
            expression: self.take32()
        });
    }
    fn Infinite(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Infinite data structure");
        return crate::Box::new(crate::_Infinite {});
    }
    fn Limit(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Limit data structure");
        return crate::Box::new(crate::_Limit {
            variable: self.take32(),
            approach: self.take32(),
            direction: self.take8(),
            pointer: self.take32(),
            exponent: self.take32()
        });
    }
    fn Nest(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Nest data structure");
        return crate::Box::new(crate::_Nest {
            pointer: self.take32()
        });
    }
    fn Node(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Node data structure");
        return crate::Box::new(crate::_Node {
            pointer: self.take32()
        });
    }
    fn Number(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Number data structure");
        return crate::Box::new(crate::_Number {
            value: self.take32(),
            shift: self.take8()
        });
    }
    fn Start(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Start data structure");
        return crate::Box::new(crate::_Start {
            statements: (&*self.list32()).into()
        });
    }
    fn Tensor(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Tensor data structure");
        return crate::Box::new(crate::_Tensor {
            values: (&*self.list32()).into()
        });
    }
    fn Term(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Term data structure");
        return crate::Box::new(crate::_Term {
            numerator: (&*self.list32()).into(),
            denominator: (&*self.list32()).into()
        });
    }
    fn Use(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Use data structure");
        return crate::Box::new(crate::_Use {
            name: (&*self.listchar()).into(),
            module: self.take32()
        });
    }
    fn Variable(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Variable data structure");
        return crate::Box::new(crate::_Variable {
            characters: (&*self.listchar()).into()
        });
    }
}

//> CONVERTER -> METHODS
impl Converter {
    pub fn new() -> Self {
        return Converter { 
            locus: 0,
            memory: crate::Vec::<crate::Box <dyn Class>>::new()
        }
    }
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
    fn list8(&mut self) -> crate::Vec<u8> {
        let mut values = crate::Vec::<u8>::new();
        loop {match self.take8() {
            0 => break,
            value => values.push(value)
        }}
        return values;
    }
    fn list32(&mut self) -> crate::Vec<u32> {
        let mut values = crate::Vec::<u32>::new();
        loop {match self.take32() {
            0 => break,
            value => values.push(value)
        }}
        return values;
    }
    fn listchar(&mut self) -> crate::String {
        let mut values = crate::String::new();
        loop {match self.take8() {
            0 => break,
            value => values.push(value as char)
        }}
        return values;
    }
    fn check(&self, distance: usize) -> () {
        if self.locus + distance > crate::SETTINGS.ir.len() {
            crate::stdout::crash(crate::stdout::Code::MalformedIR);
        }
    }
}