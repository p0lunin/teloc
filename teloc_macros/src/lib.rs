mod common;
mod container;
mod derive_teloc;
mod generics;

extern crate proc_macro;
extern crate quote;
extern crate syn;

use crate::common::compile_error;
use crate::container::ContainerInput;
use proc_macro::TokenStream;
use std::convert::identity;
use syn::Data;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Teloc, attributes(init, by))]
pub fn derive_teloc(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let s = match input.data {
        Data::Struct(ds) => ds,
        Data::Enum(_) => return compile_error("Expected struct, found enum").into(),
        Data::Union(_) => return compile_error("Expected struct, found union").into(),
    };
    let res = derive_teloc::derive(&s, input.ident, &input.generics);
    res.unwrap_or_else(identity).into()
}

#[proc_macro]
pub fn container(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ContainerInput);
    let res = container::container(input);
    res.unwrap_or_else(identity).into()
}
