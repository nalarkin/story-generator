use crate::components::location;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Actor {
  pub name: String,
  pub id: i32,
  pub location: Option<location::Location>,
  pub locations: Vec<location::Location>,
  pub map_locations: HashMap<i32, location::Location>,
  pub entry_time: i32,
  pub life_time: i32,
  pub members: HashMap<i32, Actor>,
  pub parent_id: i32,
}

impl Actor {
  pub fn add_location(&mut self, location: &location::Location) {
    let test_id = location.id;
    self.locations.push(location.to_owned());
    self.map_locations.insert(test_id, location.to_owned());
  }
  pub fn set_entry_time(&mut self, time: i32) {
    self.entry_time = time;
  }
  pub fn has_member(&self, &member_id: &i32) -> bool {
    self.members.contains_key(&member_id)
  }
}

impl Default for Actor {
  fn default() -> Actor {
    Actor {
      name: Default::default(),
      id: Default::default(),
      entry_time: Default::default(),
      life_time: Default::default(),
      parent_id: Default::default(),
      locations: Default::default(),
      map_locations: Default::default(),
      location: Some(location::Location::default()),
      members: Default::default(),
    }
  }
}
