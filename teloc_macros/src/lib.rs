mod common;
mod derive_teloc;
mod generics;
mod inject;

extern crate proc_macro;
extern crate quote;
extern crate syn;

use crate::common::compile_error;
use proc_macro::TokenStream;
use std::convert::identity;
use syn::Data;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Dependency, attributes(init))]
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

#[proc_macro_attribute]
pub fn inject(_: TokenStream, input: TokenStream) -> TokenStream {
    let imp = parse_macro_input!(input as inject::InjectInput);
    let res = inject::expand(&imp);
    let tokens = res.unwrap_or_else(identity);
    (quote::quote! { #imp #tokens }).into()
}
