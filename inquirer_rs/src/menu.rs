use std::fmt;
use strum::IntoEnumIterator;

use crate::helpers::{inquire_menu, into_menu_string};

/// When `#[derive(EnumMenuable)]` is used on an enum, it will implement functions to help
/// with using that enum as a TUI menu.
pub trait Menuable:
    IntoEnumIterator + fmt::Display + std::clone::Clone + Default + InquireableMenu
{
}

pub trait InquireableMenu {
    /// Create a string menu of options from the fields of an enum, and prompt the user for a choice
    ///
    /// ### Example:
    /// ```rust
    /// #[derive(Menuable)]
    /// enum Example {
    ///   Option1,
    ///   Option2,
    /// }
    ///
    /// Example::inquire("Sample Title");
    ///
    /// // Output:
    ///
    /// // Sample Title
    /// // ---
    /// // 1) Option1
    /// // 2) Option2
    /// //  
    /// // Enter choice:
    /// ```
    fn inquire<T: Menuable>(title: &str) -> T {
        let choices: Vec<T> = T::iter().collect::<Vec<T>>();
        inquire_menu(into_menu_string(&choices, title), &choices)
    }
}
