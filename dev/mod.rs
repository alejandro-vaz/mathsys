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
    Argument, File, Flag, Future, Instant, Target
};

//> HEAD -> LOCAL
use self::base::issues::{noFileProvided, noTargetProvided, Issue, unknownTarget};
use self::base::tokenizer::{Token, Tokenizer};


//^
//^ UTILS
//^

//> UTILS -> TIMED
async fn timed<Function, In, Return>(function: Function) -> Return where Function: FnOnce() -> In, In: Future<Output = Return> {
    let start = Instant::now();
    let result = function().await;
    println!("[INFO] Compiled in {:.3?}", start.elapsed());
    return result;
}


//^
//^ PIPELINE
//^

//> PIPELINE -> TARGET TRAIT
trait TargetType {
    type Output;
    async fn act(settings: Settings) -> Result<Self::Output, Issue>;
}

//> PIPELINE -> TOKENS
struct Tokens; impl TargetType for Tokens {
    type Output = Vec<Token>; 
    async fn act(settings: Settings) -> Result<Self::Output, Issue> {
        //= TOKENS -> PRELOAD
        let content = settings.content().await;
        let mut tokenizer = Tokenizer::new();
        //= TOKENS -> RUN
        return timed(async || tokenizer.run(content)).await;
    }
}

//> PIPELINE -> LENGTH
struct Length; impl TargetType for Length {
    type Output = usize;
    async fn act(settings: Settings) -> Result<Self::Output, Issue> {
        //= LENGTH -> PRELOAD
        let content = settings.content().await;
        let mut tokenizer = Tokenizer::new();
        //= LENGTH -> RUN
        return timed(async || Ok(tokenizer.run(content)?.len())).await;
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
    pub async fn content(&self) -> String {self.file.read().await}
}

//> TARGETS -> WRAPPER
pub async fn wrapper(arguments: Vec<Argument>) -> Result<(), Issue> {
    let settings = Settings::set(arguments)?;
    match &*settings.target.name {
        "tokens" => println!("{:#?}", Tokens::act(settings).await?),
        "length" => println!("{}", Length::act(settings).await?),
        other => return Err(unknownTarget(other))
    };
    return Ok(());
}

//> TARGETS -> FUNCTIONS
pub static TARGETS: [&'static str; 2] = [
    "tokens",
    "length"
];