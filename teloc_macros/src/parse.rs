use itertools::{Either, Itertools};
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{parse_str, ImplItem, ImplItemMethod, ItemImpl, Path};

use crate::common::strip_annotation_by_path;

pub struct ParseInjectImpl {
    pub item_impl: ItemImpl,
    pub init_method: ImplItemMethod,
}

impl ParseInjectImpl {
    const INIT_ANNOTATION_STR: &'static str = "inject::init";

    pub fn parse(item_impl: ItemImpl) -> syn::Result<Self> {
        let span = item_impl.span();
        let (methods, rest): (Vec<_>, Vec<_>) =
            item_impl
                .items
                .into_iter()
                .partition_map(|item| match item {
                    ImplItem::Method(method) => Either::Left(method),
                    other => Either::Right(other),
                });

        let init_method = Self::get_annotated_method(span, &methods)?;
        let init_method = if let Some(init_method) = init_method {
            init_method
        } else {
            Self::get_only_method(span, &methods)?
        };

        let methods = strip_annotation_by_path(methods, Self::init_annotation_path())
            .into_iter()
            .map(ImplItem::Method);
        let items = rest.into_iter().chain(methods).collect();

        Ok(Self {
            item_impl: ItemImpl { items, ..item_impl },
            init_method,
        })
    }

    fn get_annotated_method(
        span: Span,
        methods: &[ImplItemMethod],
    ) -> syn::Result<Option<ImplItemMethod>> {
        let annotated_methods = methods
            .iter()
            .flat_map(|method| {
                match method
                    .attrs
                    .iter()
                    .find(|attr| attr.path == Self::init_annotation_path())
                {
                    Some(_) => Some(method),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        match annotated_methods.as_slice() {
            [method] => Ok(Some((*method).clone())),
            [] => Ok(None),
            _ => Err(syn::Error::new(
                span,
                format!(
                    "Found more than one method annotated with #[{}] in impl!",
                    Self::INIT_ANNOTATION_STR
                ),
            )),
        }
    }

    fn get_only_method(span: Span, methods: &[ImplItemMethod]) -> syn::Result<ImplItemMethod> {
        match methods {
            [method] => Ok((*method).clone()),
            _ => Err(syn::Error::new(span, "Expected one method in impl!")),
        }
    }

    fn init_annotation_path() -> Path {
        parse_str(Self::INIT_ANNOTATION_STR).unwrap()
    }
}
