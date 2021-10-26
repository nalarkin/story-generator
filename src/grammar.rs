use crate::utils;
use crate::*;
use std::collections::HashMap;

/// This struct is used to manage and store the grammar rules. This is the main structure
/// the client will interface with.
#[derive(Debug)]
pub struct Grammar {
  pub rules: HashMap<String, Vec<String>>,
  pub validation: HashMap<String, i32>,
  pub start_nonterminal: String,
  pub validator: Validator,
}
impl Grammar {
  pub fn new() -> Grammar {
    Grammar {
      rules: Default::default(),
      validation: Default::default(),
      start_nonterminal: Default::default(),
      validator: Validator::new(),
    }
  }
  /// Add the rule with LHS non-terminal 'key' and the RHS non-terminals
  /// and/or terminals. Could be used in future for interactive console deletion.
  pub fn rule_add(&mut self, key: &str, value: &str) {
    let parsed: Vec<String> = value
      .split("|")
      .collect::<Vec<&str>>()
      .iter()
      .map(|&x| String::from(x.trim()))
      .collect();
    // get mutable access to value associated with key, guarding against the key
    // possibly not being set
    let right_hand_side = self.rules.entry(key.to_string()).or_insert(vec![]);
    for option in parsed {
      right_hand_side.push(option);
    }
  }

  pub fn change_start_nonterminal(&mut self, new_value: &str) {
    self.start_nonterminal = String::from(new_value);
  }

  /// Add a rule struct into the existing grammar rules. If no matching LHS
  /// currently matches, it will create a new grammar rule. If a matching LHS
  /// already exists, it will extend the options of the existing respective RHS
  /// to include the RHS of the provided rule.
  pub fn rule_add_from_file(&mut self, rule: Rule) {
    let key = self.rules.entry(rule.left_hand).or_insert(vec![]);
    key.extend(rule.right_hand);
  }

  /// Delete the rule with LHS non-terminal 'key'. Prints a success or error
  /// message based on whether the rule existed before the deletion.
  /// Could be used in future for interactive console deletion.
  pub fn rule_delete(&mut self, key: &str) {
    if let Some(value_removed) = self.rules.remove(key) {
      println!("Removed key: {} , value: {:#?}", key, value_removed)
    } else {
      println!("Error. Could not find rule with key: '{}'", key)
    }
  }

  /// Generate random sentences starting from LHS non-termianal 'key'
  pub fn generate_sentences(&self, key: &str, count: i32) -> Vec<String> {
    let mut sentences = vec![];
    for _ in 0..count {
      sentences.push(self.generate_sentence(key));
    }
    sentences
  }

  /// Generate a single random sentence from provided non-terminal.
  pub fn generate_sentence(&self, key: &str) -> String {
    let unformatted_sentence = self.build_random(key);
    let mut trimmed_sentence = String::from(unformatted_sentence.trim());
    trimmed_sentence.push_str(".");
    // capitalize first letter in the sentence.
    let mut c = trimmed_sentence.chars();
    match c.next() {
      None => String::new(),
      Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
  }
  /// recursive call, if key doesn't exist, it must be a token, so return
  /// that string, otherwise evaulate RHS
  pub fn build_random(&self, key: &str) -> String {
    if let Some(options) = self.rules.get(key) {
      let random_choice = utils::get_random_from_vector(options);
      let sub_choices = parse_subunits(&random_choice);
      let mut built_sentence = String::new();
      for token in sub_choices {
        built_sentence.push_str(&self.build_random(&token));
      }
      built_sentence
    } else {
      format!(" {}", key)
    }
  }

  pub fn get_unreachable_nonterminals(&mut self) -> Vec<String> {
    self
      .validator
      .get_unreachable_nonterminals(&self.rules, &self.start_nonterminal)
  }

  /// Validates the grammer rules have at least 1 valid path. See Validator
  /// struct for more detailed info on validation procedure.
  pub fn validate(&mut self) -> Result<(), String> {
    self.validator.validate(&self.rules)
  }
}

/// Executes all grammar validation logic and stores results in
/// the HashMap 'validation' attribute.
#[derive(Debug)]
pub struct Validator {
  pub validation: HashMap<String, i32>,
}

impl Validator {
  pub fn new() -> Validator {
    Validator {
      validation: Default::default(),
    }
  }

