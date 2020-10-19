use crate::common::{expect_1_path_ident, get_ty_path, name_generator};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseBuffer};
use syn::{Token, Type};

pub fn container(input: ContainerInput) -> Result<TokenStream, TokenStream> {
    let count_fields = input.types.len();
    let field = get_field_idents(count_fields);
    let field2 = get_field_idents(count_fields);
    let field3 = get_field_idents(count_fields);
    let field4 = get_field_idents(count_fields);
    let ty = input.types.iter();
    let ty2 = input.types.iter();
    //let ty3 = input.types.iter();

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
                impl teloc::Get<#ty2> for teloc::ContainerWrapper<Container> {
                    fn get(&mut self) -> #ty2 {
                        let mut res = None;
                        std::mem::swap(&mut res, &mut self.0.#field2);
                        res.unwrap()
                    }
                }
                impl teloc::GetRef<#ty2> for teloc::ContainerWrapper<Container> {
                    fn get_ref(&self) -> &#ty2 {
                        self.0.#field2.as_ref().unwrap()
                    }
                }
            )*
            let container = Container {
                #(
                    #field3: None,
                )*
            };
            let mut wrapper = teloc::ContainerWrapper(container);
            let cref = &mut wrapper;
            #(
                cref.0.#field4 = Some(<_>::init(cref));
            )*
            wrapper
        }
    })
}

fn get_field_idents(count: usize) -> impl Iterator<Item = Ident> {
    name_generator()
        .map(|name| Ident::new(name.as_str(), Span::call_site()))
        .take(count)
}

pub struct ContainerInput {
    types: Vec<Type>,
}

impl Parse for ContainerInput {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let types = input.parse_terminated::<Type, Token![,]>(Type::parse)?;
        Ok(Self {
            types: types.into_iter().collect(),
        })
    }
}
