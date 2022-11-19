use std::io::Write;

use anyhow::Result;
use chrono::NaiveDate;
use error::InquisitionError;
use log::debug;

mod error;
pub mod helpers;
pub mod menu;

pub trait Inquireable {
    type Item;

    /// Inquire one time
    ///
    /// return Some(Item): if inquisition is successful.  
    ///
    /// return None: if inquisition is unsuccessful.
    fn inquire(prompt_label: Option<&str>) -> Result<Self::Item>;

    /// Inquire once, and if `None` is returned, Inquire `retries` times before returning `None`  
    ///
    /// for a total of `retries + 1` inquisitions.
    fn inquire_retry_on_none(
        retries: u32,
        retry_msg: Option<&str>,
        prompt_label: Option<&str>,
    ) -> Result<Self::Item> {
        debug!("starting first try");
        let first_try = Self::inquire(prompt_label);

        if let Ok(ft) = first_try {
            debug!("first try Ok");
            return Ok(ft);
        }

        debug!("retry");

        for i in 0..retries {
            if let Some(rtry) = retry_msg {
                println!("\nRetry: {}", rtry);
                println!("Retries Left: {}", retries - (i + 1));
                if retries - (i + 1) == 0 {
                    println!("\nLast Try!");
                }
                println!("")
            }

            if let Ok(val) = Self::inquire(prompt_label) {
                return Ok(val);
            }
        }
        Err(InquisitionError::FailedRetry)?
    }
}

impl Inquireable for String {
    type Item = String;
    fn inquire(prompt_label: Option<&str>) -> Result<Self::Item> {
        let mut s = String::new();

        if let Some(lbl) = prompt_label {
            print!("{}", lbl);
        }

        if let Ok(_) = std::io::stdout().flush() {
            if let Ok(_) = std::io::stdin().read_line(&mut s) {
                return Ok(s.trim().to_owned());
            }
        }

        Err(InquisitionError::String)?
    }
}

impl Inquireable for i32 {
    type Item = i32;

    fn inquire(prompt_label: Option<&str>) -> Result<Self::Item> {
        let string_num = String::inquire(prompt_label)?;

        return match string_num.parse::<i32>() {
            Ok(n) => Ok(n),
            Err(_) => Err(InquisitionError::I32)?,
        };
    }
}

impl Inquireable for u32 {
    type Item = u32;

    fn inquire(prompt_label: Option<&str>) -> Result<Self::Item> {
        let string_num = String::inquire(prompt_label)?;

        return match string_num.parse::<u32>() {
            Ok(n) => Ok(n),
            Err(_) => Err(InquisitionError::U32)?,
        };
    }
}

impl Inquireable for NaiveDate {
    type Item = NaiveDate;
    fn inquire(prompt_label: Option<&str>) -> Result<Self::Item> {
        let year = i32::inquire(prompt_label)?;
        match NaiveDate::from_ymd_opt(year, 1, 1) {
            Some(valid_year) => Ok(valid_year),
            None => Err(InquisitionError::NaiveDate)?,
        }
    }
}
