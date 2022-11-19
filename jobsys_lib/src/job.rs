use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::IdAble;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Job {
    id: Uuid,
    description: String,
    date_created: DateTime<Local>,
    last_updated: DateTime<Local>,
}

impl Job {
    pub fn new(description: String) -> Job {
        let now = Local::now();

        Job {
            id: Uuid::new_v4(),
            description,
            date_created: now,
            last_updated: now,
        }
    }
}

impl IdAble for Job {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
