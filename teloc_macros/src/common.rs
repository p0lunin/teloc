use quote::ToTokens;
use syn::{Attribute, Type, Path, PathArguments};
use proc_macro2::{TokenStream, Ident};
use quote::quote;

pub fn compile_error<T: ToTokens>(data: T) -> proc_macro2::TokenStream {
    quote! {
        compile_error!(#data);
    }
}

pub fn to_turbofish(path: &Path) -> TokenStream {
    let mut res = quote! {};
    for segment in path.segments.iter() {
        let ident = &segment.ident;
        res.extend(quote! {#ident});
        match &segment.arguments {
            PathArguments::None => {}
            PathArguments::AngleBracketed(args) => {
                res.extend(quote! { ::#args });
            }
            _ => unimplemented!()
        }
    }

    res
}

pub fn get_1_teloc_attr(attrs: &[Attribute]) -> Result<Option<&Attribute>, TokenStream> {
    let mut teloc_attrs = vec![];
    attrs.iter().for_each(|attr| {
        if attr.path.is_ident("by") || attr.path.is_ident("init") {
            teloc_attrs.push(attr);
        }
    });
    match teloc_attrs.as_slice() {
        [] => Ok(None),
        [x] => Ok(Some(x)),
        _ => Err(compile_error(format!("Expected 0 or 1 `clone` or `init` attribute, found {}", teloc_attrs.len())))
    }
}

pub fn get_ty_path(ty: &Type) -> Result<&Path, TokenStream> {
    match ty {
        Type::Path(path) => Ok(&path.path),
        _ => {
            println!("{}", Into::<proc_macro::TokenStream>::into(quote! { #ty }));
            Err(compile_error("Expected path"))
        }
    }
}

pub fn expect_1_path_ident<'a>(
    path: &'a Path,
    err: &'static str,
) -> Result<&'a Ident, TokenStream> {
    match path.segments.iter().collect::<Vec<_>>().as_slice() {
        [x] => Ok(&x.ident),
        _ => Err(compile_error(err)),
    }
}
