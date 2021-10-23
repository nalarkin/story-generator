use std::error::Error;
use std::fs;
use std::process;

// Special file to declare file structure
pub mod components {
  pub mod actor;
  pub mod location;
  pub mod world;
}
pub mod grammar {
  pub mod grammar;
  pub mod sentence;
  pub mod noun {
    pub mod noun;
  }
}

pub mod utils;
// pub mod world;

pub struct Config {
  pub filename: String,
  pub quantity: i32,
  pub start_nonterminal: String,
}
impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let filename = args[1].clone();
    let quantity = match args[2].clone().parse::<i32>() {
      Ok(i) => i,
      Err(_e) => return Err("third argument was not an integer"),
    };

    Ok(Config {
      filename,
      quantity,
      start_nonterminal: String::new(),
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  let lines = &contents.split("\n").collect::<Vec<&str>>();
  let parsed_rules = parse_file(&lines);
  if parsed_rules.len() == 0 {
    panic!("unable to parse file or file is empty");
  }
  let mut grammar = grammar::grammar::Grammar::new();
  grammar.change_start_nonterminal(&parsed_rules[0].left_hand);
  for rule in parsed_rules {
    grammar.rule_add_from_file(rule);
  }
  // println!("Grammar: {:#?}", grammar);
  let generated_sentences = grammar.generate_sentences(&grammar.start_nonterminal, config.quantity);
  println!("{:?}", generated_sentences);
  Ok(())
}

fn parse_file(lines: &Vec<&str>) -> Vec<Rule> {
  let mut rules = vec![];
  for line in lines.iter() {
    let built_rule = Rule::new(&line).unwrap_or_else(|err| {
      println!("Problem parsing arguments: {}", err);
      process::exit(1);
    });
    rules.push(built_rule);
  }
  rules
}
#[derive(Debug)]
pub struct Rule {
  pub left_hand: String,
  pub right_hand: Vec<String>,
}

impl Rule {
  pub fn new(line: &str) -> Result<Rule, &str> {
    let parsed = line.split("=").collect::<Vec<&str>>();
    if parsed.len() < 2 {
      return Err("line doesn't contain '=");
    }
    let left_hand = String::from(parsed[0].trim());
    let right_unparsed = String::from(parsed[1].trim());
    let right_hand = parse_right_hand_side(&right_unparsed);
    Ok(Rule {
      left_hand,
      right_hand,
    })
  }
}
fn parse_right_hand_side(rhs: &str) -> Vec<String> {
  let parsed: Vec<String> = rhs
    .split("|")
    .collect::<Vec<&str>>()
    .iter()
    .map(|x| String::from(x.trim()))
    .collect();
  parsed
}
