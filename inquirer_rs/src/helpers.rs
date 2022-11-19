use anyhow::Result;

use crate::{error::InquisitionError, Inquireable};

pub fn into_menu_string<T: std::fmt::Display>(choices: &Vec<T>, title: &str) -> String {
    let mut display = String::new();

    display.push_str("\n");
    display.push_str(title);
    display.push_str("\n");
    display.push_str("---");
    display.push_str("\n");

    let mut i: usize = 1;
    for choice in choices {
        display.push_str(&format!("{}) {}\n", i, choice));
        i += 1;
    }

    display
}

pub fn inquire_menu<T: Clone + std::fmt::Display + std::fmt::Debug>(
    display_menu: String,
    choices: &Vec<T>,
) -> Result<T> {
    loop {
        println!("{}", display_menu);

        let result = String::inquire(Some("Enter Choice: "))?;

        match result.parse::<usize>() {
            Ok(n) => match choices.get(n - 1).cloned() {
                Some(res) => return Ok(res),
                None => Err(InquisitionError::MenuChoiceError(
                    format!("Invalid Menu Choice: {}", n).to_string(),
                ))?,
            },
            Err(e) => Err(InquisitionError::MenuChoiceError(
                format!(
                    "Failed Parsing Invalid Choice Value: {}, Error: {}",
                    result, e
                )
                .to_string(),
            ))?,
        }
    }
}
