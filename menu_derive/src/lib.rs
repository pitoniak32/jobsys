extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(ChooseFn)]
pub fn enum_choosable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let expanded = quote!{
        impl menu::structs::Choosable for #name {
            fn get_choice<T: fmt::Debug + Clone + IntoEnumIterator>(default: T) -> T {
                let choices: Vec<T> = T::iter().collect::<Vec<T>>();
                pick_choice(choices, default)
            }
        }
    };

    TokenStream::from(expanded)
}

// pub fn inquireable(_input: TokenStream) -> TokenStream { Will allow Enums, Structs to have a default inquirerr}