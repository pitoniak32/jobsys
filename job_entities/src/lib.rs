use uuid::Uuid;

pub mod job;
pub mod vehicle;
pub mod customer;

pub trait IdAble {
  fn get_id(&self) -> Uuid;
}

pub trait PathAble {
  fn get_path() -> String;
}