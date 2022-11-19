use std::fmt;

use anyhow::Result;
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use inquirer_rs::Inquireable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::IdAble;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vehicle {
    id: Uuid,
    vin_num: String,
    make: String,
    model: String,
    year: Option<i32>,
    last_oil_change: Option<DateTime<Utc>>,
    date_created: DateTime<Utc>,
    last_updated: DateTime<Utc>,
}

impl Vehicle {
    pub fn new(vin_num: String, make: String, model: String, year: Option<i32>) -> Vehicle {
        let now = Utc::now();
        Vehicle {
            id: Uuid::new_v4(),
            vin_num,
            make,
            model,
            year,
            last_oil_change: None,
            date_created: now,
            last_updated: now,
        }
    }
}

impl Inquireable for Vehicle {
    type Item = Vehicle;
    fn inquire(_prompt_label: Option<&str>) -> Result<Self::Item> {
        Ok(Vehicle::new(
            String::inquire(Some("Enter vin number: "))?,
            String::inquire(Some("Enter make: "))?,
            String::inquire(Some("Enter model: "))?,
            Some(NaiveDate::inquire(Some("Enter year: "))?.year()),
        ))
    }
}

impl IdAble for Vehicle {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl fmt::Display for Vehicle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vin: ({})", self.vin_num)
    }
}
