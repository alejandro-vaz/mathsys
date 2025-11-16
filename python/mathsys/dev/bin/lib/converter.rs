//^
//^ CLASS
//^

//> CLASS -> TRAIT
pub trait Class {
    fn name(&self) -> &'static str;
    fn evaluate(&self, context: &mut crate::runtime::Context) -> crate::Box<dyn crate::runtime::Value>;
    fn locale(&self, code: u8) -> ();
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
        crate::stdout::space("[CONVERTER] Processing IR");
        while self.locus < crate::SETTINGS.ir.len() {
            let object = match self.use8() {
                0x01 => self.start(),
                0x02 => self.declaration(),
                0x03 => self.definition(),
                0x04 => self.node(),
                0x05 => self.equation(),
                0x06 => self.comment(),
                0x07 => self.expression(),
                0x08 => self.term(),
                0x09 => self.factor(),
                0x0A => self.limit(),
                0x0B => self.infinite(),
                0x0C => self.variable(),
                0x0D => self.nest(),
                0x0E => self.vector(),
                0x0F => self.number(),
                other => crate::stdout::crash(crate::stdout::Code::UnknownIRObject)
            };
            self.memory.push(object);
        };
        return &self.memory;
    }
    fn comment(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Comment data structure");
        return crate::Box::new(crate::Comment::new(
            &self.listchar()
        ));
    }
    fn declaration(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Declaration data structure");
        return crate::Box::new(crate::Declaration::new(
            self.use32(),
            self.use32()
        ));
    }
    fn definition(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Definition data structure");
        return crate::Box::new(crate::Definition::new(
            self.use32(),
            self.use32()
        ));
    }
    fn equation(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Equation data structure");
        return crate::Box::new(crate::Equation::new(
            self.use32(), 
            self.use32()
        ));
    }
    fn expression(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Expression data structure");
        return crate::Box::new(crate::Expression::new(
            &self.list32(), 
            &self.list8()
        ));
    }
    fn factor(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Factor data structure");
        return crate::Box::new(crate::Factor::new(
            self.use32(), 
            self.use32()
        ));
    }
    fn infinite(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Infinite data structure");
        return crate::Box::new(crate::Infinite::new());
    }
    fn limit(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Limit data structure");
        return crate::Box::new(crate::Limit::new(
            self.use32(), 
            self.use32(), 
            self.use8(), 
            self.use32(), 
            self.use32()
        ));
    }
    fn nest(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Nest data structure");
        return crate::Box::new(crate::Nest::new(
            self.use32()
        ));
    }
    fn node(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Node data structure");
        return crate::Box::new(crate::Node::new(
            self.use32()
        ));
    }
    fn number(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Number data structure");
        return crate::Box::new(crate::Number::new(
            self.use32(),
            self.use8()
        ));
    }
    fn start(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Start data structure");
        return crate::Box::new(crate::Start::new(
            &self.list32()
        ));
    }
    fn term(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Term data structure");
        return crate::Box::new(crate::Term::new(
            &self.list32(), 
            &self.list32()
        ));
    }
    fn variable(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Variable data structure");
        return crate::Box::new(crate::Variable::new(
            &self.listchar()
        ));
    }
    fn vector(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating Vector data structure");
        return crate::Box::new(crate::Vector::new(
            &self.list32()
        ));
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
    fn use8(&mut self) -> u8 {
        self.check(1);
        let value = crate::SETTINGS.ir[self.locus];
        self.inc(1);
        return value;
    }
    fn use32(&mut self) -> u32 {
        self.check(4);
        let value = &crate::SETTINGS.ir[self.locus..self.locus + 4];
        self.inc(4);
        return u32::from_le_bytes([value[0], value[1], value[2], value[3]]);
    }
    fn list8(&mut self) -> crate::Vec::<u8> {
        let mut values = crate::Vec::<u8>::new();
        loop {match self.use8() {
            0 => break,
            value => values.push(value)
        }}
        return values;
    }
    fn list32(&mut self) -> crate::Vec::<u32> {
        let mut values = crate::Vec::<u32>::new();
        loop {match self.use32() {
            0 => break,
            value => values.push(value)
        }}
        return values;
    }
    fn listchar(&mut self) -> crate::String {
        let mut values = crate::String::new();
        loop {match self.use8() {
            0 => break,
            value => values.push(value as char)
        }}
        return values;
    }
    #[inline(always)]
    fn inc(&mut self, sum: usize) -> () {
        self.locus += sum;
    }
    #[inline(always)]
    fn check(&self, distance: usize) -> () {
        if self.locus + distance > crate::SETTINGS.ir.len() {
            crate::stdout::crash(crate::stdout::Code::MalformedIR);
        }
    }
}