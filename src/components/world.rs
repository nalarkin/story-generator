use crate::components::location;
use std::collections::HashMap;

#[derive(Debug)]
pub struct World {
  pub name: String,
  pub id: i32,
  pub locations: Vec<location::Location>,
  pub map_locations: HashMap<i32, location::Location>,
}

impl World {
  pub fn add_location(&mut self, location: &location::Location) {
    let test_id = location.id;
    self.locations.push(location.to_owned());
    self.map_locations.insert(test_id, location.to_owned());
  }
}

pub fn build_world(name: String, id: i32) -> World {
  World {
    name,
    id,
    locations: vec![],
    map_locations: HashMap::new(),
  }
}
