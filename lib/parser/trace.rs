//^
//^ HEAD
//^

//> HEAD -> PETGRAPH
use petgraph::graph::NodeIndex;


//^
//^ TRACE
//^

//> TRACE -> STRUCT
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Trace(pub NodeIndex);