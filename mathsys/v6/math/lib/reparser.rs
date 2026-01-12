//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Class, decompress, space, crash, Code, trace, _Start, _Declaration, _Definition, _Annotation, _Node, _Equation, _Use, _Expression, _Term, _Factor, _Limit, _Infinite, _Variable, _Nest, _Tensor, _Whole, _Absolute, _Undefined, _Rational, _Casts, Group, Sign, Pointer, BigUint, BitVec, BitSlice, Lsb0, Opcode, BitField, class
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
        trace("Decompressing IR");
        let Ok(bytes) = decompress(ir) else {crash(Code::FailedIRDecompression)};
        let binary = BitVec::<u8, Lsb0>::from_vec(bytes);
        trace(format!(
            "Parsing {} data bits",
            binary.len()
        ));
        let mut counter = 0;
        while self.locus < binary.len() {
            let object = match self.takeOpcode(&binary) {
                Opcode::Continue => continue,
                Opcode::Start => Class::_Start(self.Start(&binary)),
                Opcode::Declaration => Class::_Declaration(self.Declaration(&binary)),
                Opcode::Definition => Class::_Definition(self.Definition(&binary)),
                Opcode::Annotation => Class::_Annotation(self.Annotation(&binary)),
                Opcode::Node => Class::_Node(self.Node(&binary)),
                Opcode::Equation => Class::_Equation(self.Equation(&binary)),
                Opcode::Use => Class::_Use(self.Use(&binary)),
                Opcode::Expression => Class::_Expression(self.Expression(&binary)),
                Opcode::Term => Class::_Term(self.Term(&binary)),
                Opcode::Factor => Class::_Factor(self.Factor(&binary)),
                Opcode::Limit => Class::_Limit(self.Limit(&binary)),
                Opcode::Infinite => Class::_Infinite(self.Infinite(&binary)),
                Opcode::Variable => Class::_Variable(self.Variable(&binary)),
                Opcode::Nest => Class::_Nest(self.Nest(&binary)),
                Opcode::Tensor => Class::_Tensor(self.Tensor(&binary)),
                Opcode::Whole => Class::_Whole(self.Whole(&binary)),
                Opcode::Absolute => Class::_Absolute(self.Absolute(&binary)),
                Opcode::Undefined => Class::_Undefined(self.Undefined(&binary)),
                Opcode::Rational => Class::_Rational(self.Rational(&binary)),
                Opcode::Casts => Class::_Casts(self.Casts(&binary))
            };
            class(format!("{counter}{object:?}"));
            counter += 1;
            memory.push(object);
        };
        return memory
    }
    fn Absolute(&mut self, binary: &BitVec<u8, Lsb0>) -> _Absolute {return _Absolute {
        value: self.takePointer(binary)
    }}
    fn Annotation(&mut self, binary: &BitVec<u8, Lsb0>) -> _Annotation {return _Annotation {
        group: self.takeGroup(binary),
        variables: self.takeVec(binary, |self1, binary1| self1.takePointer(binary1))
    }}
    fn Declaration(&mut self, binary: &BitVec<u8, Lsb0>) -> _Declaration {return _Declaration {
        group: self.takeOption(binary, |self1, binary1| self1.takeGroup(binary1)),
        variable: self.takePointer(binary),
        value: self.takePointer(binary)
    }}
    fn Definition(&mut self, binary: &BitVec<u8, Lsb0>) -> _Definition {return _Definition {
        group: self.takeOption(binary, |self1, binary1| self1.takeGroup(binary1)),
        variable: self.takePointer(binary),
        value: self.takePointer(binary)
    }}
    fn Equation(&mut self, binary: &BitVec<u8, Lsb0>) -> _Equation {return _Equation {
        leftside: self.takePointer(binary),
        rightside: self.takePointer(binary)
    }}
    fn Expression(&mut self, binary: &BitVec<u8, Lsb0>) -> _Expression {return _Expression {
        signs: self.takeVec(binary, |self1, binary1| self1.takeVec(binary1, |self2, binary2| self2.takeSign(binary2))),
        terms: self.takeVec(binary, |self1, binary1| self1.takePointer(binary1))
    }}
    fn Factor(&mut self, binary: &BitVec<u8, Lsb0>) -> _Factor {return _Factor {
        value: self.takePointer(binary),
        exponent: self.takeOption(binary, |self1, binary1| self1.takePointer(binary1))
    }}
    fn Infinite(&mut self, binary: &BitVec<u8, Lsb0>) -> _Infinite {return _Infinite {}}
    fn Limit(&mut self, binary: &BitVec<u8, Lsb0>) -> _Limit {return _Limit {
        variable: self.takePointer(binary),
        approach: self.takePointer(binary),
        direction: self.takeOption(binary, |self1, binary1| self1.takeSign(binary1)),
        nest: self.takePointer(binary),
        exponent: self.takeOption(binary, |self1, binary1| self1.takePointer(binary1))
    }}
    fn Nest(&mut self, binary: &BitVec<u8, Lsb0>) -> _Nest {return _Nest {
        value: self.takeOption(binary, |self1, binary1| self1.takePointer(binary1))
    }}
    fn Node(&mut self, binary: &BitVec<u8, Lsb0>) -> _Node {return _Node {
        value: self.takePointer(binary)
    }}
    fn Rational(&mut self, binary: &BitVec<u8, Lsb0>) -> _Rational {return _Rational {
        whole: self.takeBigUint(binary),
        decimal: self.takeBigUint(binary)
    }}
    fn Start(&mut self, binary: &BitVec<u8, Lsb0>) -> _Start {return _Start {
        stream: self.takeVec(binary, |self1, binary1| self1.takePointer(binary1))
    }}
    fn Tensor(&mut self, binary: &BitVec<u8, Lsb0>) -> _Tensor {return _Tensor {
        values: self.takeVec(binary, |self1, binary1| self1.takePointer(binary1))
    }}
    fn Term(&mut self, binary: &BitVec<u8, Lsb0>) -> _Term {return _Term {
        numerator: self.takeVec(binary, |self1, binary1| self1.takePointer(binary1)),
        denominator: self.takeVec(binary, |self1, binary1| self1.takePointer(binary1))
    }}
    fn Undefined(&mut self, binary: &BitVec<u8, Lsb0>) -> _Undefined {return _Undefined {}}
    fn Use(&mut self, binary: &BitVec<u8, Lsb0>) -> _Use {return _Use {
        name: self.takeString(binary),
        start: self.takeOption(binary, |self1, binary1| self1.takePointer(binary1))
    }}
    fn Variable(&mut self, binary: &BitVec<u8, Lsb0>) -> _Variable {return _Variable {
        representation: self.takeString(binary)
    }}
    fn Whole(&mut self, binary: &BitVec<u8, Lsb0>) -> _Whole {return _Whole {
        value: self.takeBigUint(binary)
    }}
    fn Casts(&mut self, binary: &BitVec<u8, Lsb0>) -> _Casts {return _Casts {
        element: self.takePointer(binary),
        to: self.takeGroup(binary)
    }}
}

