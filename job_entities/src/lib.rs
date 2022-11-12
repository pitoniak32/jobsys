use uuid::Uuid;

pub mod job;
pub mod vehicle;
pub mod customer;

use customer::Customer;
use vehicle::Vehicle;
use job::Job;

pub enum JobEntities {
  Customer(Customer),
  Vehicle(Vehicle),
  Job(Job),
}

pub trait Inquireable {
  fn inquire<T>() -> T;
}

pub trait IdAble {
  fn get_id(&self) -> Uuid;
}

pub trait PathAble {
  fn get_path() -> String;
}