//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    state::State,
    trace::Trace
};


//^
//^ PARSEHEAD
//^

//> PARSEHEAD -> STRUCT
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Head {
    pub state: State,
    pub trace: Trace
}