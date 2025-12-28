//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::{_Absolute, _Annotation, _Comment, _Declaration, _Definition, _Equation, _Expression, _Factor, _Infinite, _Limit, _Nest, _Node, _Start, _Tensor, _Term, _Use, _Variable, _Whole};
use crate::runtime::Runtime;
use crate::object::Object;
use crate::stdout::class;
use crate::types::Pointer;


//^
//^ CLASS
//^

//> CLASS -> ENUM
#[derive(Clone)]
pub enum Class {
    _Absolute(_Absolute),
    _Annotation(_Annotation),
    _Comment(_Comment),
    _Declaration(_Declaration),
    _Definition(_Definition),
    _Equation(_Equation),
    _Expression(_Expression),
    _Factor(_Factor),
    _Infinite(_Infinite),
    _Limit(_Limit),
    _Nest(_Nest),
    _Node(_Node),
    _Start(_Start),
    _Tensor(_Tensor),
    _Term(_Term),
    _Use(_Use),
    _Variable(_Variable),
    _Whole(_Whole)
}

//> CLASS -> EVALUATE
impl Class {
    pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {return self.result(match self {
        Class::_Absolute(class) => class.evaluate(runtime, id, memory),
        Class::_Annotation(class) => class.evaluate(runtime, id, memory),
        Class::_Comment(class) => class.evaluate(runtime, id, memory),
        Class::_Declaration(class) => class.evaluate(runtime, id, memory),
        Class::_Definition(class) => class.evaluate(runtime, id, memory),
        Class::_Equation(class) => class.evaluate(runtime, id, memory),
        Class::_Expression(class) => class.evaluate(runtime, id, memory),
        Class::_Factor(class) => class.evaluate(runtime, id, memory),
        Class::_Infinite(class) => class.evaluate(runtime, id, memory),
        Class::_Limit(class) => class.evaluate(runtime, id, memory),
        Class::_Nest(class) => class.evaluate(runtime, id, memory),
        Class::_Node(class) => class.evaluate(runtime, id, memory),
        Class::_Start(class) => class.evaluate(runtime, id, memory),
        Class::_Tensor(class) => class.evaluate(runtime, id, memory),
        Class::_Term(class) => class.evaluate(runtime, id, memory),
        Class::_Use(class) => class.evaluate(runtime, id, memory),
        Class::_Variable(class) => class.evaluate(runtime, id, memory),
        Class::_Whole(class) => class.evaluate(runtime, id, memory)
    })}
}

//> CLASS -> REPRESENTATION
impl crate::Display for Class {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {return match self {
    Class::_Absolute(class) => class.display(formatter),
    Class::_Annotation(class) => class.display(formatter),
    Class::_Comment(class) => class.display(formatter),
    Class::_Declaration(class) => class.display(formatter),
    Class::_Definition(class) => class.display(formatter),
    Class::_Equation(class) => class.display(formatter),
    Class::_Expression(class) => class.display(formatter),
    Class::_Factor(class) => class.display(formatter),
    Class::_Infinite(class) => class.display(formatter),
    Class::_Limit(class) => class.display(formatter),
    Class::_Nest(class) => class.display(formatter),
    Class::_Node(class) => class.display(formatter),
    Class::_Start(class) => class.display(formatter),
    Class::_Tensor(class) => class.display(formatter),
    Class::_Term(class) => class.display(formatter),
    Class::_Use(class) => class.display(formatter),
    Class::_Variable(class) => class.display(formatter),
    Class::_Whole(class) => class.display(formatter)
}}} impl crate::Debug for Class {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {return match self {
    Class::_Absolute(class) => class.debug(formatter),
    Class::_Annotation(class) => class.debug(formatter),
    Class::_Comment(class) => class.debug(formatter),
    Class::_Declaration(class) => class.debug(formatter),
    Class::_Definition(class) => class.debug(formatter),
    Class::_Equation(class) => class.debug(formatter),
    Class::_Expression(class) => class.debug(formatter),
    Class::_Factor(class) => class.debug(formatter),
    Class::_Infinite(class) => class.debug(formatter),
    Class::_Limit(class) => class.debug(formatter),
    Class::_Nest(class) => class.debug(formatter),
    Class::_Node(class) => class.debug(formatter),
    Class::_Start(class) => class.debug(formatter),
    Class::_Tensor(class) => class.debug(formatter),
    Class::_Term(class) => class.debug(formatter),
    Class::_Use(class) => class.debug(formatter),
    Class::_Variable(class) => class.debug(formatter),
    Class::_Whole(class) => class.debug(formatter)
}}}

//> CLASS -> COMMON
impl Class {fn result(&self, value: Object) -> Object {
    class(format!(
        "{} > {:?}",
        value, value
    ));
    return value;
}}