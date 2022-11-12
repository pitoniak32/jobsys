use std::fmt;
use std::io::{stdout, stdin, Write};

use anyhow::Result;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait Choosable {
  fn get_choice<T: fmt::Debug + Clone + IntoEnumIterator>(default: T) -> T {
    let choices: Vec<T> = T::iter().collect::<Vec<T>>();
    pick_choice(choices, default)
  }
}

#[derive(Debug, EnumIter, PartialEq, Clone)]
pub enum MainMenuChoices {
  Jobs,
  Vehicles,
  Customers,
  Settings,
  Quit,
}

impl Choosable for MainMenuChoices {}

impl fmt::Display for MainMenuChoices {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

#[derive(Debug, EnumIter, PartialEq, Clone)]
pub enum EntityOptions {
  New,
  Update,
  Delete,
  View,
  Back,
}

impl Choosable for EntityOptions {}

impl fmt::Display for EntityOptions {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

pub fn prompt_for_menu_choice() -> Result<String> {
  let mut s=String::new();

  print!("Please enter choice: ");
  let _ = stdout().flush();

  stdin().read_line(&mut s)?;

  if let Some('\n')=s.chars().next_back() {
      s.pop();
  }
  if let Some('\r')=s.chars().next_back() {
      s.pop();
  }

  Ok(s)
} 

fn pick_choice<T: Clone + fmt::Debug>(choices: Vec<T>, default: T) -> T  {
    let copy: Vec<T> = choices.clone();
    let mut display = String::new();

    let mut i = 1;
    for choice in choices {
      display.push_str(&format!("{:?}) {:?}\n", i, choice));
      i += 1;
    }

    println!("{}", display);

    let choice = prompt_for_menu_choice().unwrap_or("1".to_owned());
    let num: usize = choice.parse().unwrap();

    copy.get(num-1).unwrap_or(&default).to_owned()
}