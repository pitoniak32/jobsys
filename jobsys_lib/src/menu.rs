use strum_macros::EnumIter;
use menu_derive::EnumMenuable;

#[derive(Debug, EnumIter, PartialEq, Clone, EnumMenuable)]
pub enum MainMenuChoices {
  Jobs,
  Vehicles,
  Customers,
  Settings,
  Quit,
}

#[derive(Debug, EnumIter, PartialEq, Clone, EnumMenuable)]
pub enum EntityOptions {
  New,
  Update,
  Delete,
  View,
  Back,
}