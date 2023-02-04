use std::process;

use anyhow::Result;
use clap::Parser;
use inquire::{validator::Validation, Text};
use log::{debug, error};

use jobsys_lib::system::{Cli, JobSys};

fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::builder()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    let validator = |input: &str| -> Result<Validation, _> {
        if input.chars().count() > 5 {
            Ok(Validation::Invalid(
                "You're only allowed 5 characters.".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    let status = Text::new("What are you thinking about?")
        .with_validator(validator)
        .prompt();

    match status {
        Ok(status) => println!("Your status is being published..."),
        Err(err) => println!("Error while publishing your status: {}", err),
    }

    // let mut jobsys = JobSys::new("".to_owned(), cli);
    //
    // match jobsys.run() {
    //     Ok(_) => {
    //         debug!("Completed Ok, Saving Data...");
    //         jobsys.data_save();
    //     }
    //     Err(err) => {
    //         error!("System Encountered an Error, Saving Data... msg: {}", err);
    //         jobsys.data_save();
    //         process::exit(1);
    //     }
    // }

    Ok(())
}
