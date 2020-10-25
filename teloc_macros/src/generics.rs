use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericParam, Generics};

pub fn get_impl_block_generics(generics: &Generics) -> TokenStream {
    let params = &generics.params;
    match params.len() {
        0 => quote!(),
        _ => quote! {
            <#params>
        },
    }
}

pub fn get_where_clause(generics: &Generics) -> TokenStream {
    let clause = &generics.where_clause;
    quote! {
        #clause
    }
}

pub fn get_struct_block_generics(generics: &Generics) -> TokenStream {
    let params = generics.params.iter().map(get_generic_ident);
    match params.len() {
        0 => quote!(),
        _ => quote! {
            <#(#params),*>
        },
    }
}

fn get_generic_ident(g: &GenericParam) -> TokenStream {
    match g {
        GenericParam::Type(t) => {
            let id = &t.ident;
            quote!(#id)
        }
        GenericParam::Lifetime(l) => quote!(#l),
        GenericParam::Const(_) => unimplemented!(), // TODO: const generics
    }
}
