use crate::utils;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Grammar {
  pub rules: HashMap<String, Vec<String>>,
  pub validation: HashMap<String, i32>,
}
impl Grammar {
  pub fn new() -> Grammar {
    Grammar {
      rules: Default::default(),
      validation: Default::default(),
    }
  }
  pub fn rule_add(&mut self, key: &str, value: &str) {
    let mut parsed = vec![];
    let split: Vec<&str> = value.split("|").collect();
    for word in split {
      parsed.push(String::from(word.trim()));
    }
    // update a key, guarding against the key possibly not being set
    let right_hand_side = self.rules.entry(key.to_string()).or_insert(vec![]);
    for option in parsed {
      right_hand_side.push(option);
    }
  }
  pub fn rule_delete(&mut self, key: &str) {
    if let Some(value_removed) = self.rules.remove(key) {
      println!("Removed key: {} , value: {:#?}", key, value_removed)
    } else {
      println!("Error. Could not find rule with key: '{}'", key)
    }
  }

  pub fn generate_sentences(&self, key: &str, count: i32) -> Vec<String> {
    let mut sentences = vec![];
    for _ in 0..count {
      sentences.push(self.generate_sentence(key));
    }
    sentences
  }

  pub fn generate_sentence(&self, key: &str) -> String {
    // TODO: Add verification that each key has at least 1 path to a terminal
    let unformatted_sentence = &self.build_random(key);
    let mut trimmed_sentence = String::from(unformatted_sentence.trim());
    trimmed_sentence.push_str(".");
    // capitalize first letter
    let mut c = trimmed_sentence.chars();
    match c.next() {
      None => String::new(),
      Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
  }
  /// recursive call, if key doesn't exist, it's a token, so return that string,
  /// otherwise evaulate RHS
  pub fn build_random(&self, key: &str) -> String {
    if let Some(options) = self.rules.get(key) {
      // split value
      // for each element in split value, find it's value in
      let random_choice = utils::get_random_from_vector(options);
      let sub_choices = self.parse_selected_choice(&random_choice);
      let mut built_sentence = String::new();
      for token in sub_choices {
        built_sentence.push_str(&self.build_random(&token));
      }
      built_sentence
    } else {
      format!(" {}", key)
    }
  }
  // fn is_nonterminal(&self, )
  fn parse_selected_choice(&self, option: &str) -> Vec<String> {
    let mut all_options = vec![];
    let options = option.split_whitespace().collect::<Vec<&str>>();
    for possible in options {
      all_options.push(String::from(possible.trim()));
    }
    all_options
  }

  fn reset_validation(&mut self) {
    self.validation = Default::default();
    for key in self.rules.keys() {
      self.validation.insert(String::from(key), Status::UNVISITED);
    }
  }

  pub fn validate(&mut self) -> Vec<String> {
    (&mut *self).reset_validation();
    let mut t = self.validation.clone();
    for key in (&self).rules.keys() {
      match dfs(key, &self.rules, &mut t) {
        true => t.insert(String::from(key), Status::SAFE),
        false => t.insert(String::from(key), Status::UNSAFE),
      };
    }
    self.validation = t;
    let unsafe_keys = get_unsafe_keys(&self.validation);
    unsafe_keys
  }
}

#[non_exhaustive]
struct Status;

impl Status {
  pub const UNVISITED: i32 = 0;
  pub const VISITING: i32 = 1;
  pub const UNSAFE: i32 = 2;
  pub const SAFE: i32 = 3;
}
// TODO: Fix Validation
// Currently having weird bug where sometimes it validates test case 3, sometimes it
// does the validation incorrectly. My guess is that the hashmap order is affecting the
// validation dfs traversal.
fn dfs(
  node: &str,
  graph: &HashMap<String, Vec<String>>,
  status: &mut HashMap<String, i32>,
) -> bool {
  match status.get(node) {
    None => true,
    Some(&Status::SAFE) => true,
    Some(&Status::UNSAFE) => false,
    Some(&Status::VISITING) => false,
    Some(&Status::UNVISITED) => {
      // println!("================");
      status.insert(String::from(node), Status::VISITING);
      // println!("status: {:#?}", status);
      let mut is_safe = false;
      if let Some(options) = graph.get(node) {
        // println!("node:{}, all options: {:#?}", node, options);
        for option in options {
          let parsed = parse_subunits(option);
          let mut validity_check: Vec<bool> = vec![];
          for sub_option in parsed {
            // println!("node {} calling dfs on neighbor {}", node, sub_option);
            validity_check.push(dfs(&sub_option, graph, status));
          }
          // println!("current option: {}", option);
          // println!("validity_check: {:#?}", validity_check);
          // let is_valid_option = all_sub_options_are_safe(validity_check.clone());
          // let is_valid_option = validity_check.iter().all(|e| e == &true);
          let is_valid_option = validity_check.iter().all(sub_option_is_safe);
          if is_valid_option {
            is_safe = true;
          }
        }
      }
      if is_safe {
        // println!("setting node {} to {}", node, Status::SAFE);
        status.insert(String::from(node), Status::SAFE);
      } else {
        // println!("setting node {} to {}", node, Status::UNVISITED);
        // status.insert(String::from(node), Status::UNSAFE);
        status.insert(String::from(node), Status::UNVISITED);
      }
      is_safe
    }
    _ => false,
  }
}

fn parse_subunits(option: &str) -> Vec<String> {
  let mut all_options = vec![];
  let options = option.split_whitespace().collect::<Vec<&str>>();
  for possible in options {
    all_options.push(String::from(possible.trim()));
  }
  all_options
}
/// Simply a function to be more explicit about what is occuring when called
fn sub_option_is_safe(sub_option: &bool) -> bool {
  sub_option == &true
}

// fn all_sub_options_are_safe(slice: Vec<bool>) -> bool {
//   let mut all_is_valid = true;
//   for value in slice {
//     if !value {
//       all_is_valid = false;
//     }
//   }
//   all_is_valid
// }

fn get_unsafe_keys(status: &HashMap<String, i32>) -> Vec<String> {
  let mut response = vec![];
  for (key, value) in status {
    if value == &Status::UNSAFE {
      response.push(String::from(key));
    };
  }
  response
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashMap;

  #[test]
  fn test_validation_1() {
    let mut grammar = Grammar::new();
    grammar.rule_add("<sentence>", "<vp> | np");
    grammar.rule_add("<vp>", "noun");
    grammar.rule_add("noun", "<vp>");
    let unsafe_keys = grammar.validate();
    let mut expected = HashMap::new();
    expected.insert(String::from("<sentence>"), 3);
    expected.insert(String::from("<vp>"), 2);
    expected.insert(String::from("noun"), 2);
    // assert!(unsafe_keys.contains(String::from("<vp>")));
    // assert!(unsafe_keys.contains("<vp>".to_string()));
    assert!(unsafe_keys.iter().any(|e| e == "<vp>"));
    assert!(unsafe_keys.iter().any(|e| e == "noun"));
    assert_eq!(unsafe_keys.len(), 2);
    // assert!(unsafe_keys.contains("<vp>"));
    assert_eq!(grammar.validation, expected);
  }
  #[test]
  fn test_validation_2() {
    let mut grammar = Grammar::new();
    grammar.rule_add("1", "1 | 2");
    grammar.rule_add("2", "3");
    let unsafe_keys = grammar.validate();
    let mut expected = HashMap::new();
    expected.insert(String::from("1"), 3);
    expected.insert(String::from("2"), 3);
    assert_eq!(unsafe_keys.len(), 0);
    assert_eq!(grammar.validation, expected);
  }
  #[test]
  fn test_validation_3() {
    let mut grammar = Grammar::new();
    grammar.rule_add("1", "1 | 2");
    grammar.rule_add("2", "1 | 3");
    let unsafe_keys = grammar.validate();
    let mut expected = HashMap::new();
    expected.insert(String::from("1"), 3);
    expected.insert(String::from("2"), 3);
    assert_eq!(unsafe_keys.len(), 0);
    assert_eq!(grammar.validation, expected);
  }
}
