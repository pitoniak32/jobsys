use log::debug;
use anyhow::Result;
use menu::{MainMenuChoices, Choosable, EntityOptions};
use std::fs::File;
use std::collections::HashMap;
use uuid::Uuid;

use std::path::Path;

use clap::Parser;

use crate::{IdAble, PathAble};
use crate::customer::Customer;
use crate::error::JobSysError;
use crate::job::Job;
use crate::vehicle::Vehicle;

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
  parsed_cli: Cli,
}

impl JobSys {
  pub fn new(base_path: String, parsed_cli: Cli, ) -> JobSys {
    let (customers, vehicles, jobs) = JobSys::data_load(base_path.to_owned());
    JobSys {
      customers,
      vehicles,
      jobs,
      base_path,
      parsed_cli,
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

  pub fn run(&mut self) -> Result<()> {
    debug!("log verbosity: {}", self.parsed_cli.verbose);
    let mut main_menu_choice = MainMenuChoices::get_choice(MainMenuChoices::Quit);

    while main_menu_choice != MainMenuChoices::Quit {

      match main_menu_choice {
        MainMenuChoices::Jobs=> {
          debug!("creating job");
          let _id = self.new_job("".to_owned());
        },
        MainMenuChoices::Vehicles => {
          debug!("creating vehicle");
          let _sub_menu_choice = EntityOptions::get_choice(EntityOptions::Back);
          let _id = self.new_vehicle("".to_owned(), "".to_owned(), "".to_owned(), Some("2022".to_owned()));
        },
        MainMenuChoices::Customers => {
          debug!("creating customer");
          let _id = self.new_customer("test_name".to_owned());
        },
        MainMenuChoices::Settings => {
          debug!("settings");
          self.settings()?
        },
        MainMenuChoices::Quit => {
          debug!("quitting");
        },
      };

      if main_menu_choice != MainMenuChoices::Quit {
        debug!("getting next choice from user");
        main_menu_choice = MainMenuChoices::get_choice(MainMenuChoices::Quit);
        debug!("Menu choice: {:?}", main_menu_choice);
      }
    }

    Ok(())
  }
}
