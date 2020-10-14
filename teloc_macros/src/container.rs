use proc_macro2::{Ident, TokenStream};
use syn::{Token, Type};
use syn::parse::{Parse, ParseBuffer};
use crate::common::{get_ty_path, expect_1_path_ident};
use quote::quote;

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
            use std::cell::Cell;
            #[allow(non_snake_case)]
            struct Container {
                #(#field : Cell<Option<#ty>>),*
            }
            impl teloc::Get<()> for Container {
                fn get(&self) -> () {
                    ()
                }
            }
            #(
                impl teloc::Get<#ty2> for teloc::ContainerWrapper<Container> {
                    fn get(&self) -> #ty2 {
                        self.0.#field2.replace(None).unwrap()
                    }
                }
                /*impl GetClone<#ty2> for Container {
                    fn get_ref(&self) -> #ty2 {
                        self.#field2.clone()
                    }
                }*/
            )*
            let container = Container {
                #(
                    #field3: Cell::new(None),
                )*
            };
            let wrapper = teloc::ContainerWrapper(container);
            let cref = &wrapper;
            #(
                wrapper.0.#field4.replace(Some(#ty3::init(cref)));
            )*
            wrapper
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