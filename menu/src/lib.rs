use anyhow::Result;
use log::debug;

pub mod data;
pub mod structs;

use data::{JobSys, Cli};
use structs::{MainMenuChoices, EntityOptions};

use crate::structs::Choosable;

pub fn run(parsed_cli: Cli, job_sys: &mut JobSys) -> Result<()> {
  debug!("log verbosity: {}", parsed_cli.verbose);
  let mut main_menu_choice = MainMenuChoices::get_choice(MainMenuChoices::Quit);

  while main_menu_choice != MainMenuChoices::Quit {

    match main_menu_choice {
      MainMenuChoices::Jobs=> {
        debug!("creating job");
        let _id = job_sys.new_job("".to_owned());
      },
      MainMenuChoices::Vehicles => {
        debug!("creating vehicle");
        let sub_menu_choice = EntityOptions::get_choice(EntityOptions::Back);
        let _id = job_sys.new_vehicle("".to_owned(), "".to_owned(), "".to_owned(), Some("2022".to_owned()));
      },
      MainMenuChoices::Customers => {
        debug!("creating customer");
        let _id = job_sys.new_customer("test_name".to_owned());
      },
      MainMenuChoices::Settings => {
        debug!("settings");
        job_sys.settings()?
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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
