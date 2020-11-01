use crate::common::{compile_error, get_1_method, ident_generator};
use crate::generics::{get_impl_block_generics, get_where_clause};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseBuffer};
use syn::{FnArg, ImplItemMethod, ItemFn, ItemImpl, ReturnType, Type};

pub fn expand(input: &InjectInput) -> Result<TokenStream, TokenStream> {
    let dependencies = input
        .sig()
        .inputs
        .iter()
        .map(|inp| match inp {
            FnArg::Receiver(_) => Err(compile_error("Function must not give self as arg!")),
            FnArg::Typed(pat) => Ok(pat.ty.as_ref()),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let struct_ty = get_struct_ty(input)?;
    let fn_ident = &input.sig().ident;

    let generics = &input.generics();
    let impl_block_generics = generics.map(get_impl_block_generics);
    let where_clause = generics.map(get_where_clause);

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

    let init = match input {
        InjectInput::Impl(_, _) => quote! { <#struct_ty>::#fn_ident(#(#names)*) },
        InjectInput::Function(_) => quote! { #fn_ident(#(#names)*) },
    };

    Ok(quote! {
        impl #impl_block_generics teloc::Dependency<teloc::Hlist![#(#dependencies),*]> for #struct_ty #where_clause {
            fn init(data: teloc::Hlist![#(#dependencies),*]) -> Self {
                let #destructure = data;
                #init
            }
        }
    })
}

fn get_struct_ty(inp: &InjectInput) -> Result<&Type, TokenStream> {
    match inp {
        InjectInput::Impl(x, _) => Ok(x.self_ty.as_ref()),
        InjectInput::Function(f) => match &f.sig.output {
            ReturnType::Default => Err(compile_error(
                "Expected return type, found default return type",
            )),
            ReturnType::Type(_, ty) => Ok(ty.as_ref()),
        },
    }
}

pub enum InjectInput {
    Impl(ItemImpl, ImplItemMethod),
    Function(ItemFn),
}
impl InjectInput {
    fn sig(&self) -> &syn::Signature {
        match self {
            InjectInput::Impl(_, x) => &x.sig,
            InjectInput::Function(x) => &x.sig,
        }
    }
    fn generics(&self) -> Option<&syn::Generics> {
        match self {
            InjectInput::Impl(x, _) => Some(&x.generics),
            InjectInput::Function(_) => None,
        }
    }
}

impl Parse for InjectInput {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        if let Ok(f) = input.parse::<ItemFn>() {
            Ok(Self::Function(f))
        } else {
            let item: ItemImpl = input.parse()?;
            let assoc = get_1_method(item.clone())?;
            Ok(Self::Impl(item, assoc))
        }
    }
}

impl ToTokens for InjectInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            InjectInput::Impl(x, _) => x.to_tokens(tokens),
            InjectInput::Function(x) => x.to_tokens(tokens),
        }
    }
}
