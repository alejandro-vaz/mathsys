//^
//^ HEAD
//^

//> HEAD -> PETGRAPH
use petgraph::graph::NodeIndex;


//^
//^ TRACE
//^

//> TRACE -> STRUCT
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)] // rm debug
pub struct Trace(pub NodeIndex);