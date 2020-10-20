use crate::common::{compile_error, get_1_teloc_attr, name_generator, to_turbofish};
use crate::generics::{
    get_impl_block_generics, get_struct_block_generics, get_struct_block_generics_without_arrows,
    get_where_clause,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseBuffer};
use syn::punctuated::Punctuated;
use syn::{Attribute, Token};
use syn::{DataStruct, Expr, Field, Fields, Generics, Type};

pub fn derive(
    ds: &DataStruct,
    ident: Ident,
    generics: &Generics,
    attrs: &[Attribute],
) -> Result<TokenStream, TokenStream> {
    let TelocStruct {
        initable,
        injectable,
    } = parse_teloc_struct(ds)?;

    let teloc_struct_attr = parse_teloc_struct_attr(attrs)?;

    let impl_block_generics = get_impl_block_generics(&generics);
    let struct_block_generics = get_struct_block_generics(&generics);
    let where_clause = get_where_clause(&generics);

    let init_field = initable.iter().map(|f| &f.field);
    let init_field_ty = initable.iter().map(|f| match f.field_ty {
        Type::Path(p) => to_turbofish(&p.path),
        Type::Verbatim(v) => v.clone(),
        Type::Macro(m) => quote! { #m },
        _ => unimplemented!(),
    });
    let init_field_exprs = initable.iter().map(|f| &f.args);

    let injectable_ident_inject = injectable
        .iter()
        .zip(generate_generics(ds.fields.len()))
        .map(|(f, gen)| {
            let ident = f.field;
            let ty = f.field_ty;
            match f.get_by {
                GetBy::Own => quote! { #ident : teloc::Get::<#gen, #ty>::get(container) },
                GetBy::Ref => quote! { #ident : teloc::GetRef::<#gen, #ty>::get_ref(container) },
                GetBy::Clone => {
                    quote! { #ident : teloc::GetClone::<#gen, #ty>::get_clone(container) }
                }
            }
        });
    let trait_need = injectable.iter().map(|f| (f.field_ty, &f.get_by));

    let mut needed = quote! {};
    let mut type_generics = quote! {};
    for ((i, (tr, get_by)), generic) in trait_need
        .enumerate()
        .zip(generate_generics(ds.fields.len()))
    {
        type_generics.extend(quote! { #generic : teloc::Getable<#tr> });
        needed.extend(match get_by {
            GetBy::Own => quote! { teloc::Get<#generic, #tr> },
            GetBy::Ref => quote! { teloc::GetRef<#generic, #tr> },
            GetBy::Clone => quote! { teloc::GetClone<#generic, #tr> },
        });
        if i != injectable.len() - 1 {
            needed.extend(quote! { + });
        }
        type_generics.extend(quote! { , });
    }

    let type_generics2 = type_generics.clone();
    let needed2 = needed.clone();
    let type_generics3 = type_generics.clone();
    let needed3 = needed.clone();

    let ty = teloc_struct_attr.impls.iter();
    let init = Ident::new(
        format!("__TelocPrivate_Init_{}", &ident).as_str(),
        Span::call_site(),
    );

    let generics_for_init_trait = get_struct_block_generics_without_arrows(generics);

    Ok(quote! {
        trait #init #struct_block_generics #where_clause {
            fn init<#type_generics2 ContainerT: #needed2>(container: &mut ContainerT) -> Self;
        }
        #(
            impl#impl_block_generics #init for #ty<#ident #generics_for_init_trait> #where_clause {
                fn init<#type_generics3 ContainerT: #needed3>(container: &mut ContainerT) -> Self {
                    #ty::new(<#ident>::init(container))
                }
            }
        )*
        impl #impl_block_generics #ident #struct_block_generics #where_clause {
            pub fn init<#type_generics ContainerT: #needed>(container: &mut ContainerT) -> Self {
                Self {
                    #(
                        #init_field : #init_field_ty::init(#init_field_exprs),
                    )*
                    #(
                        #injectable_ident_inject,
                    )*
                }
            }
        }
    })
}

fn generate_generics(count: usize) -> impl Iterator<Item = Ident> {
    name_generator()
        .map(|name| Ident::new(name.as_str(), Span::call_site()))
        .take(count)
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
                    "by" => {
                        let get_by = attr
                            .parse_args::<GetBy>()
                            .map_err(|e| compile_error(e.to_compile_error()))?;
                        injectable.push(InjectableField {
                            field_ty: &field.ty,
                            field: &field.ident.as_ref().unwrap(), // TODO: unnamed fields
                            get_by,
                        })
                    }
                    _ => unreachable!(),
                }
            }
            None => {
                injectable.push(InjectableField {
                    field_ty: &field.ty,
                    field: &field.ident.as_ref().unwrap(), // TODO: unnamed fields
                    get_by: GetBy::Own,
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

enum GetBy {
    Own,
    Ref,
    Clone,
}

impl Parse for GetBy {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let id: Ident = input.parse()?;
        match id.to_string().as_str() {
            "own" => Ok(GetBy::Own),
            "reff" => Ok(GetBy::Ref),
            "clone" => Ok(GetBy::Clone),
            _ => Err(syn::Error::new(
                id.span(),
                format!(
                    "Expected one of `own`, `ref`, `clone`, found {}",
                    id.to_string()
                ),
            )),
        }
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
    get_by: GetBy,
}

struct TelocStructAttr {
    impls: Vec<Ident>,
}

impl Parse for TelocStructAttr {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let inp = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Self {
            impls: inp.into_iter().collect(),
        })
    }
}

fn parse_teloc_struct_attr(attrs: &[Attribute]) -> Result<TelocStructAttr, TokenStream> {
    match attrs {
        [] => Ok(TelocStructAttr { impls: vec![] }),
        [attr] if attr.path.is_ident("implem") => attr
            .parse_args::<TelocStructAttr>()
            .map_err(|e| compile_error(e.to_compile_error())),
        _ => Err(compile_error("Expected 0 or 1 `implem` attr")),
    }
}
