use std::env;
use std::process;
use story_graph::Config;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
  // Use eprintln! so message does not get outputed to file if
  // client redirects stdout to file.
  eprintln!("Generating {} sentences.", config.quantity);
  if let Err(e) = story_graph::run(config) {
    eprintln!("Application error: {}", e);
    process::exit(1);
  }
}
