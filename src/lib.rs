use std::error::Error;
use std::fs;
use std::process;

// Declare modules to make them available within this crate.
pub mod grammar;
pub mod utils;

/// Validates the command line arguments, and stores their values.
#[derive(Debug)]
pub struct Config {
  pub filename: String,
  pub quantity: i32,
  pub start_nonterminal: String,
  pub paragraph_length: i32,
}

impl Config {
  /// Given CLI arguments, parse and validate the arguments.
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let filename = args[1].clone();
    // Raise error if 3rd argument is not an integer.
    let quantity = match args[2].clone().parse::<i32>() {
      Ok(i) => i,
      Err(_e) => return Err("third argument was not an integer"),
    };
    let mut paragraph_length = 1;
    // Raise error if 4th argument is not an integer
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
  let mut grammar = grammar::Grammar::new();
  grammar.change_start_nonterminal(&parsed_rules[0].left_hand);
  for rule in parsed_rules {
    grammar.rule_add_from_file(rule);
  }
  let unreachable = grammar.get_unreachable_nonterminals();
  match unreachable.len() {
    0 => eprintln!("Successful grammar rules. No unreachable non-terminals."),
    _ => eprintln!("Warning: Unreachable non-terminals: {:#?}", unreachable),
  }
  println!("{:#?}", grammar.rules);
  let generated_sentences = grammar.generate_sentences(&grammar.start_nonterminal, config.quantity);
  let generated_paragraphs =
    convert_sentences_to_paragraphs(&generated_sentences, &config.paragraph_length);
  for paragraph in generated_paragraphs {
    println!("{}", paragraph);
  }
  Ok(())
}

/// Converts generated sentences into paragraphs of given sentence length
fn convert_sentences_to_paragraphs(slice: &[String], length: &i32) -> Vec<String> {
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
fn parse_file(lines: &[&str]) -> Vec<Rule> {
  let mut rules = vec![];
  for (line_num, line) in lines.iter().enumerate() {
    if !should_ignore_line(line) {
      let built_rule = Rule::new(&line).unwrap_or_else(|err| {
        eprintln!("Problem parsing line {}: {}", line_num as i32, err);
        eprintln!("line's content is: '{}'", line);
        process::exit(1);
      });
      rules.push(built_rule);
    }
  }
  rules
}

fn should_ignore_line(line: &&str) -> bool {
  line.trim().is_empty() || line.trim().starts_with("//")
}

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
#[derive(Debug, Default)]
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
    // let right_hand = parse_right_hand_side(&right_unparsed);
    let temp_right_hand = parse_right_hand_side(&right_unparsed);
    let right_hand = process_rhs(&temp_right_hand);
    // println!(
    //   "options before: {:?} options after: {:?}",
    //   right_hand, updated_options
    // );
    Ok(Rule {
      left_hand,
      right_hand,
    })
  }
}
fn process_rhs(right_hand: &[String]) -> Vec<String> {
  let mut options: Vec<String> = vec![];
  for op in right_hand {
    let sub_units = grammar::parse_subunits(op);
    let mut perm = Permuations::new();
    for s in sub_units {
      let trimmed = s.trim();
      if trimmed.starts_with('(') && trimmed.ends_with(')') {
        let mut without_paren = trimmed.chars();
        without_paren.next(); // remove opening parenth
        without_paren.next_back(); // remove closing parenth
        perm.add_optional(without_paren.as_str());
      } else {
        perm.add_required(trimmed);
      }
    }
    // println!("rhs: {:?} rules: {:#?}", right_hand, perm);
    options.extend(
      perm
        .options
        .iter()
        .map(|x| String::from(x.trim()))
        .collect::<Vec<String>>(),
    );
    // println!("rhs: {:?} rules: {:#?}", right_hand, perm);
  }
  options
  // for
}

#[derive(Debug)]
pub struct Permuations {
  pub options: Vec<String>,
}
impl Permuations {
  pub fn new() -> Permuations {
    Permuations {
      options: Default::default(),
    }
  }
  pub fn add_optional(&mut self, optional: &str) {
    if self.options.len() == 0 {
      self.options.push(String::from(""));
      self.options.push(String::from(optional));
    } else {
      let modified: Vec<String> = self
        .options
        .iter()
        .map(|x| format!("{} {}", x, optional))
        .collect();
      self.options.extend(modified);
    }
  }
  pub fn add_required(&mut self, required: &str) {
    if self.options.len() == 0 {
      self.options.push(String::from(required))
    } else {
      let modified: Vec<String> = self
        .options
        .iter()
        .map(|x| format!("{} {}", x, required))
        .collect();
      self.options = modified;
    }
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
