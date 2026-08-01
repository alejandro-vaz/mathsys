//^
//^ HEAD
//^

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;

//> HEAD -> SUPER
use super::item::Item;


//^
//^ SUBTREE
//^

//> SUBTREE -> ENUM
#[enum_dispatch]
#[derive(EnumAsInner, Debug)] // rm debug
pub enum Subtree<'parsing, 'valid> {
    Item(Item<'parsing, 'valid>),
    Vec(Vec<Item<'parsing, 'valid>>)
}