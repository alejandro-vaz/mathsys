//^
//^ HEAD
//^

//> HEAD -> RUNTIME
use core::{run, Settings};


//^
//^ MAIN
//^

//> MAIN -> FUNCTION
fn main() -> () {run(Settings {
    ir: include_bytes!(env!("MathsysSource")),
    version: [
        env!("MathsysMajor"), 
        env!("MathsysMinor"), 
        env!("MathsysPatch")
    ]
})}