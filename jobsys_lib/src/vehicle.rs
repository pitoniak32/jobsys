use std::fmt;

use anyhow::Result;
use inquirer_rs::helpers::{inquire_string, inquire_year};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};

use crate::IdAble;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vehicle {
  id: Uuid,
  vin_num: String,
  make: String,
  model: String,
  year: Option<i32>,
  last_oil_change: Option<DateTime<Utc>>,
  date_created: DateTime<Utc>,
  last_updated: DateTime<Utc>,
}

impl Vehicle {
  pub fn new(vin_num: String, make: String, model: String, year: Option<i32>) -> Vehicle {
    let now = Utc::now();
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

  pub fn inquire() -> Result<Vehicle> {
    let vin_num = inquire_string("Enter vin number: ")?;
    let make = inquire_string("Enter make: ")?;
    let model = inquire_string("Enter model: ")?;
    let year = inquire_year();
    Ok(Vehicle::new(vin_num, make, model, year))
  }
}

impl IdAble for Vehicle {
  fn get_id(&self) -> Uuid {
    self.id
  }
}

impl fmt::Display for Vehicle {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "vin: ({})", self.vin_num)
  }
}