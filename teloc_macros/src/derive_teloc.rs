use crate::common::{compile_error, get_1_teloc_attr};
use crate::generics::{get_impl_block_generics, get_struct_block_generics, get_where_clause};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseBuffer};
use syn::punctuated::Punctuated;
use syn::Token;
use syn::{DataStruct, Expr, Field, Fields, Generics, Type};

pub fn derive(
    ds: &DataStruct,
    ident: Ident,
    generics: &Generics,
) -> Result<TokenStream, TokenStream> {
    let TelocStruct {
        initable,
        injectable,
    } = parse_teloc_struct(ds)?;

    let impl_block_generics = get_impl_block_generics(&generics);
    let struct_block_generics = get_struct_block_generics(&generics);
    let where_clause = get_where_clause(&generics);

    let init_field = initable.iter().map(|f| &f.field);
    let init_field_ty = initable.iter().map(|f| &f.field_ty);
    let init_field_exprs = initable.iter().map(|f| &f.args);

    let ty_dep = injectable.iter().map(|f| f.field_ty);
    let ty_dep2 = injectable.iter().map(|f| f.field_ty);

    let mut destructure = quote! { teloc::reexport::frunk::HNil };
    injectable.iter().map(|f| f.field).rev().for_each(|id| {
        destructure = quote! {
            teloc::reexport::frunk::HCons {
                head: #id,
                tail: #destructure
            }
        };
    });
    let names = injectable.iter().map(|f| f.field);

    Ok(quote! {
        impl #impl_block_generics
            teloc::Dependency<teloc::reexport::HList![#(#ty_dep),*]>
        for #ident #struct_block_generics #where_clause
        {
            fn init(deps: teloc::reexport::HList![#(#ty_dep2),*]) -> Self {
                let #destructure = deps;
                Self {
                    #(
                        #init_field : <#init_field_ty>::init(#init_field_exprs),
                    )*
                    #(
                        #names,
                    )*
                }
            }
        }
    })
}

pub fn derive_on_unit(ident: Ident, generics: &Generics) -> Result<TokenStream, TokenStream> {
    let impl_block_generics = get_impl_block_generics(&generics);
    let struct_block_generics = get_struct_block_generics(&generics);
    let where_clause = get_where_clause(&generics);

    Ok(quote! {
        impl #impl_block_generics
            teloc::Dependency<teloc::reexport::HList![]>
        for #ident #struct_block_generics #where_clause
        {
            fn init(_: teloc::reexport::HList![]) -> Self {
                #ident
            }
        }
    })
}

fn parse_teloc_struct(ds: &DataStruct) -> Result<TelocStruct, TokenStream> {
    let fields = get_fields(ds);
    let mut initable = vec![];
    let mut injectable = vec![];
    for field in fields {
        match get_1_teloc_attr(field.attrs.as_slice())? {
            Some(attr) => {
                match attr.path.get_ident().unwrap().to_string().as_str() {
                    "init" => {
                        let teloc = attr
                            .parse_args::<TelocAttr>()
                            .map_err(|e| compile_error(e.to_compile_error()))?;
                        let field_ty = &field.ty;
                        initable.push(InitableField {
                            args: teloc.exprs,
                            field_ty,
                            field: &field.ident.as_ref().unwrap(), // TODO: unnamed fields
                        })
                    }
                    _ => unreachable!(),
                }
            }
            None => {
                injectable.push(InjectableField {
                    field_ty: &field.ty,
                    field: &field.ident.as_ref().unwrap(), // TODO: unnamed fields
                })
            }
        }
    }
    Ok(TelocStruct {
        initable,
        injectable,
    })
}

fn get_fields(ds: &DataStruct) -> Vec<&Field> {
    match &ds.fields {
        Fields::Named(named) => named.named.iter().collect(),
        Fields::Unnamed(unnamed) => unnamed.unnamed.iter().collect(),
        Fields::Unit => vec![],
    }
}

struct TelocAttr {
    exprs: Punctuated<Expr, Token![,]>,
}
impl Parse for TelocAttr {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        Ok(Self {
            exprs: input.parse_terminated(Expr::parse)?,
        })
    }
}

struct TelocStruct<'a> {
    initable: Vec<InitableField<'a>>,
    injectable: Vec<InjectableField<'a>>,
}

struct InitableField<'a> {
    args: Punctuated<Expr, Token![,]>,
    field_ty: &'a Type,
    field: &'a Ident,
}
struct InjectableField<'a> {
    field_ty: &'a Type,
    field: &'a Ident,
}
