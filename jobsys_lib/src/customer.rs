use std::collections::HashMap;
use std::fmt;

use chrono::{DateTime, Local};
use inquirer_rs::Inquireable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{job::Job, vehicle::Vehicle, IdAble, PathAble};

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

    pub fn update_name(&mut self, name: Option<String>) {
        if let Some(n) = name {
            self.name = n;
            self.set_last_updated();
        }
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

impl Inquireable for Customer {
    type Item = Customer;
    fn inquire(_: Option<&str>) -> Option<Self::Item> {
        let name = String::inquire_retry_on_none(2, Some("Invalid String."), Some("Enter Name: "));
        let now = Local::now();
        let cust = Customer {
            id: Uuid::new_v4(),
            name: match name {
                Some(n) => n,
                None => "".to_owned(),
            },
            vehicles: HashMap::new(),
            jobs: HashMap::new(),
            date_created: now,
            last_updated: now,
        };
        Some(cust)
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
