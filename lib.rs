//^
//^ HEAD
//^

//> HEAD -> FLAGS
#![allow(unused_variables)]
#![allow(nonstandard_style)]
#![feature(try_trait_v2)]
#![feature(default_field_values)]
#![feature(if_let_guard)]

//> HEAD -> MODULES
pub mod prelude;
mod issues;
mod entry;
mod tokenizer;
mod parser;
mod solver;
mod syntax;
mod backends;
pub mod settings;

//> HEAD -> PRELUDE
use self::prelude::{
    Time, Duration, ARCH, OS, rustcv
};

//> HEAD -> LOCAL
use self::{
    syntax::Start,
    tokenizer::{
        token::ShallowToken,
        Tokenizer,
        MAXLEN
    },
    parser::Parser,
    solver::Solver,
    backends::Backends,
    issues::Issue,
    settings::Settings,
    entry::Target
};


//^
//^ PIPELINE
//^

//> PIPELINE -> VERSION
static VERSION: usize = 9;

//> PIPELINE -> BASE TRANSFORMERS
pub static TRANSFORMERS: Transformers = Transformers::new();

//> PIPELINE -> TRANSFORMERS
pub struct Transformers {
    tokenizer: Tokenizer,
    parser: Parser,
    solver: Solver
} impl Transformers {
    const fn new() -> Self {return Transformers {
        tokenizer: Tokenizer::new(),
        parser: Parser::new(),
        solver: Solver::new()
    }}
    pub fn run(&self, settings: &Settings) -> Vec<Run> {
        let mut time = Time::now();
        let mut results = Vec::new();
        for target in &settings.targets {
            results.push(Run {
                time: time.elapsed(),
                data: self.compute(target, settings)
            });
            time = Time::now();
        }
        return results;
    }
    fn compute(&self, target: &Target, settings: &Settings) -> Result<Data, Issue> {Ok(match target {
        Target::Help => return Err(Issue::GetHelp),
        Target::Version => Data::Version {
            mathsys: VERSION,
            rust: rustcv().minor,
            ..
        },
        Target::Tokens => {let tokens = self.tokens(settings)?; Data::Tokens {
            length: tokens.len(),
            percentage: tokens.len() as f32 / MAXLEN as f32,
            tokens: tokens,
            ..
        }},
        Target::Check => match self.start(settings) {
            Ok(start) => Data::Check,
            Err(issue) => return Err(issue)
        },
        Target::Ast => Data::Ast {
            start: self.start(settings)?
        },
        Target::Latex => Data::Latex {
            representation: self.start(settings)?.latex()
        }
    })}
    fn content(&self, settings: &Settings) -> Result<String, Issue> {settings.files.iter().next().ok_or(Issue::MissingFile)?.read()}
    fn tokens(&self, settings: &Settings) -> Result<Vec<ShallowToken>, Issue> {Ok(self.tokenizer.run(&self.content(settings)?, settings)?.into_iter().map(|token| token.fixate()).collect())}
    fn start(&self, settings: &Settings) -> Result<Start, Issue> {
        let content = self.content(settings)?;
        let mut start = self.solver.run(&self.parser.run(&self.tokenizer.run(&content, settings)?, settings))?;
        start.modules(self, settings)?;
        return Ok(start);
    }
}

//> PIPELINE -> RUN
pub struct Run {
    pub time: Duration,
    pub data: Result<Data, Issue>
}

//> PIPELINE -> DATA
pub enum Data {
    Version {
        mathsys: usize,
        architecture: &'static str = ARCH,
        os: &'static str = OS,
        rust: u64
    },
    Tokens {
        length: usize,
        percentage: f32,
        tokens: Vec<ShallowToken>,
        maximum: usize = MAXLEN
    },
    Check,
    Ast {
        start: Start
    },
    Latex {
        representation: String
    }
}