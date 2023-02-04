use std::fmt;

use anyhow::Result;
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
    year: Option<String>,
    last_oil_change: Option<String>,
    date_created: String,
    last_updated: String,
}

impl Vehicle {
    pub fn new(vin_num: String, make: String, model: String, year: Option<String>) -> Vehicle {
        let now = "";
        Vehicle {
            id: Uuid::new_v4(),
            vin_num,
            make,
            model,
            year,
            last_oil_change: None,
            date_created: now.into(),
            last_updated: now.into(),
        }
    }
}

impl Inquireable for Vehicle {
    type Item = Vehicle;
    fn inquire(_: &str) -> Result<Self::Item> {
        Ok(Vehicle::new(
            String::inquire("Enter vin number: ")?,
            String::inquire("Enter make: ")?,
            String::inquire("Enter model: ")?,
            Some(String::inquire("Enter year: ")?),
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
