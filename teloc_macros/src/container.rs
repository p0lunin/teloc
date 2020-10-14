use proc_macro2::{Ident, TokenStream};
use syn::{Token, Type};
use syn::parse::{Parse, ParseBuffer};
use crate::common::{get_ty_path, expect_1_path_ident};
use quote::quote;
use itertools::Itertools;

pub fn container(input: ContainerInput) -> Result<TokenStream, TokenStream> {
    let field = get_field_idents(input.types.as_slice());
    let field2 = get_field_idents(input.types.as_slice());
    let field3 = get_field_idents(input.types.as_slice());
    let field4 = get_field_idents(input.types.as_slice());
    let ty = input.types.iter();
    let ty2 = input.types.iter();
    let ty3 = input.types.iter();

    Ok(quote! {
        {
            #[allow(non_snake_case)]
            struct Container {
                #(#field : Option<#ty>),*
            }
            impl teloc::Get<()> for Container {
                fn get(&mut self) -> () {
                    ()
                }
            }
            #(
                impl teloc::Get<#ty2> for Container {
                    fn get(&mut self) -> #ty2 {
                        let mut res = None;
                        std::mem::swap(&mut self.#field2, &mut res);
                        res.unwrap()
                    }
                }
                /*impl GetClone<#ty2> for Container {
                    fn get_ref(&self) -> #ty2 {
                        self.#field2.clone()
                    }
                }*/
            )*
            let mut container = Container {
                #(
                    #field3: None,
                )*
            };
            #(
                container.#field4 = Some(#ty3::init(&mut container));
            )*
            container
        }
    })
}

fn get_field_idents<'a>(types: &'a [Type]) -> impl Iterator<Item = Ident> + 'a {
    types
        .iter()
        .map(|t| {
            let id = expect_1_path_ident(get_ty_path(t).unwrap(), "").unwrap();
            id.clone()
        })
}

pub struct ContainerInput {
    types: Vec<Type>
}

impl Parse for ContainerInput {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let types = input.parse_terminated::<Type, Token![,]>(Type::parse)?;
        Ok(Self {
            types: types.into_iter().collect()
        })
    }
}