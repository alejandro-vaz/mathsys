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
use self::base::tokenizer::{ShallowToken, Tokenizer};
use self::base::parser::Parser;
use self::base::start::Start;


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

//> PIPELINE -> TRANSFORMERS
pub struct Transformers {
    tokenizer: Tokenizer,
    parser: Parser
} impl Transformers {pub fn new() -> Self {return Transformers {
    tokenizer: Tokenizer::new(),
    parser: Parser::new()
}}}

//> PIPELINE -> TOKENS
pub fn tokens(settings: &Settings, transformers: &mut Transformers) -> Result<Vec<ShallowToken>, Issue> {
    let content = settings.file.read();
    return Ok(transformers.tokenizer.run(&content, settings)?.into_iter().map(|token| token.fixate()).collect());
}

//> PIPELINE -> LENGTH
pub fn length(settings: &Settings, transformers: &mut Transformers) -> Result<usize, Issue> {
    let content = settings.file.read();
    return Ok(transformers.tokenizer.run(&content, settings)?.len());
}

//> PIPELINE -> AST
pub fn ast(settings: &Settings, transformers: &mut Transformers) -> Result<Start, Issue> {
    let content = settings.file.read();
    let tokens = transformers.tokenizer.run(&content, settings)?;
    return transformers.parser.run(tokens, settings);
}


//^
//^ TARGETS
//^

//> TARGETS -> RUN
struct Run {
    debug: bool = false,
    class: bool = false,
    chore: bool = true,
    trace: bool = true,
    alert: bool = false,
    point: bool = true
}

//> TARGETS -> SETTINGS
pub struct Settings {
    file: File,
    target: Target,
    run: Run
} impl Settings {
    pub fn set(arguments: Vec<Argument>) -> Result<Settings, Issue> {
        let file = arguments.iter().find_map(|argument| if let Argument::File(value) = argument {Some(value.clone())} else {None});
        let target = arguments.iter().find_map(|argument| if let Argument::Target(value) = argument {Some(value.clone())} else {None});
        if file.is_none() {return Err(noFileProvided())};
        if target.is_none() {return Err(noTargetProvided())};
        let mut settings = Settings {
            file: file.unwrap(),
            target: target.unwrap(),
            run: Run {..}
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
        Argument::Flag(flag) => match &flag.value as &str {
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
pub fn wrapper(arguments: Vec<Argument>) -> Result<(), Issue> {timed(|| {
    let settings = Settings::set(arguments)?;
    let mut transformers = Transformers::new();
    return Ok(match &settings.target.name as &str {
        target if target == TARGETS[0] => println!("{:#?}", tokens(&settings, &mut transformers)?),
        target if target == TARGETS[1] => println!("{}", length(&settings, &mut transformers)?),
        target if target == TARGETS[2] => println!("{:#?}", ast(&settings, &mut transformers)?),
        other => return Err(unknownTarget(other))
    });
})}

//> TARGETS -> FUNCTIONS
pub static TARGETS: [&'static str; 3] = [
    "tokens",
    "length",
    "ast"
];