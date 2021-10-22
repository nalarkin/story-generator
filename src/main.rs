// use story_graph::components::{actor, location, world};
// use story_graph::grammar::noun::noun;
use std::collections::HashMap;
use story_graph::grammar::grammar;

// use crate::location;
// use story_graph::utils;

fn main() {
  // build_random_sentences(10);
  // build_random_sentences(10);
}

// fn print_sound<T: Animal>(t: &T) {
//   println!("The animal goes {}", t.make_sound());
// }

fn build_random_sentences(number_to_generate: i32) {
  let mut grammar = grammar::Grammar::new();
  grammar.rule_add("<sentence>", "<np> <vp>");
  grammar.rule_add("<np>", "<T> <noun> | <T> <adj> <noun>");
  grammar.rule_add("<vp>", "<verb> <T> <noun> | <verb> <T> <adj> <noun>");
  grammar.rule_add("<noun>", "dog | cat");
  grammar.rule_add("<verb>", "jumped | saw | hugged");
  grammar.rule_add("<T>", "the | a");
  grammar.rule_add("<adj>", "red | blue | yellow | green | purple");
  grammar.rule_add("<noun>", "bird");
  println!("{:#?}", grammar);
  // let mut arr = vec![];
  // for _ in 0..number_to_generate {
  //   let built_sentence = grammar.generate_sentence("<sentence>");
  //   // let formatted = format!("{}", random.trim());
  //   // println!("{}", random);
  //   arr.push(built_sentence);
  // }
  // println!("{:#?}", arr);
  // let joined = arr.join(" ");
  // println!("{}", joined);
  // grammar.rule_delete("this key doesn't exist");
  // grammar.rule_delete("<adj>");
  let built = grammar.generate_sentences("<sentence>", number_to_generate);
  // grammar.validate();
  // println!("{:#?}", built);
}
