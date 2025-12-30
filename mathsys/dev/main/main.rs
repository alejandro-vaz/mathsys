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
    ir: include_bytes!(env!("MathsysSource")),
    version: env!("MathsysVersion"),
    debug: true,
    class: true,
    chore: true,
    trace: true,
    alert: true,
    point: true
})}