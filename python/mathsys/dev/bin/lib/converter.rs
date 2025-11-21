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
                0x04 => self.annotation(),
                0x05 => self.node(),
                0x06 => self.equation(),
                0x07 => self.comment(),
                0x08 => self.expression(),
                0x09 => self.term(),
                0x0A => self.factor(),
                0x0B => self.limit(),
                0x0C => self.infinite(),
                0x0D => self.variable(),
                0x0E => self.nest(),
                0x0F => self.tensor(),
                0x10 => self.number(),
                other => crate::stdout::crash(crate::stdout::Code::UnknownIRObject)
            };
            self.memory.push(object);
        };
        return &self.memory;
    }
    fn annotation(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Annotation data structure");
        return crate::Box::new(crate::_Annotation::new(
            self.use8(),
            self.use32()
        ));
    }
    fn comment(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Comment data structure");
        return crate::Box::new(crate::_Comment::new(
            &self.listchar()
        ));
    }
    fn declaration(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Declaration data structure");
        return crate::Box::new(crate::_Declaration::new(
            self.use8(),
            self.use32(),
            self.use32()
        ));
    }
    fn definition(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Definition data structure");
        return crate::Box::new(crate::_Definition::new(
            self.use8(),
            self.use32(),
            self.use32()
        ));
    }
    fn equation(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Equation data structure");
        return crate::Box::new(crate::_Equation::new(
            self.use32(), 
            self.use32()
        ));
    }
    fn expression(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Expression data structure");
        return crate::Box::new(crate::_Expression::new(
            &self.list32(), 
            &self.list8()
        ));
    }
    fn factor(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Factor data structure");
        return crate::Box::new(crate::_Factor::new(
            self.use32(), 
            self.use32()
        ));
    }
    fn infinite(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Infinite data structure");
        return crate::Box::new(crate::_Infinite::new());
    }
    fn limit(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Limit data structure");
        return crate::Box::new(crate::_Limit::new(
            self.use32(), 
            self.use32(), 
            self.use8(), 
            self.use32(), 
            self.use32()
        ));
    }
    fn nest(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Nest data structure");
        return crate::Box::new(crate::_Nest::new(
            self.use32()
        ));
    }
    fn node(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Node data structure");
        return crate::Box::new(crate::_Node::new(
            self.use32()
        ));
    }
    fn number(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Number data structure");
        return crate::Box::new(crate::_Number::new(
            self.use32(),
            self.use8()
        ));
    }
    fn start(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Start data structure");
        return crate::Box::new(crate::_Start::new(
            &self.list32()
        ));
    }
    fn tensor(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Tensor data structure");
        return crate::Box::new(crate::_Tensor::new(
            &self.list32()
        ));
    }
    fn term(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Term data structure");
        return crate::Box::new(crate::_Term::new(
            &self.list32(), 
            &self.list32()
        ));
    }
    fn variable(&mut self) -> crate::Box<dyn Class> {
        crate::stdout::trace("Creating _Variable data structure");
        return crate::Box::new(crate::_Variable::new(
            &self.listchar()
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