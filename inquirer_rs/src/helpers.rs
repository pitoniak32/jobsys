use anyhow::Result;
use inquire::Select;

pub fn inquire_menu<T: Clone + std::fmt::Display + std::fmt::Debug>(
    title: &str,
    choices: Vec<T>,
) -> Result<T> {
    let result = Select::new(title, choices).prompt()?;
    Ok(result)
}

pub fn inquire_skip() {
    let result = Select::new("Pick: ", vec![1, 2, 3, 4]).prompt_skippable();
    match result {
        Ok(re) => println!("{:?}", re),
        Err(_) => println!("Err"),
    }
}
