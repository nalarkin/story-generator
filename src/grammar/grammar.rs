use crate::utils;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Grammar {
  pub rules: HashMap<String, Vec<String>>,
}
impl Grammar {
  pub fn add_rule(&mut self, key: &str, value: &str) {
    // let parsed_value = format!("<|>{}<|>", value);
    let mut parsed = vec![];
    // let value_copy = String::from(value);
    let split: Vec<&str> = value.split("|").collect();
    for word in &split {
      parsed.push(String::from(word.trim()));
    }
    // let str_split = Vec::from(split);
    self.rules.insert(key.to_string(), parsed);
  }
  pub fn delete_rule(&mut self, key: &str) {
    self.rules.remove(key);
  }
  // recursive call, if key doesn't exist, it's a token, so return that string,
  // otherwise evaulate RHS
  pub fn build_random(&self, key: &str) -> String {
    if let Some(options) = self.rules.get(key) {
      // split value
      // for each element in split value, find it's value in
      let random_choice = utils::get_random_from_vector(&options);
      // println!("random choice: {}", &random_choice);
      let sub_choices = self.parse_options(&random_choice);
      let mut built_sentence = String::new();
      for token in sub_choices {
        built_sentence.push_str(&self.build_random(&token));
      }
      built_sentence
    } else {
      // options[0].clone()
      format!(" {}", key)
    }
  }
  // fn is_nonterminal(&self, )
  fn parse_options(&self, option: &str) -> Vec<String> {
    let mut all_options = vec![];
    let options = option.split_whitespace().collect::<Vec<&str>>();
    for possible in options {
      all_options.push(String::from(possible.trim()));
    }
    all_options
  }
}
