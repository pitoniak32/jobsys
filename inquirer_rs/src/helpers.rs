use crate::Inquireable;

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

pub fn inquire_menu<T: Clone + std::fmt::Display>(display_menu: String, choices: &Vec<T>) -> T {
    let mut result: Option<T> = None;
    let invalid_menu_choice_msg = "is invalid choice.";

    while result.is_none() {
        println!("{}", display_menu);

        result = match String::inquire(Some("Enter Choice: ")) {
            Some(ch) => match ch.parse::<usize>() {
                Ok(n) => match choices.get(n - 1).cloned() {
                    Some(res) => Some(res),
                    None => {
                        println!("\n---\nError: [{}] {}\n---", n, invalid_menu_choice_msg);
                        None
                    }
                },
                Err(_) => {
                    println!("\n---\nError: [{}] {}\n---", ch, invalid_menu_choice_msg);
                    None
                }
            },
            None => None,
        }
    }

    result.expect("Menu choice should never be None")
}
