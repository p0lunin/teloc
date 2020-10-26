use crate::common::{compile_error, get_1_method, ident_generator};
use crate::generics::{get_impl_block_generics, get_struct_block_generics, get_where_clause};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, ItemImpl};

pub fn expand(item: &ItemImpl) -> Result<TokenStream, TokenStream> {
    let method = get_1_method(item)?;
    let dependencies = method
        .sig
        .inputs
        .iter()
        .map(|inp| match inp {
            FnArg::Receiver(_) => Err(compile_error("Function must not give self as arg!")),
            FnArg::Typed(pat) => Ok(pat.ty.as_ref()),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let struct_ty = item.self_ty.as_ref();
    let fn_ident = &method.sig.ident;

    let generics = &item.generics;
    let impl_block_generics = get_impl_block_generics(generics);
    let struct_block_generics = get_struct_block_generics(generics);
    let where_clause = get_where_clause(generics);

    let mut destructure = quote! { teloc::frunk::HNil };
    ident_generator(dependencies.len())
        .into_iter()
        .rev()
        .for_each(|id| {
            destructure = quote! {
                teloc::frunk::HCons {
                    head: #id,
                    tail: #destructure
                }
            };
        });
    let names = ident_generator(dependencies.len());

    Ok(quote! {
        impl #impl_block_generics teloc::Dependency<teloc::Hlist![#(#dependencies),*]> for #struct_ty #struct_block_generics #where_clause {
            fn init(data: Hlist![#(#dependencies),*]) -> Self {
                let #destructure = data;
                <#struct_ty>::#fn_ident(#(#names)*)
            }
        }
    })
}
