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
use crate::menu::CustomerOptions;
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
        match Customer::inquire_retry_on_none(2, None, None) {
            Ok(cust) => {
                JobSys::log_insert_result(self.customers.insert(cust.get_id().to_owned(), cust));
                Ok(())
            }
            Err(e) => Err(JobSysError::FailedToCreateNewCustomer)?,
        }
    }

    pub fn inquire_customer_mut<'a>(&'a mut self) -> Result<&'a mut Customer> {
        let choices: Vec<Customer> = self.customers.clone().into_values().collect();
        let display = into_menu_string(&choices, "Customers");
        let choice = inquire_menu(display, &choices)?;
        Ok(self.customers.get_mut(&choice.get_id()).unwrap())
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

        let customer = self.inquire_customer_mut()?;

        let mut customer_option: CustomerOptions = CustomerOptions::inquire("Customer Menu")?;

        // Once a customer is selected, use them for all other operations.
        while customer_option != CustomerOptions::Back {
            match customer_option {
                CustomerOptions::ViewJobs => customer.display_jobs()?,
                CustomerOptions::AddJob => customer.new_job()?,
                CustomerOptions::UpdateJob => todo!(),
                CustomerOptions::ViewVehicles => customer.display_vehicles()?,
                CustomerOptions::AddVehicle => customer.new_vehicle()?,
                CustomerOptions::UpdateVehicle => todo!(),
                CustomerOptions::UpdateName => customer.update_name()?,
                CustomerOptions::Back => todo!(),
            }
            customer_option = CustomerOptions::inquire("Customer Menu")?;
        }

        Ok(())
    }
}
