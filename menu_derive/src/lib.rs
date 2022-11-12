extern crate proc_macro;
use proc_macro::{TokenStream};

#[proc_macro_derive(Choosable)]
pub fn choosable(_input: TokenStream) -> TokenStream {
    "Choosable".parse().unwrap()
}

// pub fn inquireable(_input: TokenStream) -> TokenStream { Will allow Enums, Structs to have a default inquirerr}