  /// Validates the grammar rules to ensure there are no cycles that are
  /// gauranteed to go infinitely. Each non-terminal is valid if there is at
  /// least 1 valid path for the non-terminal, and each non-terminal.
  /// Note: Recursive rules and cycles are still allowed.
  /// # Examples
  /// ```
  /// // the below is valid
  /// // <noun> = <adj> | <noun>
  /// // <adj> = happy
  ///
  /// // the below is valid
  /// // <noun> = <adj> <noun> | <ending>
  /// // <adj> = <noun> | <adj>
  /// // <ending> = abc
  ///
  /// // the following is not valid
  /// // <noun> = <verb>
  /// // <verb> = <noun>
  /// ```
  pub fn validate(&mut self, rules: &HashMap<String, Vec<String>>) -> Result<(), String> {
    // self.reset_validation(rules);
    let mut validation_map = self.reset_validation(rules);
    for key in rules.keys() {
      match dfs(key, rules, &mut validation_map) {
        true => validation_map.insert(String::from(key), Status::SAFE),
        false => validation_map.insert(String::from(key), Status::UNSAFE),
      };
    }
    self.validation = validation_map;
    let unsafe_keys = get_unsafe_keys(&self.validation);
    // if no unsafe keys, then validation is successful
    match unsafe_keys.len() {
      0 => Ok(()),
      _ => {
        let error_str = format!("unsafe non-terminals: {}", unsafe_keys.join(" "));
        return Err(error_str);
      }
    }
  }

  fn reset_validation(&mut self, rules: &HashMap<String, Vec<String>>) -> HashMap<String, i32> {
    let validation = rules
      .keys()
      .map(|key| (String::from(key), Status::UNVISITED))
      .collect();
    validation
  }

