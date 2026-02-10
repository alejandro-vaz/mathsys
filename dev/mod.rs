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
    pub mod resolver;
    pub mod start;
    pub mod tokenizer;
}

//> HEAD -> PRELUDE
use crate::prelude::{
    Argument, File, Flag, Time, Target, VERSION, OS, ARCH, rustcv
};

//> HEAD -> LOCAL
use self::base::start::Start;
use self::base::issues::Issue;
use self::base::tokenizer::{ShallowToken, Tokenizer, MAXLEN};
use self::base::parser::Parser;
use self::base::resolver::Resolver;


//^
//^ PIPELINE
//^

//> PIPELINE -> TRANSFORMERS
pub struct Transformers {
    tokenizer: Tokenizer,
    parser: Parser,
    resolver: Resolver
} impl Transformers {pub fn new() -> Self {return Transformers {
    tokenizer: Tokenizer::new(),
    parser: Parser::new(),
    resolver: Resolver::new()
}}}

//> PIPELINE -> HELP
pub fn help(settings: &Settings, transformers: &mut Transformers) -> Result<(), Issue> {return Err(Issue::GetHelp)}

//> PIPELINE -> VERSION
pub fn version(settings: &Settings, transformers: &mut Transformers) -> Result<usize, Issue> {return Ok(VERSION)}

//> PIPELINE -> TOKENS
pub fn tokens(settings: &Settings, transformers: &mut Transformers) -> Result<Vec<ShallowToken>, Issue> {
    let content = settings.file.clone().ok_or(Issue::MissingFile)?.read();
    let tokens = transformers.tokenizer.run(&content, settings)?;
    return Ok(tokens.into_iter().map(|token| token.fixate()).collect());
}

//> PIPELINE -> LENGTH
pub fn length(settings: &Settings, transformers: &mut Transformers) -> Result<usize, Issue> {
    let content = settings.file.clone().ok_or(Issue::MissingFile)?.read();
    let tokens = transformers.tokenizer.run(&content, settings)?;
    return Ok(tokens.len());
}

//> PIPELINE -> CHECK
pub fn check(settings: &Settings, transformers: &mut Transformers) -> Result<(), Issue> {
    let content = settings.file.clone().ok_or(Issue::MissingFile)?.read();
    let tokens = transformers.tokenizer.run(&content, settings)?;
    let pool = transformers.parser.run(&tokens, settings);
    let start = transformers.resolver.run(&pool)?;
    return Ok(());
}

//> PIPELINE -> AST
pub fn ast(settings: &Settings, transformers: &mut Transformers) -> Result<Start, Issue> {
    let content = settings.file.clone().ok_or(Issue::MissingFile)?.read();
    let tokens = transformers.tokenizer.run(&content, settings)?;
    let pool = transformers.parser.run(&tokens, settings);
    let start = transformers.resolver.run(&pool)?;
    return Ok(start);
}


//^
//^ TARGETS
//^

//> TARGETS -> NOISE
enum Noise {
    Debug,
    Verbose,
    Normal,
    Quiet,
    Zero
} impl Noise {
    fn change(&mut self, shift: bool) -> () {*self = match self {
        Noise::Debug => if shift {Noise::Debug} else {Noise::Verbose},
        Noise::Normal => if shift {Noise::Verbose} else {Noise::Quiet},
        Noise::Quiet => if shift {Noise::Normal} else {Noise::Zero},
        Noise::Verbose => if shift {Noise::Debug} else {Noise::Normal},
        Noise::Zero => if shift {Noise::Quiet} else {Noise::Zero}
    }}
    fn verbose(&self) -> bool {match self {
        Noise::Debug | Noise::Verbose => true,
        other => false
    }}
    fn quiet(&self) -> bool {match self {
        Noise::Quiet | Noise::Zero => true,
        other => false
    }}
    fn debug(&self) -> bool {if let Noise::Debug = self {true} else {false}}
    fn zero(&self) -> bool {if let Noise::Zero = self {true} else {false}}
}

//> TARGETS -> SETTINGS
pub struct Settings {
    file: Option<File> = None,
    target: Option<Target> = None,
    noise: Noise = Noise::Normal
} impl Settings {
    pub fn set(arguments: &[Argument]) -> Result<Settings, Issue> {
        let mut settings = Settings {..};
        for argument in arguments {settings.apply(argument)?}
        return Ok(settings);
    }
    pub fn apply(&mut self, argument: &Argument) -> Result<(), Issue> {return Ok(match argument {
        Argument::Alias(alias) => for (index, letter) in alias.letters.iter().enumerate() {if let Some((character, aliasing)) = FLAGLIASES.iter().find_map(|(key, second, third)| if key == letter {Some((key, second))} else {None}) {
            self.apply(&Argument::Flag(Flag {value: aliasing.to_string()}))?;
        } else {return Err(Issue::UnknownAliasCharacter {
            alias: alias.clone(), 
            at: index
        })}}
        Argument::File(file) => if self.file.is_none() {self.file = Some(file.clone())},
        Argument::Flag(flag) => match &flag.value.to_lowercase() as &str {
            name if name == FLAGLIASES[0].1 => return Err(Issue::GetHelp),
            name if name == FLAGLIASES[1].1 => self.noise.change(false),
            name if name == FLAGLIASES[2].1 => self.noise.change(true),
            other => return Err(Issue::UnknownFlag(flag.clone()))
        },
        Argument::Target(target) => if self.target.is_none() {self.target = Some(target.clone())}
    })}
}

//> TARGETS -> WRAPPER
pub fn wrapper(arguments: &[Argument]) -> Result<(), Issue> {
    let time = Time::now();
    let settings = Settings::set(arguments)?;
    let mut transformers = Transformers::new();
    match &settings.target.clone().unwrap_or(Target {name: String::from("help")}).name as &str {
        target if target == TARGETS[0].0 => help(&settings, &mut transformers)?,
        target if target == TARGETS[1].0 => println!("Running Mathsys v{} on {}/{}/{}", version(&settings, &mut transformers)?, OS, ARCH, rustcv()),
        target if target == TARGETS[2].0 => println!("{:#?}", tokens(&settings, &mut transformers)?),
        target if target == TARGETS[3].0 => {let len = length(&settings, &mut transformers)?; println!("Token length: {len} / {MAXLEN} ({}%)", len as f32 / MAXLEN as f32 * 100.0)},
        target if target == TARGETS[4].0 => println!("{}", {check(&settings, &mut transformers)?; "Valid"}),
        target if target == TARGETS[5].0 => println!("{:#?}", ast(&settings, &mut transformers)?),
        other => return Err(Issue::UnknownTarget(other.to_string()))
    };
    if settings.noise.verbose() {println!("Execution time: {:?}", time.elapsed())}
    return Ok(());
}

//> TARGETS -> LIST
pub static TARGETS: [(&'static str, &'static str); 6] = [
    ("help", "show this informative menu"),
    ("version", "check current Mathsys version"),
    ("tokens", "tokenize the input file"),
    ("length", "show length of token stream"),
    ("check", "validate the semantics"),
    ("ast", "build the abstract syntax tree")
];

//> TARGETS -> FLAGS AND ALIASES
pub static FLAGLIASES: [(char, &'static str, &'static str); 3] = [
    ('h', "help", "show help menu"),
    ('q', "quiet", "silence the output"),
    ('v', "verbose", "increase program verbosity")
];