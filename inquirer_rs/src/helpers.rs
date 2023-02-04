use anyhow::Result;
use inquire::{Select, InquireError};

pub fn inquire_menu<T: Clone + std::fmt::Display + std::fmt::Debug>(
    title: &str,
    choices: Vec<T>,
) -> Result<T, InquireError> {
    Select::new(title, choices).prompt()
}
