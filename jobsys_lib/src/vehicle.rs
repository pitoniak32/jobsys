use std::fmt;

use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{Local, DateTime};

use crate::{IdAble, PathAble};

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
  id: Uuid,
  vin_num: String,
  make: String,
  model: String,
  year: Option<String>,
  last_oil_change: Option<DateTime<Local>>,
  date_created: DateTime<Local>,
  last_updated: DateTime<Local>,
}

impl Vehicle {
  pub fn new(vin_num: String, make: String, model: String, year: Option<String>) -> Vehicle {
    let now = Local::now();
    Vehicle {
      id: Uuid::new_v4(),
      vin_num,
      make,
      model,
      year,
      last_oil_change: None,
      date_created: now,
      last_updated: now,
    }
  }
}

impl IdAble for Vehicle {
  fn get_id(&self) -> Uuid {
    self.id
  }
}

impl PathAble for Vehicle {
  fn get_path() -> String {
    "vehicles.yml".to_owned()
  }
}

impl fmt::Display for Vehicle {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "vin: ({})", self.vin_num)
  }
}