use inquirer_derive::EnumMenuable;
use strum_macros::EnumIter;
use smart_default::SmartDefault;

#[derive(Debug, EnumIter, PartialEq, Clone, SmartDefault, EnumMenuable)]
pub enum MainMenuChoices {
  Customers,
  Jobs,
  Vehicles,
  Settings,
  Save,

  #[default]
  Quit,
}

#[derive(Debug, EnumIter, PartialEq, Clone, SmartDefault, EnumMenuable)]
pub enum EntityOptions {
  New,
  Update,
  Delete,
  View,

  #[default]
  Back,
}