use chrono::{DateTime, Local};
use inquirer_rs::Inquireable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::IdAble;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Job {
    id: Uuid,
    description: String,
    vehicle_id: Option<Uuid>,
    date_created: DateTime<Local>,
    last_updated: DateTime<Local>,
}

impl Inquireable for Job {
    type Item = Job;

    fn inquire(_: &str) -> anyhow::Result<Self::Item> {
        let now = Local::now();
        let cust = Job {
            id: Uuid::new_v4(),
            description: String::inquire("Enter Description: ")?,
            vehicle_id: None,
            date_created: now,
            last_updated: now,
        };
        Ok(cust)
    }
}

impl Job {
    pub fn new(description: String) -> Job {
        let now = Local::now();

        Job {
            id: Uuid::new_v4(),
            description,
            vehicle_id: None,
            date_created: now,
            last_updated: now,
        }
    }

    pub fn set_vehicle_id(&mut self, vehicle_id: Uuid) {
        self.vehicle_id = Some(vehicle_id);
    }
}

impl IdAble for Job {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
