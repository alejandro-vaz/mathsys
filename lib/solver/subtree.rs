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
#[derive(Clone, EnumAsInner, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Subtree<'valid> {
    Item(Item<'valid>),
    Vec(Vec<Item<'valid>>)
}