use anyhow::Result;
use inquire::{validator::Validation, Text};
use log::debug;

pub mod helpers;
pub mod menu;

pub trait Inquireable {
    type Item;

    /// Inquire one time
    ///
    /// return Some(Item): if inquisition is successful.  
    ///
    /// return None: if inquisition is unsuccessful.
    fn inquire(prompt_label: &str) -> Result<Self::Item>;
}

impl Inquireable for String {
    type Item = String;
    fn inquire(prompt_label: &str) -> Result<Self::Item> {
        debug!("starting first try");

        let validator = |input: &str| -> Result<Validation, _> {
            if input.chars().count() > 5 {
                Ok(Validation::Invalid(
                    "You're only allowed 5 characters.".into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        };

        let result = Text::new(prompt_label).with_validator(validator).prompt()?;

        Ok(result)
    }
}
