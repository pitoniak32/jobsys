use log::debug;
use anyhow::Result;
use std::fs::File;
use std::collections::HashMap;
use uuid::Uuid;
use jobsys_error::JobSysError;
use std::path::Path;

use clap::Parser;
use job_entities::{customer::Customer, vehicle::Vehicle, job::Job, IdAble, PathAble};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

pub struct JobSys {
  customers: HashMap<Uuid, Customer>,
  vehicles: HashMap<Uuid, Vehicle>,
  jobs: HashMap<Uuid, Job>,
  base_path: String,
}

impl JobSys {
  pub fn new(base_path: String) -> JobSys {
    let (customers, vehicles, jobs) = JobSys::data_load(base_path.to_owned());
    JobSys {
      customers,
      vehicles,
      jobs,
      base_path,
    }
  }

  pub fn new_customer(&mut self, name: String) -> Uuid {
    let customer = Customer::new(name);
    let id = customer.get_id().to_owned();
    JobSys::log_insert_result(self.customers.insert(customer.get_id().to_owned(), customer));
    return id;
  }

  pub fn new_vehicle(&mut self, vin_num: String, make: String, model: String, year: Option<String>) -> Uuid {
    let vehicle = Vehicle::new(vin_num, make, model, year);
    let id = vehicle.get_id().to_owned();
    JobSys::log_insert_result(self.vehicles.insert(vehicle.get_id(), vehicle));
    return id;
  }

  pub fn new_job(&mut self, description: String) -> Uuid {
    let job = Job::new(description);
    let id = job.get_id().to_owned();
    JobSys::log_insert_result(self.jobs.insert(job.get_id(), job));
    return id;
  }

  pub fn settings(&self) -> Result<()> {
    Err(JobSysError::NonImplementedMenuChoice)?
  }

  /// If this action fails the app should panic.
  pub fn data_save(&mut self) { 
    serde_yaml::to_writer(File::create(Path::new(&self.base_path).join(Customer::get_path())).unwrap(), &self.customers).unwrap();
    serde_yaml::to_writer(File::create(Path::new(&self.base_path).join(Vehicle::get_path())).unwrap(), &self.vehicles).unwrap();
    serde_yaml::to_writer(File::create(Path::new(&self.base_path).join(Job::get_path())).unwrap(), &self.jobs).unwrap();
  }

  /// If this action fails the app should panic.
  fn data_load(base_path: String) -> (HashMap<Uuid, Customer>, HashMap<Uuid, Vehicle>, HashMap<Uuid, Job>) {
    (
      serde_yaml::from_reader(File::open(Path::new(&base_path).join(Customer::get_path())).unwrap()).unwrap(),
      serde_yaml::from_reader(File::open(Path::new(&base_path).join(Vehicle::get_path())).unwrap()).unwrap(),
      serde_yaml::from_reader(File::open(Path::new(&base_path).join(Job::get_path())).unwrap()).unwrap(),
    )
  }

  fn log_insert_result(insert_result: Option<impl IdAble>) {
    if let Some(result) = insert_result {
      debug!("updated at id: {}", result.get_id());
    } else {
      debug!("inserted new record");
    }
  }
}