//> REPARSER -> METHODS
impl Reparser {
    //~ METHODS -> DEFAULT
    pub fn new() -> Self {return Reparser {locus: 0}}
    fn check(&self, distance: usize, binary: &BitVec<u8, Lsb0>) -> () {if self.locus + distance > binary.len() {crash(Code::UnexpectedEndOfIR)}}
    fn take<'bin>(&mut self, amount: usize, binary: &'bin BitVec<u8, Lsb0>) -> &'bin BitSlice<u8, Lsb0> {
        self.check(amount, binary);
        let slice = &binary[self.locus..self.locus + amount];
        self.locus += amount;
        return slice;
    }
    //~ METHODS -> ITEMS
    fn takeOpcode(&mut self, binary: &BitVec<u8, Lsb0>) -> Opcode {return self.take(5, binary).load_le::<u8>().into()}
    fn takePointer(&mut self, binary: &BitVec<u8, Lsb0>) -> Pointer {return self.take(32, binary).load_le::<u32>().into()}
    fn takeSign(&mut self, binary: &BitVec<u8, Lsb0>) -> Sign {return self.take(1, binary)[0].into()}
    fn takeOption<Type, Function>(&mut self, binary: &BitVec<u8, Lsb0>, parser: Function) -> Option<Type> where Function: Fn(&mut Self, &BitVec<u8, Lsb0>) -> Type {
        return if self.take(1, binary)[0] {Some(parser(self, binary))} else {None}
    }
    fn takeBigUint(&mut self, binary: &BitVec<u8, Lsb0>) -> BigUint {return self.take(128, binary).load_le::<u128>().into()}
    fn takeString(&mut self, binary: &BitVec<u8, Lsb0>) -> String {return (0..self.take(16, binary).load_le::<u16>()).map(|index| self.take(8, binary).load_le::<u8>() as char).collect()}
    fn takeGroup(&mut self, binary: &BitVec<u8, Lsb0>) -> Group {return self.take(4, binary).load_le::<u8>().into()}
    fn takeVec<Type, Function>(&mut self, binary: &BitVec<u8, Lsb0>, parser: Function) -> Vec<Type> where Function: Fn(&mut Self, &BitVec<u8, Lsb0>) -> Type {
        return (0..self.take(32, binary).load_le::<u32>()).map(|index| parser(self, binary)).collect();
    }
}