use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
// use rand;
pub fn get_random_in_range(start: i32, end: i32) -> i32 {
  let mut rng = rand::thread_rng();
  let die = Uniform::from(start..end);
  let throw = die.sample(&mut rng);
  return throw;

  // loop {
  //     let throw = die.sample(&mut rng);
  //     println!("Roll the die: {}", throw);
  //     if throw == 6 {
  //         break;
  //     }
  // }
}
pub fn get_random_from_vector(slice: &std::vec::Vec<String>) -> String {
  let mut rng = rand::thread_rng();
  if let Some(randomly_picked) = slice.choose(&mut rng) {
    format!("{}", randomly_picked)
  } else {
    String::new()
  }
}
// pub fn get_random_from_vector(slice: &[usize]) -> &str {
//   let mut rng = rand::thread_rng();
//   format!("{:?}", slice.choose(&mut rng))
// }
