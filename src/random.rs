//! Module that provides random helper functions that are used in
//! the application.
use rand::seq::SliceRandom;

/// Uses uniform distribution to select a random element from the
/// provided string slice, and return a copy of the String value.
pub fn get_random_from_vector(slice: &[String]) -> String {
  let mut rng = rand::thread_rng();
  if let Some(randomly_picked) = slice.choose(&mut rng) {
    String::from(randomly_picked)
  } else {
    String::new()
  }
}
