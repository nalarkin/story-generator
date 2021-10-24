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
#[derive(Debug)]
pub struct Config {
  pub filename: String,
  pub quantity: i32,
  pub start_nonterminal: String,
  pub paragraph_length: i32,
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
    let mut paragraph_length = 1;
    if args.len() == 4 {
      paragraph_length = match args[3].clone().parse::<i32>() {
        Ok(i) => i,
        Err(_e) => return Err("fourth argument was not an integer"),
      }
    }

    Ok(Config {
      filename,
      quantity,
      start_nonterminal: String::new(),
      paragraph_length,
    })
  }
}

/// Main function which runs and controls the life time of the application.
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
  let unreachable = grammar.get_unreachable_nonterminals();
  match unreachable.len() {
    0 => eprintln!("No unreachable non-terminals."),
    _ => eprintln!("Warning: Unreachable non-terminals: {:#?}", unreachable),
  }

  let generated_sentences = grammar.generate_sentences(&grammar.start_nonterminal, config.quantity);
  let generated_paragraphs =
    convert_sentences_to_paragraphs(&generated_sentences, &config.paragraph_length);
  for paragraph in generated_paragraphs {
    println!("{}", paragraph);
  }
  Ok(())
}

/// Converts generated sentences into paragraphs of given sentence length
fn convert_sentences_to_paragraphs(slice: &Vec<String>, length: &i32) -> Vec<String> {
  let mut paragraphs = vec![];
  let mut idx = 0;
  while idx < slice.len() {
    let mut count = 0;
    let mut single_paragraph = vec![];
    while count + idx < slice.len() && count < *length as usize {
      single_paragraph.push(slice[count + idx].clone());
      count += 1;
    }
    idx += count;
    paragraphs.push(single_paragraph.join(" "));
  }
  paragraphs
}

/// Convert lines from a file into grammar rules
/// Program exits if any lines do not follow the rules listed in the README.md
fn parse_file(lines: &Vec<&str>) -> Vec<Rule> {
  let mut rules = vec![];
  for line in lines.iter() {
    if !should_ignore_line(line) {
      let built_rule = Rule::new(&line).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
      });
      rules.push(built_rule);
    }
  }
  rules
}

fn should_ignore_line(line: &&str) -> bool {
  line.trim().starts_with("//")
}

#[derive(Debug, Default)]
/// Represents a grammar rule formed from a single line in the file provided.
/// # Example
/// ```
/// use story_graph::Rule;
///
/// let example = Rule::new("noun = cat | dog").unwrap_or(Rule::default());
/// assert_eq!(example.left_hand, "noun");
/// assert_eq!(example.right_hand, vec!["cat", "dog"]);    
///
/// let example_failure = Rule::new("noun cat | dog").unwrap_or(Rule::default());
/// let expected = Rule::default();
/// assert_eq!(example_failure.left_hand, expected.left_hand);
/// assert_eq!(example_failure.right_hand, expected.right_hand);
/// ```
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

/// Parses the string that that is to the rigth of the equal sign delimiter
/// # Examples:
/// ```
/// use story_graph::parse_right_hand_side;
/// assert_eq!(parse_right_hand_side("bag | dog"), vec!["bag", "dog"]);
/// assert_eq!(parse_right_hand_side("bag | dog cat"),vec!["bag", "dog cat"]);
/// ```
pub fn parse_right_hand_side(rhs: &str) -> Vec<String> {
  let parsed: Vec<String> = rhs
    .split("|")
    .collect::<Vec<&str>>()
    .iter()
    .map(|x| String::from(x.trim()))
    .collect();
  parsed
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn parse_rhs_simple() {
    assert_eq!(parse_right_hand_side("bag | dog"), vec!["bag", "dog"]);
  }
  #[test]
  fn parse_rhs() {
    assert_eq!(
      parse_right_hand_side("bag | dog cat"),
      vec!["bag", "dog cat"]
    );
  }
  #[test]
  fn test_rule() {
    let example = Rule::new("noun = cat | dog").unwrap_or(Rule::default());
    assert_eq!(example.left_hand, "noun");
    assert_eq!(example.right_hand, vec!["cat", "dog"]);
  }
  #[test]
  fn test_rule_fails() {
    let example_failure = Rule::new("noun cat | dog").unwrap_or(Rule::default());
    let expected = Rule::default();
    assert_eq!(example_failure.left_hand, example_failure.left_hand);
    assert_eq!(example_failure.right_hand, expected.right_hand);
  }
}
