use inquirer_derive::EnumMenuable;
use smart_default::SmartDefault;
use strum_macros::EnumIter;

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
pub enum CustomerOptions {
    ViewJobs,
    AddJob,
    UpdateJob,
    ViewVehicles,
    AddVehicle,
    UpdateVehicle,
    UpdateName,

    #[default]
    Back,
}
