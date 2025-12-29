//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Class,
    decompress,
    space,
    crash,
    Code,
    trace,
    _Start,
    _Declaration,
    _Definition,
    _Annotation,
    _Node,
    _Equation,
    _Comment,
    _Use,
    _Expression,
    _Term,
    _Factor,
    _Limit,
    _Infinite,
    _Variable,
    _Nest,
    _Tensor,
    _Whole,
    _Absolute,
    Group,
    Sign,
    Pointer,
    BigUint
};


//^
//^ REPARSER
//^

//> REPARSER -> STRUCT
pub struct Reparser {
    locus: usize,
}

//> REPARSER -> IMPLEMENTATION
impl Reparser {
    pub fn run(&mut self, ir: &'static [u8]) -> Vec<Class> {
        space("{REPARSER} Processing IR");
        let mut memory = Vec::with_capacity(ir.len());
        let Ok(binary) = decompress(ir) else {crash(Code::FailedIRDecompression)};
        while self.locus < binary.len() {
            let object = match self.take8(&binary) {
                0x01 => Class::_Start(self.Start(&binary)),
                0x02 => Class::_Declaration(self.Declaration(&binary)),
                0x03 => Class::_Definition(self.Definition(&binary)),
                0x04 => Class::_Annotation(self.Annotation(&binary)),
                0x05 => Class::_Node(self.Node(&binary)),
                0x06 => Class::_Equation(self.Equation(&binary)),
                0x07 => Class::_Comment(self.Comment(&binary)),
                0x08 => Class::_Use(self.Use(&binary)),
                0x09 => Class::_Expression(self.Expression(&binary)),
                0x0A => Class::_Term(self.Term(&binary)),
                0x0B => Class::_Factor(self.Factor(&binary)),
                0x0C => Class::_Limit(self.Limit(&binary)),
                0x0D => Class::_Infinite(self.Infinite(&binary)),
                0x0E => Class::_Variable(self.Variable(&binary)),
                0x0F => Class::_Nest(self.Nest(&binary)),
                0x10 => Class::_Tensor(self.Tensor(&binary)),
                0x11 => Class::_Whole(self.Whole(&binary)),
                0x12 => Class::_Absolute(self.Absolute(&binary)),
                other => crash(Code::UnknownIRObject)
            };
            trace(format!(
                "Creating {} data structure",
                object
            ));
            memory.push(object);
        };
        return memory
    }
    fn Absolute(&mut self, binary: &Vec<u8>) -> _Absolute {return _Absolute {
        expression: self.takePointer(binary)
    }}
    fn Annotation(&mut self, binary: &Vec<u8>) -> _Annotation {return _Annotation {
        group: self.takeGroup(binary),
        variables: self.listPointer(binary)
    }}
    fn Comment(&mut self, binary: &Vec<u8>) -> _Comment {return _Comment {
        text: self.listString(binary)
    }}
    fn Declaration(&mut self, binary: &Vec<u8>) -> _Declaration {return _Declaration {
        group: self.takeGroup(binary),
        variable: self.takePointer(binary),
        expression: self.takePointer(binary)
    }}
    fn Definition(&mut self, binary: &Vec<u8>) -> _Definition {return _Definition {
        group: self.takeGroup(binary),
        variable: self.takePointer(binary),
        expression: self.takePointer(binary)
    }}
    fn Equation(&mut self, binary: &Vec<u8>) -> _Equation {_Equation {
        leftexpression: self.takePointer(binary),
        rightexpression: self.takePointer(binary)
    }}
    fn Expression(&mut self, binary: &Vec<u8>) -> _Expression {return _Expression {
        signs: self.listSign(binary),
        terms: self.listPointer(binary)
    }}
    fn Factor(&mut self, binary: &Vec<u8>) -> _Factor {return _Factor {
        value: self.takePointer(binary),
        exponent: self.takePointer(binary)
    }}
    fn Infinite(&mut self, binary: &Vec<u8>) -> _Infinite {return _Infinite {}}
    fn Limit(&mut self, binary: &Vec<u8>) -> _Limit {return _Limit {
        variable: self.takePointer(binary),
        approach: self.takePointer(binary),
        direction: self.takeSign(binary),
        nest: self.takePointer(binary),
        exponent: self.takePointer(binary)
    }}
    fn Nest(&mut self, binary: &Vec<u8>) -> _Nest {return _Nest {
        expression: self.takePointer(binary)
    }}
    fn Node(&mut self, binary: &Vec<u8>) -> _Node {return _Node {
        expression: self.takePointer(binary)
    }}
    fn Start(&mut self, binary: &Vec<u8>) -> _Start {return _Start {
        statements: self.listPointer(binary)
    }}
    fn Tensor(&mut self, binary: &Vec<u8>) -> _Tensor {return _Tensor {
        values: self.listPointer(binary)
    }}
    fn Term(&mut self, binary: &Vec<u8>) -> _Term {return _Term {
        numerator: self.listPointer(binary),
        denominator: self.listPointer(binary)
    }}
    fn Use(&mut self, binary: &Vec<u8>) -> _Use {return _Use {
        name: self.listString(binary),
        start: self.takePointer(binary)
    }}
    fn Variable(&mut self, binary: &Vec<u8>) -> _Variable {return _Variable {
        representation: self.listString(binary)
    }}
    fn Whole(&mut self, binary: &Vec<u8>) -> _Whole {return _Whole {
        value: self.takeBigUint(binary)
    }}
}

//> REPARSER -> METHODS
impl Reparser {
    //~ METHODS -> 8 INDIVIDUAL
    pub fn new() -> Self {return Reparser {locus: 0}}
    fn take8(&mut self, binary: &Vec<u8>) -> u8 {
        self.check(1, binary);
        let value = binary[self.locus];
        self.locus += 1;
        return value;
    }
    fn takeGroup(&mut self, binary: &Vec<u8>) -> Group {return self.take8(binary).into()}
    fn takeSign(&mut self, binary: &Vec<u8>) -> Sign {return self.take8(binary).into()}
    //~ METHODS -> 8 LIST
    fn listSign(&mut self, binary: &Vec<u8>) -> Vec<Sign> {
        let mut values = Vec::<Sign>::new();
        loop {match self.take8(binary) {
            0 => break,
            other => values.push(other.into()),
        }};
        return values;
    }
    fn listString(&mut self, binary: &Vec<u8>) -> String {
        let mut values = String::new();
        loop {match self.take8(binary) {
            0 => break,
            other => values.push(other as char)
        }}
        return values
    }
    //~ 32 INDIVIDUAL
    fn take32(&mut self, binary: &Vec<u8>) -> u32 {
        self.check(4, binary);
        let value = &binary[self.locus..self.locus + 4];
        self.locus += 4;
        return u32::from_le_bytes([value[0], value[1], value[2], value[3]]);
    }
    fn takePointer(&mut self, binary: &Vec<u8>) -> Pointer {return self.take32(binary).into()}
    fn takeBigUint(&mut self, binary: &Vec<u8>) -> BigUint {return self.take32(binary).into()}
    //~ 32 LIST
    fn listPointer(&mut self, binary: &Vec<u8>) -> Vec<Pointer> {
        let mut values = Vec::<Pointer>::new();
        loop {match self.take32(binary) {
            0 => break,
            other => values.push(other.into())
        }}
        return values;
    }
    fn check(&self, distance: usize, binary: &Vec<u8>) -> () {
        if self.locus + distance > binary.len() {
            crash(Code::UnexpectedEndOfIR);
        }
    }
}