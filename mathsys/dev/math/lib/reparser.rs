//^
//^ HEAD
//^

//> HEAD -> CRATES
use miniz_oxide::inflate::decompress_to_vec;

//> HEAD -> CROSS-SCOPE TRAIT
use crate::{_Absolute, _Annotation, _Comment, _Declaration, _Definition, _Equation, _Expression, _Factor, _Infinite, _Limit, _Nest, _Node, _Start, _Tensor, _Term, _Use, _Variable, _Whole};
use crate::class::Class;
use crate::stdout::{space, crash, Code, trace};
use crate::group::Group;
use crate::types::Pointer;


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
        let mut memory = Vec::new();
        let Ok(binary) = decompress_to_vec(ir) else {crash(Code::FailedIRDecompression)};
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
        text: self.listchar(binary)
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
        signs: self.list8(binary),
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
        direction: self.take8(binary),
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
        name: self.listchar(binary),
        start: self.takePointer(binary)
    }}
    fn Variable(&mut self, binary: &Vec<u8>) -> _Variable {return _Variable {
        representation: self.listchar(binary)
    }}
    fn Whole(&mut self, binary: &Vec<u8>) -> _Whole {return _Whole {
        value: self.take32(binary)
    }}
}

//> REPARSER -> METHODS
impl Reparser {
    pub fn new() -> Self {return Reparser { 
        locus: 0
    }}
    fn takeGroup(&mut self, binary: &Vec<u8>) -> Group {
        self.check(1, binary);
        let value = binary[self.locus];
        self.locus += 1;
        return Group::from(value);
    }
    fn take8(&mut self, binary: &Vec<u8>) -> u8 {
        self.check(1, binary);
        let value = binary[self.locus];
        self.locus += 1;
        return value;
    }
    fn takePointer(&mut self, binary: &Vec<u8>) -> Pointer {
        self.check(4, binary);
        let value = &binary[self.locus..self.locus + 4];
        self.locus += 4;
        return Pointer(u32::from_le_bytes([value[0], value[1], value[2], value[3]]));
    }
    fn take32(&mut self, binary: &Vec<u8>) -> u32 {
        self.check(4, binary);
        let value = &binary[self.locus..self.locus + 4];
        self.locus += 4;
        return u32::from_le_bytes([value[0], value[1], value[2], value[3]]);
    }
    fn list8(&mut self, binary: &Vec<u8>) -> Vec<u8> {
        let mut values = Vec::<u8>::new();
        loop {match self.take8(binary) {
            0 => break,
            other => values.push(other)
        }};
        return values;
    }
    fn listPointer(&mut self, binary: &Vec<u8>) -> Vec<Pointer> {
        let mut values = Vec::<Pointer>::new();
        loop {match self.takePointer(binary) {
            Pointer(0) => break,
            other => values.push(other),
        }};
        return values;
    }
    fn listchar(&mut self, binary: &Vec<u8>) -> String {
        let mut values = String::new();
        loop {match self.take8(binary) {
            0 => break,
            other => values.push(other as char)
        }}
        return values
    }
    fn check(&self, distance: usize, binary: &Vec<u8>) -> () {
        if self.locus + distance > binary.len() {
            crash(Code::UnexpectedEndOfIR);
        }
    }
}