  /// Traverses the grammar rules from the starting non-terminal
  /// to find the non-teminals that cannot be reached. Returns a
  /// vector of non-terminals which cannot be reached. If all
  /// non-terminals are reachable, returns a vector with length 0.
  pub fn get_unreachable_nonterminals(
    &mut self,
    rules: &HashMap<String, Vec<String>>,
    key: &str,
  ) -> Vec<String> {
    self.validation = self.reset_validation(rules);
    find_reachable(key, rules, &mut self.validation);
    let unreachable_keys: Vec<String> = self
      .validation
      .iter()
      .filter(|(_, &val)| val != Status::SAFE)
      .map(|(key, _)| String::from(key))
      .collect();

    unreachable_keys
  }
}

/// Used for graph coloring during traversal required for validation methods.
#[non_exhaustive]
struct Status;
impl Status {
  pub const UNVISITED: i32 = 0;
  pub const VISITING: i32 = 1;
  pub const UNSAFE: i32 = 2;
  pub const SAFE: i32 = 3;
}

/// Recursive depth first search which marks nodes (non-terminal) as safe
/// it all sub options are safe. Otherwise, mark the node as unvisited.
fn dfs(
  node: &str,
  graph: &HashMap<String, Vec<String>>,
  status: &mut HashMap<String, i32>,
) -> bool {
  match status.get(node) {
    None => true,
    Some(&Status::SAFE) => true,
    Some(&Status::UNSAFE) => false,
    Some(&Status::VISITING) => false, // occurs when there is a cycle
    Some(&Status::UNVISITED) => {
      status.insert(String::from(node), Status::VISITING);
      let mut is_safe = false;
      if let Some(right_hand_side_options) = graph.get(node) {
        for option in right_hand_side_options {
          let parsed = parse_subunits(option);
          let valid_options: Vec<bool> = parsed
            .iter()
            .map(|sub_option| dfs(&sub_option, graph, status))
            .collect();
          let is_valid_option = valid_options.iter().all(sub_option_is_safe);
          if is_valid_option {
            // used to set the LHS as true if at least 1 of the options in the RHS is safe.
            is_safe = true;
          }
        }
      }
      if is_safe {
        status.insert(String::from(node), Status::SAFE);
      } else {
        // necessary to mark as unvisited instead of unsafe. This is because cycles
        // are allowed, but only 1 valid path is necessary to validate the non-terminal.
        // this node might be valid from another path.
        status.insert(String::from(node), Status::UNVISITED);
      }
      is_safe
    }
    _ => false, // removes non_exhaustive erorr, but we know that value can only be 0-3
  }
}

/// Finds the non-terminals which are not reachable from the
/// original starting non-terminal which creates all sentences.
/// Stores it's findings in the status hash map that is provided.
fn find_reachable(
  node: &str,
  graph: &HashMap<String, Vec<String>>,
  status: &mut HashMap<String, i32>,
) {
  match status.get(node) {
    Some(&Status::SAFE) => (),
    Some(&Status::UNVISITED) => {
      status.insert(String::from(node), Status::SAFE);
      if let Some(options) = graph.get(node) {
        // println!("node:{}, all options: {:#?}", node, options);
        for option in options {
          let parsed = parse_subunits(option);
          for sub_option in parsed {
            // println!("node {} calling dfs on neighbor {}", node, sub_option);
            find_reachable(&sub_option, graph, status);
          }
        }
      }
    }
    None => (),
    Some(_) => (),
  }
}

/// Used to find all components of an option. When a right-hand-side option has multiple
/// components
/// # Example
/// ```
/// use story_graph::grammar::parse_subunits;
/// let example = parse_subunits("<id> <noun>");
/// assert_eq!(example, vec!["<id>", "<noun>"]);
/// ```
pub fn parse_subunits(option: &str) -> Vec<String> {
  let possible_options: Vec<String> = option
    .split_whitespace()
    .map(|possible_option| String::from(possible_option.trim()))
    .collect();
  possible_options
}
/// Simply a function to be more explicit about what is occuring when called
fn sub_option_is_safe(sub_option: &bool) -> bool {
  sub_option == &true
}

fn get_unsafe_keys(status: &HashMap<String, i32>) -> Vec<String> {
  let unsafe_keys: Vec<String> = status
    .iter()
    .filter(|(_, value)| value == &&Status::UNSAFE)
    .map(|(key, _)| key.to_string())
    .collect();
  unsafe_keys
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashMap;

  #[test]
  fn test_validation_1() {
    let mut grammar = Grammar::new();

    let rule_1 = Rule::new("<sentence> = <vp> | np").unwrap_or_default();
    let rule_2 = Rule::new("<vp> = noun").unwrap_or_default();
    let rule_3 = Rule::new("noun = <vp>").unwrap_or_default();
    grammar.rule_add_from_file(rule_1);
    grammar.rule_add_from_file(rule_2);
    grammar.rule_add_from_file(rule_3);

    let unsafe_keys = grammar.validate().unwrap_err();
    let mut expected = HashMap::new();
    expected.insert(String::from("<sentence>"), 3);
    expected.insert(String::from("<vp>"), 2);
    expected.insert(String::from("noun"), 2);
    assert!(unsafe_keys.contains("<vp>"));
    assert!(unsafe_keys.contains("noun"));
    assert_eq!(grammar.validator.validation, expected);
  }
  #[test]
  fn test_validation_2() {
    let mut grammar = Grammar::new();
    grammar.rule_add("1", "1 | 2");
    grammar.rule_add("2", "3");
    grammar.validate().unwrap();
    let mut expected = HashMap::new();
    expected.insert(String::from("1"), 3);
    expected.insert(String::from("2"), 3);
    assert_eq!(grammar.validator.validation, expected);
  }
  #[test]
  fn test_validation_3() {
    let mut grammar = Grammar::new();
    grammar.rule_add("1", "1 | 2");
    grammar.rule_add("2", "1 | 3");
    grammar.validate().unwrap();
    let mut expected = HashMap::new();
    expected.insert(String::from("1"), 3);
    expected.insert(String::from("2"), 3);
    assert_eq!(grammar.validator.validation, expected);
  }

  #[test]
  fn test_parse_subunits() {
    let example = parse_subunits("<id> <noun>");
    assert_eq!(example, vec!["<id>", "<noun>"]);
  }

  #[test]
  fn test_find_reachable_1() {
    let mut graph = HashMap::new();
    let mut status = HashMap::new();
    graph.insert("a".to_string(), vec!["a".to_string(), "b".to_string()]);
    graph.insert("b".to_string(), vec!["c".to_string(), "d".to_string()]);
    for (key, _) in &graph {
      status.insert(key.to_string(), Status::UNVISITED);
    }
    find_reachable("a", &graph, &mut status);
    let mut expected = HashMap::new();
    expected.insert("a".to_string(), Status::SAFE);
    expected.insert("b".to_string(), Status::SAFE);
    assert_eq!(status, expected);
  }
  #[test]
  fn test_find_reachable_2() {
    let mut graph = HashMap::new();
    let mut status = HashMap::new();
    graph.insert("a".to_string(), vec!["d".to_string(), "e".to_string()]);
    graph.insert("b".to_string(), vec!["c".to_string(), "d".to_string()]);
    for (key, _) in &graph {
      status.insert(key.to_string(), Status::UNVISITED);
    }
    find_reachable("a", &graph, &mut status);
    let mut expected = HashMap::new();
    expected.insert("a".to_string(), Status::SAFE);
    expected.insert("b".to_string(), Status::UNVISITED);
    assert_eq!(status, expected);
  }
}
