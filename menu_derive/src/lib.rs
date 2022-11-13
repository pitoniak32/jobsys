extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(EnumMenuable)]
pub fn enum_choosable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let expanded = quote!{
        impl menu::Menuable for #name {}
    };

    TokenStream::from(expanded)
}

// pub fn inquireable(_input: TokenStream) -> TokenStream { Will allow Enums, Structs to have a default inquirerr}