use std::collections::HashMap;
use std::fmt;

use chrono::{Local, DateTime};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::{vehicle::Vehicle, job::Job, PathAble, IdAble};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Customer {
  id: Uuid,
  name: String,
  vehicles: HashMap<Uuid, Vehicle>,
  jobs: HashMap<Uuid, Job>,
  date_created: DateTime<Local>,
  last_updated: DateTime<Local>,
}

impl Customer {
  pub fn new(name: String) -> Customer {
    let now = Local::now();
    Customer {
      id: Uuid::new_v4(),
      name,
      vehicles: HashMap::new(),
      jobs: HashMap::new(),
      date_created: now,
      last_updated: now,
    }
  }

  pub fn update_name(&mut self, name: String) {
    self.name = name;
    self.set_last_updated();
  }

  /// If the vehicle is not in the list it will be added
  /// If the vehicle is in the list it will be updated.
  pub fn upsert_vehicle(&mut self, vehicle: Vehicle) {
    self.vehicles.insert(vehicle.get_id(), vehicle);
    self.set_last_updated();
  }

  /// If the job is not in the list it will be added
  /// If the job is in the list it will be updated.
  pub fn upsert_job(&mut self, job: Job) {
    self.jobs.insert(job.get_id(), job);
    self.set_last_updated();
  }

  pub fn get_vehicles(&self) -> &HashMap<Uuid, Job> {
    &self.jobs
  }

  pub fn get_jobs(&self) -> &HashMap<Uuid, Job> {
    &self.jobs
  }

  fn set_last_updated(&mut self) {
    self.last_updated = Local::now();
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
    write!(f, "{}", self.name)
  }
}