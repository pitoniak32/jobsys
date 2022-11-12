use clap::Parser;
use menu::{data::{JobSys, Cli}, run};

use log::{error, debug};
use std::process;


fn main() {
    let cli = Cli::parse();

    env_logger::builder().filter_level(cli.verbose.log_level_filter()).init();

    let base_path = "..".to_owned();

    let mut job_sys = JobSys::new(base_path);

    let result = run(cli, &mut job_sys);

    match result {
        Ok(_) => {
            debug!("Completed Ok... Saving Data...");
            job_sys.data_save();
            process::exit(0)
        },
        Err(err) => {
            error!("Error: {}", err);
            job_sys.data_save();
            process::exit(1);
        },
    }
}

