use story_graph::components::{actor, location, world};
use story_graph::grammar::noun::noun;
use story_graph::grammar::{grammar, sentence};
// use crate::location;
use story_graph::utils;

trait Animal {
  fn make_sound(&self) -> String;
}

struct Cat {
  sound: String,
}
impl Animal for Cat {
  fn make_sound(&self) -> String {
    (&self.sound).to_string()
  }
}
struct Dog {
  sound: String,
  color: String,
}
impl Animal for Dog {
  fn make_sound(&self) -> String {
    format!("{} {}", &self.sound, &self.color)
    // (&self.sound).to_string() + &self.color
  }
}

fn main() {
  // println!("Hello, world!");
  let tester = location::build_location(String::from("Paris"), 49);
  // println!("{:#?}", tester);
  // utils::get_random_in_range(1,20);
  let mut world = world::build_world(String::from("Nathan"), 49);
  // println!("{:#?}", world);
  world.add_location(&tester);
  // let mut george = actor::Actor {
  //   name: String::from("George"),
  //   id: 69,
  //   parent_id: 69,
  //   life_time: 420,
  //   ..Default::default()
  // };
  // let sally = actor::Actor {
  //   name: String::from("sally"),
  //   id: 22,
  //   parent_id: 69,
  //   life_time: 420,
  //   ..Default::default()
  // };
  // let test_sentence = sentence::Sentence {
  //   noun_phrase: sentence::NounPhrase {},
  //   verb_phrase: sentence::VerbPhrase {},
  // };
  // println!("{}", test_sentence.build());
  // let cat_test = Cat {
  //   sound: String::from("Meow"),
  // };
  // let dog_test = Dog {
  //   sound: String::from("Woof"),
  //   color: String::from("Yellow"),
  // };
  // // println!("{}", cat_test.make_sound());
  // print_sound(&cat_test);
  // print_sound(&dog_test);
  // println!("{}", dog_test.make_sound());
  let mut grammar = grammar::Grammar {
    rules: Default::default(),
  };
  grammar.add_rule("<sentence>", "<np> <vp>");
  grammar.add_rule("<np>", "<T> | <noun>");
  grammar.add_rule("<vp>", "<np> <verb> <noun>");
  grammar.add_rule("<noun>", "dog | cat");
  grammar.add_rule("<verb>", "jumped");
  grammar.add_rule("<T>", "the");
  // println!("{:#?}", grammar);
  // grammar.build_random("<noun>");
  // grammar.build_random("<noun>");
  let tester = grammar.build_random("<np>");
  println!("{}", tester);
  let tester2 = grammar.build_random("<sentence>");
  println!("{}", tester2);
  build_random_sentences(10);
}

fn print_sound<T: Animal>(t: &T) {
  println!("The animal goes {}", t.make_sound());
}

fn build_random_sentences(number_to_generate: i32) {
  let mut grammar = grammar::Grammar {
    rules: Default::default(),
  };
  grammar.add_rule("<sentence>", "<np> <vp>");
  grammar.add_rule("<np>", "<T> <noun> | <T> <adj> <noun>");
  grammar.add_rule("<vp>", "<verb> <T> <noun> | <verb> <T> <adj> <noun>");
  grammar.add_rule("<noun>", "dog | cat");
  grammar.add_rule("<verb>", "jumped | saw | hugged");
  grammar.add_rule("<T>", "the | a");
  grammar.add_rule("<adj>", "red | blue | yellow | green | purple");
  let mut arr = vec![];
  for _ in 0..number_to_generate {
    let random = grammar.build_random("<sentence>");
    let formatted = format!("{}", random.trim());
    // println!("{}", random);
    arr.push(formatted);
  }
  println!("{:#?}", arr);
}
