use std::fmt;

use strum_macros::EnumIter;

use menu::Menuable;

#[derive(Debug, EnumIter, PartialEq, Clone)]
pub enum MainMenuChoices {
  Jobs,
  Vehicles,
  Customers,
  Settings,
  Quit,
}

impl Menuable for MainMenuChoices {}

impl fmt::Display for MainMenuChoices {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

#[derive(Debug, EnumIter, PartialEq, Clone)]
pub enum EntityOptions {
  New,
  Update,
  Delete,
  View,
  Back,
}

impl Menuable for EntityOptions {}

impl fmt::Display for EntityOptions {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}