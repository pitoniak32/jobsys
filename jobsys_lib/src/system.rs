use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use anyhow::Result;
use clap::Parser;
use inquirer_rs::helpers::{inquire_menu, into_menu_string};
use inquirer_rs::menu::InquireableMenu;
use inquirer_rs::Inquireable;
use log::debug;
use uuid::Uuid;

use crate::customer::Customer;
use crate::error::JobSysError;
use crate::job::Job;
use crate::menu::{EntityOptions, MainMenuChoices};
use crate::vehicle::Vehicle;
use crate::{IdAble, PathAble};

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
    pub fn new(base_path: String, parsed_cli: Cli) -> JobSys {
        let customers = JobSys::data_load(base_path.to_owned());
        JobSys {
            customers,
            base_path,
            parsed_cli,
        }
    }

    pub fn new_customer(&mut self) -> Result<()> {
        if let Some(cust) = Customer::inquire_retry_on_none(2, None, None) {
            JobSys::log_insert_result(self.customers.insert(cust.get_id().to_owned(), cust));
            return Ok(());
        }
        Err(JobSysError::FailedToCreateNewCustomer)?
    }

    pub fn update_customer(&mut self) -> Result<()> {
        let id = self.inquire_customer_id();
        let customer = self.customers.get_mut(&id).unwrap();
        let name = String::inquire_retry_on_none(
            2,
            Some("Invalid Customer Please Try Again."),
            Some("Enter name: "),
        );
        customer.update_name(name);
        Ok(())
    }

    pub fn inquire_customer_id(&mut self) -> Uuid {
        let choices: Vec<Customer> = self.customers.clone().into_values().collect();
        let display = into_menu_string(&choices, "Customers");
        let choice = inquire_menu(display, &choices);
        choice.get_id()
    }

    pub fn new_vehicle(&mut self) -> Result<()> {
        let customer_id = self.inquire_customer_id();
        if let Some(v) =
            Vehicle::inquire_retry_on_none(2, Some("Invalid Vehicle Please Try Again."), None)
        {
            self.customers
                .get_mut(&customer_id)
                .unwrap()
                .upsert_vehicle(v);
            return Ok(());
        }
        Err(JobSysError::FailedToCreateNewVehicle)?
    }

    pub fn new_job(&mut self, description: String) {
        let customer_id = self.inquire_customer_id();
        let job = Job::new(description);
        self.customers
            .get_mut(&customer_id)
            .unwrap()
            .upsert_job(job);
    }

    pub fn settings(&self) -> Result<()> {
        Err(JobSysError::NonImplementedMenuChoice)?
    }

    /// If this action fails the app should panic.
    pub fn data_save(&mut self) {
        serde_yaml::to_writer(
            File::create(Path::new(&self.base_path).join(Customer::get_path())).unwrap(),
            &self.customers,
        )
        .unwrap();
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
            match main_menu_choice {
                MainMenuChoices::Jobs => {
                    match EntityOptions::inquire(&format!("{} Menu", main_menu_choice.to_string()))
                    {
                        EntityOptions::New => todo!(),
                        EntityOptions::Update => todo!(),
                        EntityOptions::Delete => todo!(),
                        EntityOptions::View => todo!(),
                        EntityOptions::Back => todo!(),
                    }
                }
                MainMenuChoices::Vehicles => {
                    match EntityOptions::inquire(&format!("{} Menu", main_menu_choice.to_string()))
                    {
                        EntityOptions::New => self.new_vehicle()?,
                        EntityOptions::Update => todo!(),
                        EntityOptions::Delete => todo!(),
                        EntityOptions::View => todo!(),
                        EntityOptions::Back => todo!(),
                    }
                }
                MainMenuChoices::Customers => {
                    match EntityOptions::inquire(&format!("{} Menu", main_menu_choice.to_string()))
                    {
                        EntityOptions::New => {
                            self.new_customer()?;
                        }
                        EntityOptions::Update => self.update_customer()?,
                        EntityOptions::Delete => todo!(),
                        EntityOptions::View => todo!(),
                        EntityOptions::Back => todo!(),
                    }
                }
                MainMenuChoices::Save => self.data_save(),
                MainMenuChoices::Settings => {
                    debug!("settings");
                    self.settings()?
                }
                MainMenuChoices::Quit => {
                    debug!("quitting");
                }
            };

            if main_menu_choice != MainMenuChoices::Quit {
                main_menu_choice = MainMenuChoices::inquire("Main Menu");
            }
        }

        Ok(())
    }
}
