use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use quote::ToTokens;
use syn::Path;
use syn::{Attribute, ImplItemMethod};

pub fn compile_error<T: ToTokens>(data: T) -> proc_macro2::TokenStream {
    quote! {
        compile_error!(#data);
    }
}

pub fn get_1_teloc_attr(attrs: &[Attribute]) -> Result<Option<&Attribute>, TokenStream> {
    let mut teloc_attrs = vec![];
    attrs.iter().for_each(|attr| {
        if attr.path.is_ident("init") {
            teloc_attrs.push(attr);
        }
    });
    match teloc_attrs.as_slice() {
        [] => Ok(None),
        [x] => Ok(Some(x)),
        _ => Err(compile_error(format!(
            "Expected 0 or 1 `clone` or `init` attribute, found {}",
            teloc_attrs.len()
        ))),
    }
}

pub fn strip_annotation_by_path(
    methods: Vec<ImplItemMethod>,
    annotation_path: Path,
) -> Vec<ImplItemMethod> {
    methods
        .into_iter()
        .map(|method| {
            let attrs = method
                .attrs
                .into_iter()
                .filter(|attr| attr.path != annotation_path)
                .collect();

            ImplItemMethod { attrs, ..method }
        })
        .collect()
}

pub fn ident_generator(count: usize) -> Vec<Ident> {
    name_generator()
        .take(count)
        .map(|s| Ident::new(&s, Span::call_site()))
        .collect()
}

pub fn name_generator() -> impl Iterator<Item = String> {
    const ALPHABET: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    (1..).flat_map(|i| {
        ALPHABET
            .iter()
            .combinations_with_replacement(i)
            .map(|arr| arr.iter().join(""))
    })
}
