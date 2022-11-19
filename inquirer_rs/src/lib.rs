use std::io::Write;

use chrono::NaiveDate;

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
    fn inquire(prompt_label: Option<&str>) -> Option<Self::Item>;

    /// Inquire once, and if `None` is returned, Inquire `retries` times before returning `None`  
    ///
    /// for a total of `retries + 1` inquisitions.
    fn inquire_retry_on_none(
        retries: u32,
        retry_msg: Option<&str>,
        prompt_label: Option<&str>,
    ) -> Option<Self::Item> {
        let first_try = Self::inquire(prompt_label);

        if let Some(ft) = first_try {
            return Some(ft);
        }

        for i in 0..retries {
            if let Some(rtry) = retry_msg {
                println!("\nRetry: {}", rtry);
                println!("Retries Left: {}", retries - (i + 1));
                if retries - (i + 1) == 0 {
                    println!("\nLast Try!");
                }
                println!("")
            }

            if let Some(val) = Self::inquire(prompt_label) {
                return Some(val);
            }
        }
        None
    }
}

impl Inquireable for String {
    type Item = String;
    fn inquire(prompt_label: Option<&str>) -> Option<Self::Item> {
        let mut s = String::new();

        if let Some(lbl) = prompt_label {
            print!("{}", lbl);
        }

        if let Ok(_) = std::io::stdout().flush() {
            if let Ok(_) = std::io::stdin().read_line(&mut s) {
                return Some(s.trim().to_owned());
            }
        }
        None
    }
}

impl Inquireable for i32 {
    type Item = i32;

    fn inquire(prompt_label: Option<&str>) -> Option<Self::Item> {
        if let Some(num) = String::inquire(prompt_label) {
            return match num.parse::<i32>() {
                Ok(n) => Some(n),
                Err(_) => None,
            };
        }
        None
    }
}

impl Inquireable for u32 {
    type Item = u32;
    fn inquire(prompt_label: Option<&str>) -> Option<Self::Item> {
        if let Some(num) = String::inquire(prompt_label) {
            return match num.parse::<u32>() {
                Ok(n) => Some(n),
                Err(_) => None,
            };
        }
        None
    }
}

impl Inquireable for NaiveDate {
    type Item = NaiveDate;
    fn inquire(prompt_label: Option<&str>) -> Option<Self::Item> {
        let year = i32::inquire(prompt_label);
        if let Some(year) = year {
            if let Some(valid_year) = NaiveDate::from_ymd_opt(year, 1, 1) {
                return Some(valid_year);
            }
        }
        None
    }
}
