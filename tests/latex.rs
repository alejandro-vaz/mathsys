//^
//^ HEAD
//^

//> HEAD -> API
use mathsys::Interpreter;


//^
//^ TESTS
//^

//> TESTS -> DATA
#[test]
fn data() -> () {
    let a = Interpreter::ARCHITECTURE;
    let b = Interpreter::OS;
    let c = Interpreter::VERSION;
}

//> TESTS -> INPUT
#[test]
fn input() -> () {
    let text = r#"2 + 2 = 4
    3 * 7
    use "mymodule"
    caverna
    "#.repeat(20);
    let o = Interpreter.latex(&text);
    //assert_eq!(o.value.unwrap(), text);
}