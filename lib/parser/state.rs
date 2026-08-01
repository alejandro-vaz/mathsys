//^
//^ HEAD
//^

//> HEAD -> PETGRAPH
use petgraph::graph::NodeIndex;


//^
//^ STATE
//^

//> STATE -> STRUCT
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct State(pub NodeIndex);