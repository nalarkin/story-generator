#[derive(Debug, Clone)]
pub struct Location {
  pub name: String,
  pub id: i32,
}

pub fn build_location(name: String, id: i32) -> Location {
  Location { name, id }
}

impl Default for Location {
  fn default() -> Location {
    Location {
      name: Default::default(),
      id: Default::default(),
    }
  }
}
