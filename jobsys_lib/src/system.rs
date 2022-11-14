use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

use clap::Parser;
use inquirer_rs::helpers::{into_menu_string, inquire_string, inquire_menu};
use inquirer_rs::menu::InquireableMenu;
use uuid::Uuid;
use log::debug;
use anyhow::Result;

use crate::menu::{MainMenuChoices, EntityOptions};
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
  base_path: String,
  parsed_cli: Cli,
}

impl JobSys {
  pub fn new(base_path: String, parsed_cli: Cli, ) -> JobSys {
    let customers = JobSys::data_load(base_path.to_owned());
    JobSys {
      customers,
      base_path,
      parsed_cli,
    }
  }

  pub fn new_customer(&mut self) -> Result<()> {
    let name = inquire_string("Enter name: ".to_string())?;
    let customer = Customer::new(name);
    JobSys::log_insert_result(self.customers.insert(customer.get_id().to_owned(), customer));
    Ok(())
  }

  pub fn update_customer(&mut self) -> Result<()> {
    let id = self.get_customer_id();
    let customer = self.customers.get_mut(&id).unwrap();
    let name = inquire_string("Enter name: ".to_string())?;
    customer.update_name(name);
    Ok(())
  }

  pub fn get_customer_id(&mut self) -> Uuid {
    let choices: Vec<Customer> = self.customers.clone().into_values().collect();
    let display = into_menu_string(&choices, "Customers");
    let choice = inquire_menu(display, &choices); 
    choice.get_id()
  }

  pub fn new_vehicle(&mut self, vin_num: String, make: String, model: String, year: Option<String>) {
    let customer_id = self.get_customer_id();
    let vehicle = Vehicle::new(vin_num, make, model, year);
    self.customers.get_mut(&customer_id).unwrap().upsert_vehicle(vehicle);
  }

  pub fn new_job(&mut self, description: String) {
    let customer_id = self.get_customer_id();
    let job = Job::new(description);
    self.customers.get_mut(&customer_id).unwrap().upsert_job(job);
  }

  pub fn settings(&self) -> Result<()> {
    Err(JobSysError::NonImplementedMenuChoice)?
  }

  /// If this action fails the app should panic.
  pub fn data_save(&mut self) { 
    serde_yaml::to_writer(File::create(Path::new(&self.base_path).join(Customer::get_path())).unwrap(), &self.customers).unwrap();
  }

  /// If this action fails the app should panic.
  fn data_load(base_path: String) -> HashMap<Uuid, Customer> {
    let load_path = Path::new(&base_path).join(Customer::get_path());
    debug!("loading from path {:?}", load_path);
    serde_yaml::from_reader(File::open(load_path).unwrap()).unwrap()
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
    let mut main_menu_choice: MainMenuChoices = MainMenuChoices::inquire("Main Menu");

    while main_menu_choice != MainMenuChoices::Quit {
      let sub_menu_choice: EntityOptions = EntityOptions::inquire(&format!("{} Menu", main_menu_choice.to_string()));

      match main_menu_choice {
        MainMenuChoices::Jobs => {
          match sub_menu_choice {
            EntityOptions::New => todo!(),
            EntityOptions::Update => todo!(),
            EntityOptions::Delete => todo!(),
            EntityOptions::View => todo!(),
            EntityOptions::Back => todo!(),
        }
        },
        MainMenuChoices::Vehicles => {
          match sub_menu_choice {
            EntityOptions::New => todo!(),
            EntityOptions::Update => todo!(),
            EntityOptions::Delete => todo!(),
            EntityOptions::View => todo!(),
            EntityOptions::Back => todo!(),
        }
        },
        MainMenuChoices::Customers => {
          match sub_menu_choice {
            EntityOptions::New => {
              self.new_customer()?;
            },
            EntityOptions::Update => {
              self.update_customer()?
            },
            EntityOptions::Delete => todo!(),
            EntityOptions::View => todo!(),
            EntityOptions::Back => todo!(),
        }
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
        main_menu_choice = MainMenuChoices::inquire("Main Menu");
      }
    }

    Ok(())
  }
}
