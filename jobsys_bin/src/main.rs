use std::process;

use anyhow::Result;
use clap::Parser;
use log::{debug, error};

use jobsys_lib::system::{Cli, JobSys};

fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::builder()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    let mut jobsys = JobSys::new("".to_owned(), cli);

    match jobsys.run() {
        Ok(_) => {
            debug!("Completed Ok, Saving Data...");
            jobsys.data_save();
        }
        Err(err) => {
            error!("System Encountered an Error, Saving Data... msg: {err}");
            jobsys.data_save();
            process::exit(1);
        }
    }

    Ok(())
}
