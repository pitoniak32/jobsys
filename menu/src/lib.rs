use std::fmt;
use std::io::{stdout, stdin, Write};

use anyhow::Result;
use strum::IntoEnumIterator;

pub trait Menuable: IntoEnumIterator + fmt::Debug + std::clone::Clone {
  fn get_choice<T: Menuable>(default: T) -> T {
    let choices: Vec<T> = T::iter().collect::<Vec<T>>();
    let copy: Vec<T> = choices.clone();
    let mut display = String::new();

    let mut i = 1;
    for choice in choices {
      display.push_str(&format!("{:?}) {:?}\n", i, choice));
      i += 1;
    }

    println!("{}", display);

    let choice = T::prompt_for_menu_choice().unwrap_or("1".to_owned());
    let num: usize = choice.parse().unwrap();

    copy.get(num-1).unwrap_or(&default).to_owned()
  }

  fn prompt_for_menu_choice() -> Result<String> {
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
}