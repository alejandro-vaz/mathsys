//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    _Absolute, _Annotation, _Casts, _Declaration, _Definition, _Equation, _Expression, _Factor, _Infinite, _Limit, _Nest, _Node, _Rational, _Start, _Tensor, _Term, _Undefined, _Use, _Variable, _Whole, Runtime, Pointer, Object, fmt, stdout
};


//^
//^ CLASS
//^

//> CLASS -> ENUM
#[derive(Clone)]
pub enum Class {
    _Absolute(_Absolute),
    _Annotation(_Annotation),
    _Casts(_Casts),
    _Declaration(_Declaration),
    _Definition(_Definition),
    _Equation(_Equation),
    _Expression(_Expression),
    _Factor(_Factor),
    _Infinite(_Infinite),
    _Limit(_Limit),
    _Nest(_Nest),
    _Node(_Node),
    _Rational(_Rational),
    _Start(_Start),
    _Tensor(_Tensor),
    _Term(_Term),
    _Undefined(_Undefined),
    _Use(_Use),
    _Variable(_Variable),
    _Whole(_Whole)
}

//> CLASS -> EVALUATE
impl Class {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {return self.result(match self {
        Class::_Absolute(class) => class.evaluate(runtime, id, memory),
        Class::_Annotation(class) => class.evaluate(runtime, id, memory),
        Class::_Casts(class) => class.evaluate(runtime, id, memory),
        Class::_Declaration(class) => class.evaluate(runtime, id, memory),
        Class::_Definition(class) => class.evaluate(runtime, id, memory),
        Class::_Equation(class) => class.evaluate(runtime, id, memory),
        Class::_Expression(class) => class.evaluate(runtime, id, memory),
        Class::_Factor(class) => class.evaluate(runtime, id, memory),
        Class::_Infinite(class) => class.evaluate(runtime, id, memory),
        Class::_Limit(class) => class.evaluate(runtime, id, memory),
        Class::_Nest(class) => class.evaluate(runtime, id, memory),
        Class::_Node(class) => class.evaluate(runtime, id, memory),
        Class::_Rational(class) => class.evaluate(runtime, id, memory),
        Class::_Start(class) => class.evaluate(runtime, id, memory),
        Class::_Tensor(class) => class.evaluate(runtime, id, memory),
        Class::_Term(class) => class.evaluate(runtime, id, memory),
        Class::_Undefined(class) => class.evaluate(runtime, id, memory),
        Class::_Use(class) => class.evaluate(runtime, id, memory),
        Class::_Variable(class) => class.evaluate(runtime, id, memory),
        Class::_Whole(class) => class.evaluate(runtime, id, memory)
})}}

//> CLASS -> DEBUG
impl fmt::Debug for Class {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {match self {
    Class::_Absolute(item) => fmt::Debug::fmt(item, formatter),
    Class::_Annotation(item) => fmt::Debug::fmt(item, formatter),
    Class::_Casts(item) => fmt::Debug::fmt(item, formatter),
    Class::_Declaration(item) => fmt::Debug::fmt(item, formatter),
    Class::_Definition(item) => fmt::Debug::fmt(item, formatter),
    Class::_Equation(item) => fmt::Debug::fmt(item, formatter),
    Class::_Expression(item) => fmt::Debug::fmt(item, formatter),
    Class::_Factor(item) => fmt::Debug::fmt(item, formatter),
    Class::_Infinite(item) => fmt::Debug::fmt(item, formatter),
    Class::_Limit(item) => fmt::Debug::fmt(item, formatter),
    Class::_Nest(item) => fmt::Debug::fmt(item, formatter),
    Class::_Node(item) => fmt::Debug::fmt(item, formatter),
    Class::_Rational(item) => fmt::Debug::fmt(item, formatter),
    Class::_Start(item) => fmt::Debug::fmt(item, formatter),
    Class::_Tensor(item) => fmt::Debug::fmt(item, formatter),
    Class::_Term(item) => fmt::Debug::fmt(item, formatter),
    Class::_Undefined(item) => fmt::Debug::fmt(item, formatter),
    Class::_Use(item) => fmt::Debug::fmt(item, formatter),
    Class::_Variable(item) => fmt::Debug::fmt(item, formatter),
    Class::_Whole(item) => fmt::Debug::fmt(item, formatter),
}}}

//> CLASS -> COMMON
impl Class {fn result(&self, value: Object) -> Object {
    stdout.point(format!("{value:?}"));
    return value;
}}