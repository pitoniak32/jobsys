use uuid::Uuid;

pub mod customer;
pub mod error;
pub mod job;
pub mod menu;
pub mod system;
pub mod vehicle;

use customer::Customer;
use job::Job;
use vehicle::Vehicle;

pub enum JobEntities {
    Customer(Customer),
    Vehicle(Vehicle),
    Job(Job),
}

pub trait IdAble {
    fn get_id(&self) -> Uuid;
}

pub trait PathAble {
    fn get_path() -> String;
}
