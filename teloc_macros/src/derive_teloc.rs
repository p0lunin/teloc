use syn::{DataStruct, Expr, Type, Field, Fields, Generics, Path};
use proc_macro2::{TokenStream, Ident};
use syn::punctuated::Punctuated;
use syn::Token;
use quote::{quote};
use crate::common::{get_1_teloc_attr, compile_error, get_ty_path};
use syn::parse::{Parse, ParseBuffer};
use crate::generics::{get_impl_block_generics, get_struct_block_generics, get_where_clause};

pub fn derive(ds: &DataStruct, ident: Ident, generics: &Generics) -> Result<TokenStream, TokenStream> {
    let TelocStruct {
        initable,
        injectable,
        injectable_cloned
    } = parse_teloc_struct(ds)?;

    let impl_block_generics = get_impl_block_generics(&generics);
    let struct_block_generics = get_struct_block_generics(&generics);
    let where_clause = get_where_clause(&generics);

    let init_field = initable.iter().map(|f| &f.field);
    let init_field_ty = initable.iter().map(|f| &f.ty_path);
    let init_field_exprs = initable.iter().map(|f| &f.args);

    let injectable_ident = injectable
        .iter()
        .map(|f| f.field);
    let trait_need = injectable
        .iter()
        .map(|f| f.field_ty);

    let mut needed_traits = quote! {};
    for (i, tr) in trait_need.enumerate() {
        needed_traits.extend(quote! { Get<#tr> });
        if i != injectable.len() - 1 {
            needed_traits.extend(quote! { + });
        }
    }

    let injectable_cloned_ident = injectable_cloned
        .iter()
        .map(|f| f.field);
    let trait_cloned_need = injectable_cloned
        .iter()
        .map(|f| f.field_ty);

    for (i, tr) in trait_cloned_need.enumerate() {
        if i == 0 {
            needed_traits.extend(quote! { + })
        }
        needed_traits.extend(quote! { GetClone<#tr> });
        if i != injectable_cloned.len() - 1 {
            needed_traits.extend(quote! { + });
        }
    }


    Ok(quote! {
        impl #impl_block_generics #ident #struct_block_generics #where_clause {
            pub fn new<T: #needed_traits>(container: &mut T) -> Self {
                Self {
                    #(
                        #init_field : #init_field_ty::new(#init_field_exprs),
                    )*
                    #(
                        #injectable_ident : Get::get(container),
                    )*
                    #(
                        #injectable_cloned_ident : GetClone::get_clone(container),
                    )*
                }
            }
        }
    })
}

fn parse_teloc_struct(ds: &DataStruct) -> Result<TelocStruct, TokenStream> {
    let fields = get_fields(ds);
    let mut initable = vec![];
    let mut injectable = vec![];
    let mut injectable_cloned = vec![];
    for field in fields {
        match get_1_teloc_attr(field.attrs.as_slice())? {
            Some(attr) => {
                match attr.path.get_ident().unwrap().to_string().as_str() {
                    "new" => {
                        let teloc = attr.parse_args::<TelocAttr>().map_err(|_| compile_error("Error when parsing args"))?;
                        let path = get_ty_path(&field.ty)?;
                        initable.push(InitableField {
                            args: teloc.exprs,
                            ty_path: path,
                            field: &field.ident.as_ref().unwrap() // TODO: unnamed fields
                        })
                    }
                    "clone" => {
                        injectable_cloned.push(InjectableField {
                            field_ty: &field.ty,
                            field: &field.ident.as_ref().unwrap() // TODO: unnamed fields
                        })
                    }
                    _ => unreachable!()
                }
            }
            None => {
                injectable.push(InjectableField {
                    field_ty: &field.ty,
                    field: &field.ident.as_ref().unwrap() // TODO: unnamed fields
                })
            }
        }
    }
    Ok(TelocStruct {
        initable,
        injectable,
        injectable_cloned
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
    exprs: Punctuated<Expr, Token![,]>
}
impl Parse for TelocAttr {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        Ok(Self {
            exprs: input.parse_terminated(Expr::parse)?
        })
    }
}

struct TelocStruct<'a> {
    initable: Vec<InitableField<'a>>,
    injectable: Vec<InjectableField<'a>>,
    injectable_cloned: Vec<InjectableField<'a>>,
}

struct InitableField<'a> {
    args: Punctuated<Expr, Token![,]>,
    ty_path: &'a Path,
    field: &'a Ident,
}
struct InjectableField<'a> {
    field_ty: &'a Type,
    field: &'a Ident,
}
