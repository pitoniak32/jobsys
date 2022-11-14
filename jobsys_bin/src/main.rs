use std::process;

use clap::Parser;
use log::{error, debug};

use jobsys_lib::{system::{JobSys, Cli}, menu::MainMenuChoices}; 

fn main() {
    let cli = Cli::parse();

    env_logger::builder().filter_level(cli.verbose.log_level_filter()).init();

    println!("{}", MainMenuChoices::Customers);

    let mut jobsys = JobSys::new("".to_owned(), cli);

    match jobsys.run() {
        Ok(_) => {
            debug!("Completed Ok... Saving Data...");
            jobsys.data_save();
        },
        Err(err) => {
            error!("Error: {}", err);
            jobsys.data_save();
            process::exit(1);
        },
    }
}

