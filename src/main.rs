//! Main module that collects the command line arguments and
//! intiates the application.

use std::env;
use std::process;
use story_gen::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = story_gen::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
