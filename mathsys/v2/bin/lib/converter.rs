//
//  CONVERTER
//

// CONVERTER -> STRUCT
pub struct Converter {
    locus: usize,
    memory: crate::Vec<crate::Box <dyn crate::Object>>
}

// CONVERTER -> IMPLEMENTATION
impl Converter {
    pub fn run(&mut self) -> &crate::Vec<crate::Box <dyn crate::Object>> {
        loop {
            if self.locus == crate::SETTINGS.ir.len() {break}
            let value = self.use8();
            let location = self.use32();
            let object = match value {
                0x01 => self.start(),
                0x02 => self.debug(),
                0x03 => self.declaration(),
                0x04 => self.definition(),
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
                0x0F => self.vector(),
                0x10 => self.number(),
                _ => continue
            } as crate::Box<dyn crate::Object>;
            self.memory.push(object);
        };
        return &self.memory;
    }
    fn comment(&mut self) -> crate::Box<dyn crate::Object> {
        let mut characters = crate::String::new();
        loop {match self.use8() {
            0 => break,
            character => characters.push(character as char)
        }}
        return crate::Box::new(crate::Comment::new(&characters));
    }
    fn debug(&mut self) -> crate::Box<dyn crate::Object> {
        return crate::Box::new(crate::Debug::new());
    }
    fn declaration(&mut self) -> crate::Box<dyn crate::Object> {
        let variable = self.use32();
        let pointer = self.use32();
        return crate::Box::new(crate::Declaration::new(variable, pointer));
    }
    fn definition(&mut self) -> crate::Box<dyn crate::Object> {
        let variable = self.use32();
        let pointer = self.use32();
        return crate::Box::new(crate::Definition::new(variable, pointer));
    }
    fn equation(&mut self) -> crate::Box<dyn crate::Object> {
        let left = self.use32();
        let right = self.use32();
        return crate::Box::new(crate::Equation::new(left, right));
    }
    fn expression(&mut self) -> crate::Box<dyn crate::Object> {
        let mut terms = crate::Vec::<u32>::new();
        loop {match self.use32() {
            0 => break,
            term => terms.push(term)
        }}
        let mut signs = crate::Vec::<u8>::new();
        for index in 0..terms.len() {
            signs.push(self.use8())
        }
        return crate::Box::new(crate::Expression::new(&terms, &signs));
    }
    fn factor(&mut self) -> crate::Box<dyn crate::Object> {
        let pointer = self.use32();
        let expression = self.use32();
        return crate::Box::new(crate::Factor::new(pointer, expression));
    }
    fn infinite(&mut self) -> crate::Box<dyn crate::Object> {
        return crate::Box::new(crate::Infinite::new());
    }
    fn limit(&mut self) -> crate::Box<dyn crate::Object> {
        let variable = self.use32();
        let approach = self.use32();
        let direction = self.use8();
        let pointer = self.use32();
        let exponent = self.use32();
        return crate::Box::new(crate::Limit::new(variable, approach, direction, pointer, exponent));
    }
    fn nest(&mut self) -> crate::Box<dyn crate::Object> {
        let pointer = self.use32();
        return crate::Box::new(crate::Nest::new(pointer));
    }
    fn node(&mut self) -> crate::Box<dyn crate::Object> {
        let pointer = self.use32();
        return crate::Box::new(crate::Node::new(pointer));
    }
    fn number(&mut self) -> crate::Box<dyn crate::Object> {
        let value = self.use32();
        let shift = self.use8();
        return crate::Box::new(crate::Number::new(value, shift));
    }
    fn start(&mut self) -> crate::Box<dyn crate::Object> {
        let mut statements = crate::Vec::<u32>::new();
        loop {match self.use32() {
            0 => break,
            statement => statements.push(statement)
        }}
        return crate::Box::new(crate::Start::new(&statements));
    }
    fn term(&mut self) -> crate::Box<dyn crate::Object> {
        let mut numerator = crate::Vec::<u32>::new();
        loop {match self.use32() {
            0 => break,
            factor => numerator.push(factor)
        }}
        let mut denominator = crate::Vec::<u32>::new();
        loop {match self.use32() {
            0 => break,
            factor => denominator.push(factor)
        }}
        return crate::Box::new(crate::Term::new(&numerator, &denominator));
    }
    fn variable(&mut self) -> crate::Box<dyn crate::Object> {
        let mut characters = crate::String::new();
        loop {match self.use8() {
            0 => break,
            character => characters.push(character as char)
        }}
        return crate::Box::new(crate::Variable::new(&characters));
    }
    fn vector(&mut self) -> crate::Box<dyn crate::Object> {
        let mut values = crate::Vec::<u32>::new();
        loop {match self.use32() {
            0 => break,
            value => values.push(value)
        }}
        return crate::Box::new(crate::Vector::new(&values));
    }
}

// CONVERTER -> METHODS
impl Converter {
    pub fn new() -> Self {
        return Converter { 
            locus: 0,
            memory: crate::Vec::<crate::Box <dyn crate::Object>>::new()
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
    #[inline(always)]
    fn inc(&mut self, sum: usize) -> () {
        self.locus += sum;
    }
    #[inline(always)]
    fn check(&self, distance: usize) -> () {
        if self.locus + distance > crate::SETTINGS.ir.len() {
            crate::stdout::crash(2);
        }
    }
}