//^
//^ HEAD
//^

use crate::dev::base::nonterminal::NonTerminal;
//> HEAD -> PRELUDE
use crate::prelude::{
    HashMap, Regex, HashSet, LazyLock
};

//> HEAD -> LOCAL
use super::nonterminal::Object;
use super::tokenizer::Kind;


//^
//^ EBNF
//^

//> EBNF -> SYNTAX
//> GRAMMAR -> SYNTAX
pub static GRAMMAR: LazyLock<HashMap<Rule, Vec<Vec<Symbol>>>> = LazyLock::new(|| Extensor::new().run("
//> SYNTAX -> START
Start -> (NEWLINES? Level1 SPACES? (NEWLINES Level1 SPACES?)*)? NEWLINES? ENDOFFILE

//> SYNTAX -> 1ºLEVEL
Declaration -> (GROUP SPACES)? Variable SPACES? EQUALITY SPACES? Level2
Definition -> (GROUP SPACES)? Variable SPACES? BINDING SPACES? Level2
Annotation -> GROUP SPACES Variable (SPACES? COMMA SPACES? Variable)*
Node -> Level2
Equation -> Level2 SPACES? EQUALITY SPACES? Level2
Use -> USE SPACES MODULE

//> SYNTAX -> 2ºLEVEL
Expression -> (SIGN SPACES?)* Level3 ((SPACES? SIGN)+ SPACES? Level3)*

//> SYNTAX -> 3ºLEVEL
Term -> Level4 ((SPACES? OPERATOR)? SPACES? Level4)*

//> SYNTAX -> 4ºLEVEL
Factor -> Level5 (EXPONENTIATION SPACES? Level2 SPACES? EXPONENTIATION)?
Limit -> LIMIT SPACES Variable SPACES? TO SPACES? Level2 SIGN? SPACES OF SPACES Nest (EXPONENTIATION SPACES? Level2 SPACES? EXPONENTIATION)?

//> SYNTAX -> 5ºLEVEL
Infinite -> INFINITE
Variable -> IDENTIFIER
Nest -> OPEN SPACES? Level2? SPACES? CLOSE
Tensor -> ENTER SPACES? (Level2 (SPACES? COMMA SPACES? Level2)* SPACES?)? EXIT
Whole -> NUMBER
Absolute -> PIPE SPACES? Level2 SPACES? PIPE
Undefined -> UNDEFINED
Rational -> RATIONAL
Casts -> Level5 GROUP

//> SYNTAX -> LEVELS
Level1 -> Declaration | Definition | Annotation | Node | Equation | Use
Level2 -> Expression
Level3 -> Term
Level4 -> Factor | Limit
Level5 -> Infinite | Variable | Nest | Tensor | Whole | Absolute | Undefined | Rational | Casts
"));

//> EBNF -> PATTERNS
static COLLAPSEREGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\(([^()]+)\)").unwrap());
static POSTFIXREGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?P<atom>\$[0-9]+|[A-Z][a-z]*[0-9]*|_?[A-Z]+)(?P<operator>[*+?])").unwrap());

//> EBNF -> EXTENSOR
struct Extensor {
    counter: u8,
    parentheses: HashMap<String, u8>,
    more: HashMap<String, u8>,
    multiple: HashMap<String, u8>,
    optional: HashMap<String, u8>,
} impl Extensor {
    pub fn new() -> Extensor {return Self {
        counter: 0,
        parentheses: HashMap::new(),
        more: HashMap::new(),
        multiple: HashMap::new(),
        optional: HashMap::new(),
    }}
    fn reset(&mut self) -> HashSet<String> {
        self.counter = 0;
        self.parentheses.clear();
        self.more.clear();
        self.multiple.clear();
        self.optional.clear();
        return HashSet::new();
    }
    pub fn run(&mut self, ebnf: &str) -> HashMap<Rule, Vec<Vec<Symbol>>> {
        let mut rules = self.reset();
        for line in ebnf.lines().map(str::trim) {
            if line.is_empty() || line.starts_with("//") {continue}
            let (rule, productions) = line.split_once("->").unwrap();
            let body = self.expand(productions, &mut rules);
            rules.insert(format!("{rule} -> {body}"));
        };
        rules.insert("$0 -> Start".to_string());
        let mut ordered = rules.into_iter().collect::<Vec<String>>();
        ordered.sort_by(|first, second| {
            let [frule, srule] = [first, second].map(|each| each.splitn(2, "->").next().unwrap().trim());
            let [fkey, skey] = [frule, srule].map(|each| if each.starts_with('$') {(1, each[1..].parse::<usize>().unwrap())} else {(0, 0)});
            fkey.cmp(&skey).then(frule.cmp(srule))
        });
        return self.serialize(ordered.join("\n"));
    }
    fn expand(&mut self, expression: &str, rules: &mut HashSet<String>) -> String {
        let mut result = expression.to_string();
        while result.contains("(") {result = self.collapse(&result, rules)};
        result = self.postfix(&result, rules);
        return result.to_string();
    }
    fn collapse(&mut self, expression: &str, rules: &mut HashSet<String>) -> String {
        let Some(hit) = COLLAPSEREGEX.find(expression.as_bytes()) else {return expression.to_string()};
        let inner = &expression[hit.start() + 1 .. hit.end() - 1].to_string();
        let symbol = *self.parentheses.entry(inner.to_string()).or_insert_with(|| {self.counter += 1; self.counter});
        let expanded = self.expand(inner, rules);
        rules.insert(format!("${symbol} -> {expanded}"));
        return format!("{}${symbol}{}", &expression[..hit.start()], &expression[hit.end()..]);
    }
    fn postfix(&mut self, expression: &str, rules: &mut HashSet<String>) -> String {
        let mut result = expression.to_string();
        while let Some(hit) = POSTFIXREGEX.captures(&result.as_bytes()) {
            let atom = String::from_utf8(hit.name("atom").unwrap().as_bytes().to_vec()).unwrap();
            let operator = String::from_utf8(hit.name("operator").unwrap().as_bytes().to_vec()).unwrap();
            let symbol = match &operator as &str {
                "+" => self.more.entry(atom.clone()).or_insert_with(|| {self.counter += 1; self.counter}),
                "*" => self.multiple.entry(atom.clone()).or_insert_with(|| {self.counter += 1; self.counter}),
                "?" => self.optional.entry(atom.clone()).or_insert_with(|| {self.counter += 1; self.counter}),
                other => unreachable!()
            };
            rules.insert(match &operator as &str {
                "+" => format!("${symbol} -> {atom} ${symbol} | {atom}"),
                "*" => format!("${symbol} -> {atom} ${symbol} |"),
                "?" => format!("${symbol} -> {atom} |"),
                other => unreachable!()
            });
            let groupzero = hit.get(0).unwrap();
            result = format!("{}${symbol}{}", &result[..groupzero.start()], &result[groupzero.end()..])
        };
        return result;
    }
    fn serialize(&self, bnf: String) -> HashMap<Rule, Vec<Vec<Symbol>>> {
        let mut map = HashMap::new();
        for line in bnf.lines() {
            let [rule, productions] = line.splitn(2, "->").map(str::trim).collect::<Vec<&str>>()[0..2] else {panic!("{line}")};
            for variant in productions.split("|").map(str::trim) {
                map.entry(rule.into()).or_insert_with(Vec::new).push(if variant.is_empty() {
                    Vec::new()
                } else {
                    variant.split(" ").map(Symbol::from).collect()
                });
            }
        }
        return map;
    }
}


//^
//^ GRAMMAR
//^

//> GRAMMAR -> RULE
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum Rule {
    NonTerminal(Object),
    Internal(u8)
} impl From<&str> for Rule {fn from(value: &str) -> Self {match value {
    internal if internal.starts_with("$") => Rule::Internal(internal[1..].parse().unwrap()),
    "Start" => Rule::NonTerminal(Object::Start),
    "Level1" => Rule::NonTerminal(Object::Level1),
    "Level2" => Rule::NonTerminal(Object::Level2),
    "Level3" => Rule::NonTerminal(Object::Level3),
    "Level4" => Rule::NonTerminal(Object::Level4),
    "Level5" => Rule::NonTerminal(Object::Level5),
    "Declaration" => Rule::NonTerminal(Object::Declaration),
    "Definition" => Rule::NonTerminal(Object::Definition),
    "Annotation" => Rule::NonTerminal(Object::Annotation),
    "Node" => Rule::NonTerminal(Object::Node),
    "Equation" => Rule::NonTerminal(Object::Equation),
    "Use" => Rule::NonTerminal(Object::Use),
    "Expression" => Rule::NonTerminal(Object::Expression),
    "Term" => Rule::NonTerminal(Object::Term),
    "Factor" => Rule::NonTerminal(Object::Factor),
    "Limit" => Rule::NonTerminal(Object::Limit),
    "Infinite" => Rule::NonTerminal(Object::Infinite),
    "Variable" => Rule::NonTerminal(Object::Variable),
    "Nest" => Rule::NonTerminal(Object::Nest),
    "Tensor" => Rule::NonTerminal(Object::Tensor),
    "Whole" => Rule::NonTerminal(Object::Whole),
    "Absolute" => Rule::NonTerminal(Object::Absolute),
    "Undefined" => Rule::NonTerminal(Object::Undefined),
    "Rational" => Rule::NonTerminal(Object::Rational),
    "Casts" => Rule::NonTerminal(Object::Casts),
    other => panic!("{other}")
}}} impl Into<Symbol> for Rule {fn into(self) -> Symbol {return match self {
    Rule::NonTerminal(object) => Symbol::NonTerminal(object),
    Rule::Internal(code) => Symbol::Internal(code)
}}}

//> GRAMMAR -> SYMBOL
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Symbol {
    NonTerminal(Object),
    Internal(u8),
    Kind(Kind)
} impl From<&str> for Symbol {fn from(value: &str) -> Self {match value {
    internal if internal.starts_with("$") => Symbol::Internal(internal[1..].parse().unwrap()),
    "Start" => Symbol::NonTerminal(Object::Start),
    "Level1" => Symbol::NonTerminal(Object::Level1),
    "Level2" => Symbol::NonTerminal(Object::Level2),
    "Level3" => Symbol::NonTerminal(Object::Level3),
    "Level4" => Symbol::NonTerminal(Object::Level4),
    "Level5" => Symbol::NonTerminal(Object::Level5),
    "Declaration" => Symbol::NonTerminal(Object::Declaration),
    "Definition" => Symbol::NonTerminal(Object::Definition),
    "Annotation" => Symbol::NonTerminal(Object::Annotation),
    "Node" => Symbol::NonTerminal(Object::Node),
    "Equation" => Symbol::NonTerminal(Object::Equation),
    "Use" => Symbol::NonTerminal(Object::Use),
    "Expression" => Symbol::NonTerminal(Object::Expression),
    "Term" => Symbol::NonTerminal(Object::Term),
    "Factor" => Symbol::NonTerminal(Object::Factor),
    "Limit" => Symbol::NonTerminal(Object::Limit),
    "Infinite" => Symbol::NonTerminal(Object::Infinite),
    "Variable" => Symbol::NonTerminal(Object::Variable),
    "Nest" => Symbol::NonTerminal(Object::Nest),
    "Tensor" => Symbol::NonTerminal(Object::Tensor),
    "Whole" => Symbol::NonTerminal(Object::Whole),
    "Absolute" => Symbol::NonTerminal(Object::Absolute),
    "Undefined" => Symbol::NonTerminal(Object::Undefined),
    "Rational" => Symbol::NonTerminal(Object::Rational),
    "Casts" => Symbol::NonTerminal(Object::Casts),
    "IDENTIFIER" => Symbol::Kind(Kind::IDENTIFIER),
    "MODULE" => Symbol::Kind(Kind::MODULE),
    "NUMBER" => Symbol::Kind(Kind::NUMBER),
    "OPERATOR" => Symbol::Kind(Kind::OPERATOR),
    "COMMENT" => Symbol::Kind(Kind::COMMENT),
    "RATIONAL" => Symbol::Kind(Kind::RATIONAL),
    "SIGN" => Symbol::Kind(Kind::SIGN),
    "GROUP" => Symbol::Kind(Kind::GROUP),
    "BINDING" => Symbol::Kind(Kind::BINDING),
    "CLOSE" => Symbol::Kind(Kind::CLOSE),
    "COMMA" => Symbol::Kind(Kind::COMMA),
    "ENTER" => Symbol::Kind(Kind::ENTER),
    "EQUALITY" => Symbol::Kind(Kind::EQUALITY),
    "EXIT" => Symbol::Kind(Kind::EXIT),
    "EXPONENTIATION" => Symbol::Kind(Kind::EXPONENTIATION),
    "INFINITE" => Symbol::Kind(Kind::INFINITE),
    "LIMIT" => Symbol::Kind(Kind::LIMIT),
    "NEWLINES" => Symbol::Kind(Kind::NEWLINES),
    "OF" => Symbol::Kind(Kind::OF),
    "OPEN" => Symbol::Kind(Kind::OPEN),
    "PIPE" => Symbol::Kind(Kind::PIPE),
    "SPACES" => Symbol::Kind(Kind::SPACES),
    "TO" => Symbol::Kind(Kind::TO),
    "UNDEFINED" => Symbol::Kind(Kind::UNDEFINED),
    "USE" => Symbol::Kind(Kind::USE),
    "ENDOFFILE" => Symbol::Kind(Kind::ENDOFFILE),
    other => panic!("{other}")
}}}