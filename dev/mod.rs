//^
//^ HEAD
//^

//> HEAD -> BASE
mod base {
    pub mod grammar;
    pub mod issues;
    pub mod level1;
    pub mod level2;
    pub mod level3;
    pub mod level4;
    pub mod level5;
    pub mod nonterminal;
    pub mod parser;
    pub mod start;
    pub mod tokenizer;
}

//> HEAD -> PRELUDE
use crate::prelude::{
    Argument, File, Flag, Time, Target
};

//> HEAD -> LOCAL
use self::base::issues::{noFileProvided, noTargetProvided, Issue, unknownTarget};
use self::base::tokenizer::{Token, Tokenizer};
use self::base::parser::Parser;


//^
//^ UTILS
//^

//> UTILS -> TIMED
fn timed<Function, Return>(function: Function) -> Return where Function: FnOnce() -> Return {
    let start = Time::now();
    let result = function();
    println!("[INFO] Compiled in {:.3?}", start.elapsed());
    return result;
}


//^
//^ PIPELINE
//^

//> PIPELINE -> TARGET TRAIT
trait TargetType {
    type Output;
    fn act(settings: Settings) -> Result<Self::Output, Issue>;
}

//> PIPELINE -> TOKENS
struct Tokens; impl TargetType for Tokens {
    type Output = Vec<Token>; 
    fn act(settings: Settings) -> Result<Self::Output, Issue> {
        //= TOKENS -> PRELOAD
        let content = settings.file.read();
        let mut tokenizer = Tokenizer::new();
        //= TOKENS -> RUN
        return timed(|| tokenizer.run(content));
    }
}

//> PIPELINE -> LENGTH
struct Length; impl TargetType for Length {
    type Output = usize;
    fn act(settings: Settings) -> Result<Self::Output, Issue> {
        //= LENGTH -> PRELOAD
        let content = settings.file.read();
        let mut tokenizer = Tokenizer::new();
        //= LENGTH -> RUN
        return timed(|| Ok(tokenizer.run(content)?.len()));
    }
}

//> PIPELINE -> VALIDATE
struct Validate; impl TargetType for Validate {
    type Output = bool;
    fn act(settings: Settings) -> Result<Self::Output, Issue> {
        //= VALIDATE -> PRELOAD
        let content = settings.file.read();
        let mut tokenizer = Tokenizer::new();
        let mut parser = Parser::new();
        //= VALIDATE -> RUN
        return timed(|| Ok(parser.run(tokenizer.run(content)?)?));
    }
}


//^
//^ TARGETS
//^

//> TARGETS -> RUN
struct Run {
    debug: bool,
    class: bool,
    chore: bool,
    trace: bool,
    alert: bool,
    point: bool
}

//> TARGETS -> SETTINGS
struct Settings {
    file: File,
    target: Target,
    run: Run
} impl Settings {
    pub fn set(arguments: Vec<Argument>) -> Result<Settings, Issue> {
        let file = arguments.iter().find_map(|argument| if let Argument::File(value) = argument {Some(value.clone())} else {None});
        let target = arguments.iter().find_map(|argument| if let Argument::Target(value) = argument {Some(value.clone())} else {None});
        if let None = file {return Err(noFileProvided())};
        if let None = target {return Err(noTargetProvided())};
        let mut settings = Settings {
            file: file.unwrap(),
            target: target.unwrap(),
            run: Run {
                debug: false,
                class: false,
                chore: true,
                trace: true,
                alert: false,
                point: true
            }
        };
        arguments.iter().for_each(|argument| settings.apply(argument));
        return Ok(settings);
    }
    pub fn apply(&mut self, argument: &Argument) -> () {match argument {
        Argument::Alias(alias) => alias.letters.iter().for_each(|letter| self.apply(&Argument::Flag(Flag {value: String::from(match letter {
            'z' => "optsize",
            'o' => "optimize",
            'd' => "debug",
            'c' => "class",
            'h' => "no-chore",
            't' => "no-trace",
            'a' => "alert",
            'p' => "no-point",
            other => return
        })}))),
        Argument::File(file) => return,
        Argument::Flag(flag) => match flag.value.as_str() {
            "debug" => self.run.debug = true,
            "class" => self.run.class = true,
            "no-chore" => self.run.chore = false,
            "no-trace" => self.run.trace = false,
            "alert" => self.run.alert = true,
            "no-point" => self.run.point = false,
            other => return
        },
        Argument::Target(target) => return
    }}
}

//> TARGETS -> WRAPPER
pub fn wrapper(arguments: Vec<Argument>) -> Result<(), Issue> {
    let settings = Settings::set(arguments)?;
    match &*settings.target.name {
        "tokens" => println!("{:#?}", Tokens::act(settings)?),
        "length" => println!("{}", Length::act(settings)?),
        "validate" => println!("{}", Validate::act(settings)?),
        other => return Err(unknownTarget(other))
    };
    return Ok(());
}

//> TARGETS -> FUNCTIONS
pub static TARGETS: [&'static str; 3] = [
    "tokens",
    "length",
    "validate"
];