// use story_graph::components::{actor, location, world};
// use story_graph::grammar::noun::noun;
use std::env;
// use std::error::Error;
use std::process;
use story_graph::Config;

// use crate::location;
// use story_graph::utils;

fn main() {
  // build_random_sentences(10);
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
  eprintln!("Generating {} sentences.", config.quantity);
  eprintln!("In file {}", config.filename);
  eprintln!("Config: {:#?}", config);
  if let Err(e) = story_graph::run(config) {
    eprintln!("Application error: {}", e);
    process::exit(1);
  }
}

// fn print_sound<T: Animal>(t: &T) {
//   println!("The animal goes {}", t.make_sound());
// }
