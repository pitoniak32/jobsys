use std::collections::HashMap;
use std::fmt;

use anyhow::Result;
use chrono::{DateTime, Local};
use inquirer_rs::{
    helpers::{inquire_menu, into_menu_string},
    Inquireable,
};
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

    pub fn update_name(&mut self) -> Result<()> {
        let name = String::inquire_retry_on_none(
            2,
            Some("Invalid Customer Please Try Again."),
            Some("Enter name: "),
        )?;
        self.name = name;
        self.set_last_updated();
        Ok(())
    }

    pub fn display_vehicles(&self) -> Result<()> {
        for (_id, vehicle) in &self.vehicles {
            println!("{:#?}", vehicle);
        }
        Ok(())
    }

    pub fn get_vehicles(&self) -> &HashMap<Uuid, Job> {
        &self.jobs
    }

    /// If the vehicle is not in the list it will be added
    /// If the vehicle is in the list it will be updated.
    pub fn upsert_vehicle(&mut self, vehicle: Vehicle) {
        self.vehicles.insert(vehicle.get_id(), vehicle);
        self.set_last_updated();
    }

    pub fn new_vehicle(&mut self) -> Result<()> {
        let vehicle =
            Vehicle::inquire_retry_on_none(2, Some("Invalid Vehicle Please Try Again."), None)?;
        self.upsert_vehicle(vehicle);
        Ok(())
    }

    pub fn display_jobs(&self) -> Result<()> {
        for (_id, job) in &self.jobs {
            println!("{:#?}", job);
        }
        Ok(())
    }

    pub fn get_jobs(&self) -> &HashMap<Uuid, Job> {
        &self.jobs
    }

    /// If the job is not in the list it will be added
    /// If the job is in the list it will be updated.
    pub fn upsert_job(&mut self, job: Job) {
        self.jobs.insert(job.get_id(), job);
        self.set_last_updated();
    }

    pub fn new_job(&mut self) -> Result<()> {
        // Create a new job.
        let mut job = Job::inquire(None)?;

        // Ask the user to pick the vehicle the job is for.
        let vehicle_choices: Vec<Vehicle> = self.vehicles.clone().into_values().collect();
        let vehicle_menu = into_menu_string(&vehicle_choices, "");
        let choice = inquire_menu(vehicle_menu, &vehicle_choices)?;

        // Update the job to be on the vehicle the user chooses.
        job.set_vehicle_id(choice.get_id());

        // Insert the job.
        self.upsert_job(job);
        Ok(())
    }

    fn set_last_updated(&mut self) {
        self.last_updated = Local::now();
    }
}

impl Inquireable for Customer {
    type Item = Customer;
    fn inquire(_: Option<&str>) -> Result<Self::Item> {
        let now = Local::now();
        let cust = Customer {
            id: Uuid::new_v4(),
            name: String::inquire_retry_on_none(2, Some("Invalid String."), Some("Enter Name: "))?,
            vehicles: HashMap::new(),
            jobs: HashMap::new(),
            date_created: now,
            last_updated: now,
        };
        Ok(cust)
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
