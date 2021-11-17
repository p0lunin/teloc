mod common;
mod derive_teloc;
mod generics;
mod inject;
mod parse;

extern crate proc_macro;
extern crate quote;
extern crate syn;

use crate::common::compile_error;
use proc_macro::TokenStream;
use std::convert::identity;
use syn::Data;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro can be used on structs with **named fields** when all fields implements `Dependency` trait or
/// fields described using `#[init(...)]` attr.
/// We do not recommend using this macro in production code.
///
/// By default macro define all fields as dependencies, but you can initialize field by yourself
/// using attribute `#[init]`. In curly braces you must define a parameters, that will be passed
/// to calling `FieldType::init` method.
///
/// Example:
/// ```compile_fail
/// use teloc::Dependency;
///
/// struct Number(u8);
/// impl Number {
///     fn init(number: u8) -> Self { Number(number) }
/// }
///
/// #[derive(Teloc)]
/// struct Foo {
///     #[init(5)]
///     a: Number
/// }
///
/// #[derive(Teloc)]
/// struct Bar {
///     foo: Foo,
/// }
/// ```
#[proc_macro_derive(Dependency, attributes(init))]
pub fn derive_teloc(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let res = match input.data {
        Data::Struct(ds) => derive_teloc::derive(&ds, input.ident, &input.generics),
        Data::Enum(_) => return compile_error("Expected struct, found enum").into(),
        Data::Union(_) => derive_teloc::derive_on_unit(input.ident, &input.generics),
    };
    res.unwrap_or_else(identity).into()
}

/// Macro can be used on free functions and impls, including impl traits, with *only one* implement
/// method. It will generate `Dependency` impl in which calling function that will tagged by this
/// macro.
/// We recommend using this macro in production code.
///
/// Example:
/// ```compile_fail
/// use teloc::inject;
///
/// struct Number(u8);
/// #[inject]
/// impl Number {
///     fn init(number: u8) -> Self { Number(number) }
/// }
///
/// struct Foo {
///     a: Number
/// }
/// #[inject]
/// fn create_foo(number: Number) -> Foo {
///     Foo { a: number }
/// }
/// ```
#[proc_macro_attribute]
pub fn inject(_: TokenStream, input: TokenStream) -> TokenStream {
    let imp = parse_macro_input!(input as inject::InjectInput);
    let res = inject::expand(&imp);
    let tokens = res.unwrap_or_else(identity);
    (quote::quote! { #imp #tokens }).into()
}
