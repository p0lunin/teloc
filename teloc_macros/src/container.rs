use crate::common::name_generator;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
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

    let mut init_dependencies = quote! {};
    for (dependency, field) in input.types.iter().zip(field4) {
        match &dependency.init {
            DependencyInitType::CertainType => {
                init_dependencies.extend(quote! {
                    cref.0.#field = Some(NewType(<#dependency>::init(cref)));
                });
            }
            DependencyInitType::Assign(assign) => {
                init_dependencies.extend(quote! {
                    cref.0.#field = Some(NewType(#assign));
                });
            }
            DependencyInitType::InterfaceType(_) => {
                let ty = &dependency.ty;
                init_dependencies.extend(quote! {
                    cref.0.#field = Some(NewType(<#ty>::init(cref)));
                });
            }
        }
    }

    Ok(quote! {
        {
            use teloc::Getable;
            struct NewType<T>(T);
            impl<T> Getable<T> for NewType<T> { }
            #[allow(non_snake_case)]
            struct Container {
                #(#field : Option<NewType<#ty>>),*
            }
            impl teloc::Get<(), ()> for Container {
                fn get(&mut self) -> () {
                    ()
                }
            }
            #(
                impl teloc::Get<NewType<#ty2>, #ty2> for teloc::ContainerWrapper<Container> {
                    fn get(&mut self) -> #ty2 {
                        let mut res = None;
                        std::mem::swap(&mut res, &mut self.0.#field2);
                        let NewType(t) = res.unwrap();
                        t
                    }
                }
                impl teloc::GetRef<NewType<#ty2>, #ty2> for teloc::ContainerWrapper<Container> {
                    fn get_ref(&self) -> &#ty2 {
                        let NewType(t) = self.0.#field2.as_ref().unwrap();
                        t
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
            #init_dependencies
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
    types: Vec<TelocExprInput>,
}

impl Parse for ContainerInput {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let types = input.parse_terminated::<TelocExprInput, Token![,]>(TelocExprInput::parse)?;
        Ok(Self {
            types: types.into_iter().collect(),
        })
    }
}

struct TelocExprInput {
    ty: Type,
    init: DependencyInitType,
}

enum DependencyInitType {
    CertainType,
    Assign(Ident),
    InterfaceType(Type),
}

impl Parse for TelocExprInput {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let ty = input.parse::<Type>()?;
        let init = match input.parse::<Token![=]>() {
            Ok(_) => DependencyInitType::Assign(input.parse::<Ident>()?),
            Err(_) => match input.parse::<Token![as]>() {
                Ok(_) => DependencyInitType::InterfaceType(input.parse::<Type>()?),
                Err(_) => DependencyInitType::CertainType,
            }
        };
        Ok(Self { ty, init })
    }
}

impl ToTokens for TelocExprInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.init {
            DependencyInitType::InterfaceType(ty) => ty.to_tokens(tokens),
            _ => self.ty.to_tokens(tokens)
        }
    }
}
