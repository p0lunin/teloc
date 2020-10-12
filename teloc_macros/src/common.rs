use quote::ToTokens;
use syn::{Attribute, Type, Path};
use proc_macro2::{TokenStream, Ident};
use quote::quote;

pub fn compile_error<T: ToTokens>(data: T) -> proc_macro2::TokenStream {
    quote! {
        compile_error!(#data);
    }
}

pub fn get_1_teloc_attr(attrs: &[Attribute]) -> Result<Option<&Attribute>, TokenStream> {
    match attrs {
        [] => Ok(None),
        [x] => Ok(Some(x)),
        _ => Err(compile_error(format!("Expected 0 or 1 attribute, found {}", attrs.len())))
    }
}

pub fn get_ty_path(ty: &Type) -> Result<&Path, TokenStream> {
    match ty {
        Type::Path(path) => Ok(&path.path),
        _ => Err(compile_error("Expected path"))
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
