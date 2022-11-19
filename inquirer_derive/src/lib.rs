extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EnumMenuable)]
pub fn enum_choosable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let expanded = quote! {
        impl inquirer_rs::menu::Menuable for #name {}
        impl inquirer_rs::menu::InquireableMenu for #name {}
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };

    TokenStream::from(expanded)
}

// #[proc_macro_derive(InquireStruct)]
// pub fn struct_inquireable(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);

//     let name = &input.ident;

//     let expanded = quote!{
//         impl inquirer_rs::InquireableStruct for #name {

//         }
//     };

//     TokenStream::from(expanded)
// }
