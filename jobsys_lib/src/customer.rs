use std::fmt;

use chrono::{Local, DateTime};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::{vehicle::Vehicle, job::Job, PathAble, IdAble};

#[derive(Serialize, Deserialize)]
pub struct Customer {
  id: Uuid,
  name: String,
  vehicles: Vec<Vehicle>,
  jobs: Vec<Job>,
  date_created: DateTime<Local>,
  last_updated: DateTime<Local>,
}

impl Customer {
  pub fn new(name: String) -> Customer {
    let now = Local::now();
    Customer {
      id: Uuid::new_v4(),
      name,
      vehicles: Vec::new(),
      jobs: Vec::new(),
      date_created: now,
      last_updated: now,
    }
  }
}

impl PathAble for Customer {
  fn get_path() -> String {
    "customers.yml".to_owned()
  }
}

impl IdAble for Customer {
  fn get_id(&self) -> Uuid {
    self.id
  }
}

impl fmt::Display for Customer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "name: {}", self.name)
  }
}