//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    _Absolute, _Annotation, _Casts, _Declaration, _Definition, _Equation, _Expression, _Factor, _Infinite, _Limit, _Nest, _Node, _Rational, _Start, _Tensor, _Term, _Undefined, _Use, _Variable, _Whole, Pointer, class, fmt, space, type_name
};


//^
//^ TIP
//^

//> TIP -> TRAIT
pub trait Tip: fmt::Debug {fn section<Type: fmt::Display>(&self, message: Type, id: Pointer) -> () {space(format!(
    "{{{}{}}} {message}",
    id.0,
    type_name::<Self>().rsplit("::").next().unwrap()
)); class(format!("{}{self:?}", id.0))}}

//> TIP -> IMPLEMENTATIONS
impl Tip for _Absolute {}
impl Tip for _Annotation {}
impl Tip for _Casts {}
impl Tip for _Declaration {}
impl Tip for _Definition {}
impl Tip for _Equation {}
impl Tip for _Expression {}
impl Tip for _Factor {}
impl Tip for _Infinite {}
impl Tip for _Limit {}
impl Tip for _Nest {}
impl Tip for _Node {}
impl Tip for _Rational {}
impl Tip for _Start {}
impl Tip for _Tensor {}
impl Tip for _Term {}
impl Tip for _Undefined {}
impl Tip for _Use {}
impl Tip for _Variable {}
impl Tip for _Whole {}