//! This module contains most of the business logic required to run the
//! application and for error handling.
use std::env;
use std::error::Error;
use std::fs;
use std::process;

// Declare modules to make them available within this crate.
pub mod grammar;
pub mod random;

/// Main function which runs and controls the life time of the application.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // let lines = &contents.split("\n").collect::<Vec<&str>>();
    let parsed_rules = parse_file(&contents);
    if parsed_rules.len() == 0 {
        panic!("unable to parse file or file is empty");
    }
    let mut grammar = grammar::Grammar::new();
    grammar.change_start_nonterminal(&parsed_rules[0].left_hand);
    for rule in parsed_rules {
        grammar.rule_add_from_file(rule);
    }
    let unreachable = grammar.get_unreachable_nonterminals();
    // Use eprintln! so message does not get outputed to file if
    // client redirects stdout to file.
    match unreachable.len() {
        0 => eprintln!("Successful grammar rules. All non-terminals are reachable."),
        _ => eprintln!("Warning: Unreachable non-terminals: {:#?}", unreachable),
    }
    // println!("{:#?}", grammar.rules);
    eprintln!("Generating {} sentences.", config.quantity);
    let generated_sentences =
        grammar.generate_sentences(&grammar.start_nonterminal, config.quantity);
    let generated_paragraphs =
        convert_sentences_to_paragraphs(&generated_sentences, &config.paragraph_length);
    for paragraph in generated_paragraphs {
        println!("{}", paragraph);
    }
    Ok(())
}

/// Validates the command line arguments, and stores their values.
#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub quantity: i32,
    pub start_nonterminal: String,
    pub paragraph_length: i32,
}

impl Config {
    /// Given CLI arguments, parse and validate the arguments. Takes an iterator of
    /// the env::Args to explicitly describe the info in used for creation.
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // first arg not needed
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let quantity = match args.next() {
            None => return Err("Didn't get a sentence count"),
            // Raise error if 3rd argument is not an integer.
            Some(arg) => match arg.parse::<i32>() {
                Ok(i) => match i > 0 {
                    true => i,
                    false => return Err("Third argument must be positive integer."),
                },
                Err(_e) => return Err("Third argument was not an integer."),
            },
        };
        let paragraph_length = match args.next() {
            None => 1, // default
            // convert to int
            Some(arg) => match arg.parse::<i32>() {
                Ok(i) => match i > 0 {
                    true => i,
                    false => return Err("Fourth argument must be positive integer."),
                },
                // Raise error if 4th argument is not an integer
                Err(_e) => return Err("Fourth argument was not an integer."),
            },
        };

        Ok(Config {
            filename,
            quantity,
            paragraph_length,
            start_nonterminal: String::new(),
        })
    }
}

/// Converts generated sentences into paragraphs of given sentence length
pub fn convert_sentences_to_paragraphs(slice: &[String], length: &i32) -> Vec<String> {
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
pub fn parse_file<'a>(content: &'a str) -> Vec<Rule> {
    // would prefer to split up the logic somewhat, but I believe this is the most
    // performant way to reference the line number when errors arise.
    content
        .lines()
        .enumerate() // used for errors to get line number
        .filter(|(_, line)| !should_ignore_line(line))
        .map(|(line_num, line)| {
            Rule::new(line).unwrap_or_else(|err| {
                eprintln!("Problem parsing line {}: {}", line_num as i32, err);
                eprintln!("line's content is: '{}'", line);
                process::exit(1);
            })
        })
        .collect()
}

fn should_ignore_line(line: &str) -> bool {
    line.trim().is_empty() || line.trim().starts_with("//")
}

/// Represents a grammar rule formed from a single line in the file provided.
/// # Example
/// ```
/// use story_gen::Rule;
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
    /// Takes a line of the file following a specific notation,
    /// and parses the line, and applies the necessary transformations
    /// to convert it into a rule.
    pub fn new(line: &str) -> Result<Rule, &str> {
        let parsed = line.split("=").collect::<Vec<&str>>();
        if parsed.len() < 2 {
            return Err("line doesn't contain '=");
        }
        let left_hand = String::from(parsed[0].trim());
        let right_unparsed = String::from(parsed[1].trim());
        let temp_right_hand = parse_right_hand_side(&right_unparsed);
        let right_hand = process_rhs_optional_combinations(&temp_right_hand);
        Ok(Rule {
            left_hand,
            right_hand,
        })
    }
}

/// Converts the RHS vector into representing all possible combinations of
/// optional tokens. If there are no optional tokens, then this method
/// will return the same array.
///
/// If there are optional tokens, then the number of options that will
/// be generated is equal to 2^n, where n is the number of optional tokens.
pub fn process_rhs_optional_combinations(right_hand: &[String]) -> Vec<String> {
    let mut options: Vec<String> = vec![];
    for op in right_hand {
        let sub_units = grammar::parse_subunits(op);
        let mut perm = Combinations::new();
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
        options.extend(
            perm.options
                .iter()
                .map(|x| String::from(x.trim()))
                .collect::<Vec<String>>(),
        );
    }
    options
}

/// Used to encapsulate combinatorics logic for optional tokens.
#[derive(Debug)]
pub struct Combinations {
    pub options: Vec<String>,
}
impl Combinations {
    pub fn new() -> Combinations {
        Combinations {
            options: vec![String::from("")],
        }
    }
    /// Add an optional token to the existing options. Each call on this method
    /// increases the number of options on the RHS by a factor of 2.
    /// For example, if there are currently 4 options, and an optional
    /// token is added, then after this method is complete, 8 options will exist.
    pub fn add_optional(&mut self, optional: &str) {
        // for every optional token, it can either be added, or not added.
        // The 'modified' vec represents the times it's added.
        let modified: Vec<String> = self
            .options
            .iter()
            .map(|x| format!("{} {}", x, optional))
            .collect();
        self.options.extend(modified); // vector length is doubled
    }
    /// Add a required token to all existing options. Every option must
    /// include this token.
    pub fn add_required(&mut self, required: &str) {
        let modified: Vec<String> = self
            .options
            .iter()
            .map(|x| format!("{} {}", x, required))
            .collect();
        self.options = modified;
    }
}

/// Parses the string that that is to the rigth of the equal sign delimiter
/// # Examples:
/// ```
/// use story_gen::parse_right_hand_side;
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
