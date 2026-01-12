//^
//^ HEAD
//^

//> HEAD -> RUNTIME
use mathsys::{run, Settings};


//^
//^ MAIN
//^

//> MAIN -> FUNCTION
fn main() -> () {run(Settings {
    ir: include_bytes!(env!("MathsysIr")),
    version: env!("MathsysVersion"),
    debug: env!("MathsysDebug").parse().unwrap(),
    class: env!("MathsysClass").parse().unwrap(),
    chore: env!("MathsysChore").parse().unwrap(),
    trace: env!("MathsysTrace").parse().unwrap(),
    alert: env!("MathsysAlert").parse().unwrap(),
    point: env!("MathsysPoint").parse().unwrap()
})}