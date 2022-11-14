use std::io::{stdout, stdin, Write};
use chrono::{NaiveDate, Datelike};

use anyhow::Result;

pub fn into_menu_string<T: std::fmt::Display>(choices: &Vec<T>, title: &str) -> String {
  let mut display = String::new();

  display.push_str("\n");
  display.push_str(title);
  display.push_str("\n");
  display.push_str("---");
  display.push_str("\n");

  let mut i: usize = 1;
  for choice in choices {
    display.push_str(&format!("{}) {}\n", i, choice));
    i += 1;
  }

  display
}

pub fn inquire_string(prompt_label: &str)-> Result<String> {
    let mut s = String::new();

    print!("{}", prompt_label);
    stdout().flush()?;
    stdin().read_line(&mut s)?;

    Ok(s.trim().to_owned())
} 

pub fn inquire_year() -> Option<i32> {
  let year = inquire_i32("Enter year (year): ").ok();
  if let Some(year) = year {
    if let Some(valid_year) = NaiveDate::from_ymd_opt(year, 0, 0) {
      return Some(valid_year.year());
    }
  }

  None
}

pub fn inquire_u32(prompt: &str) -> Result<u32> {
  let num = inquire_string(prompt)?;
  Ok(num.parse::<u32>()?)
}

pub fn inquire_i32(prompt: &str) -> Result<i32> {
  let num = inquire_string(prompt)?;
  Ok(num.parse::<i32>()?)
}

pub fn inquire_menu<T: Clone + std::fmt::Display>(display_menu: String, choices: &Vec<T>) -> T {
  let mut result: Option<T> = None; 
  let invalid_menu_choice_msg = "is invalid choice.";

  while result.is_none() {

    println!("{}", display_menu);

    result = match inquire_string("Enter Choice: ") {
      Ok(ch) => {
        match ch.parse::<usize>() {
          Ok(n) => {
            match choices.get(n-1).cloned() {
              Some(res) => Some(res),
              None => {
                println!("\n---\nError: [{}] {}\n---", n, invalid_menu_choice_msg);
                None
              },
            }
          },
          Err(_) => {
            println!("\n---\nError: [{}] {}\n---", ch, invalid_menu_choice_msg);
            None
          }
        }
      },
      Err(_) => {
        None
      }
    }
  }

  result.expect("Menu choice should never be None")
}