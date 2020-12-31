//! Support for `actix-web` crate.
use crate::{ServiceProvider, Resolver, container::InstanceContainer};
use actix_web::dev::*;
use actix_web::HttpRequest;
use actix_web::Responder;
use frunk::{HNil, HCons};
use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;

/// Struct for inject dependencies from `ServiceProvider` to an actix-web handler function.
///
/// **IMPORTANT:** dependencies from the `ServiceProvider` must be first in the list of arguments.
///
/// For example you can see [example in git repo](https://github.com/p0lunin/teloc/tree/master/examples/actix_example).
pub struct DIActixHandler<SP, CreateScope, F, ScopeResult, Conts, Infers> {
    sp: Arc<SP>,
    create_scope: CreateScope,
    f: F,
    phantom: PhantomData<(ScopeResult, Conts, Infers)>,
}

impl<ParSP, DepsSP, CreateScope, F, ScopeResult, Conts, Infers> DIActixHandler<ServiceProvider<ParSP, DepsSP>, CreateScope, F, ScopeResult, Conts, Infers>
where
    CreateScope: Fn(&ServiceProvider<Arc<ServiceProvider<ParSP, DepsSP>>, HCons<InstanceContainer<HttpRequest>, HNil>>) -> ScopeResult + Clone + 'static,
    Conts: 'static,
    Infers: 'static,
{
    /// Creates DIActixHandler with specified `ServiceProvider` and actix-web handler function.
    pub fn new(sp: Arc<ServiceProvider<ParSP, DepsSP>>, create_scope: CreateScope, f: F) -> Self {
        DIActixHandler {
            sp,
            create_scope,
            f,
            phantom: PhantomData,
        }
    }
}

impl<SP, CreateScope, F, ScopeResult, Conts, Infers> Clone for DIActixHandler<SP, CreateScope, F, ScopeResult, Conts, Infers>
where
    CreateScope: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            sp: self.sp.clone(),
            create_scope: self.create_scope.clone(),
            f: self.f.clone(),
            phantom: PhantomData,
        }
    }
}

macro_rules! impl_factory_di_args {
    (($($num:tt, $param:ident),*), $($arg:ident, $other:ident),*) => {
        impl<$($param,)* ParSP, DepsSP, CreateScope, ScopeResult, F, Res, O, $($arg, $other),*> Factory<(HttpRequest, $($param,)*), Res, O>
            for DIActixHandler<ServiceProvider<ParSP, DepsSP>, CreateScope, F, ScopeResult, ($($arg,)*), ($($other,)*)>
        where
            $($param: 'static,)*
            F: 'static,
            ParSP: 'static,
            DepsSP: 'static,
            F: Clone + Fn($($arg,)* $($param),*) -> Res,
            Res: Future<Output = O>,
            O: Responder,
            CreateScope: Fn(ServiceProvider<Arc<ServiceProvider<ParSP, DepsSP>>, HCons<InstanceContainer<HttpRequest>, HNil>>) -> ScopeResult + Clone + 'static,
            ScopeResult: $(for<'a> Resolver<'a, $arg, $other> +)* 'static,
            $($arg: 'static,)*
            $($other: 'static,)*
        {
            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            fn call(&self, data: (HttpRequest, $($param,)*)) -> Res {
                let (req, $($param,)*) = data;
                let forked = self.sp.fork_arc().add_instance(req);
                let scope = (self.create_scope)(forked);

                $(let $arg = scope.resolve();)*
                (self.f)($($arg,)* $($param),*)
            }
        }
    }
}

macro_rules! impl_factory_di {
    ($($num:tt, $param:ident),*) => {
        impl_factory_di_args!(($($num, $param),*), A1, O1);
        impl_factory_di_args!(($($num, $param),*), A1, O1, A2, O2);
        impl_factory_di_args!(($($num, $param),*), A1, O1, A2, O2, A3, O3);
        impl_factory_di_args!(($($num, $param),*), A1, O1, A2, O2, A3, O3, A4, O4);
        impl_factory_di_args!(($($num, $param),*), A1, O1, A2, O2, A3, O3, A4, O4, A5, O5);
        impl_factory_di_args!(($($num, $param),*), A1, O1, A2, O2, A3, O3, A4, O4, A5, O5, A6, O6);
        impl_factory_di_args!(($($num, $param),*), A1, O1, A2, O2, A3, O3, A4, O4, A5, O5, A6, O6, A7, O7);
        impl_factory_di_args!(($($num, $param),*), A1, O1, A2, O2, A3, O3, A4, O4, A5, O5, A6, O6, A7, O7, A8, O8);
        impl_factory_di_args!(($($num, $param),*), A1, O1, A2, O2, A3, O3, A4, O4, A5, O5, A6, O6, A7, O7, A8, O8, A9, O9);
    };
}

//impl_factory_di!();
impl_factory_di!(0, B1);
impl_factory_di!(0, B1, 1, B2);
impl_factory_di!(0, B1, 1, B2, 2, B3);
impl_factory_di!(0, B1, 1, B2, 2, B3, 3, B4);
impl_factory_di!(0, B1, 1, B2, 2, B3, 3, B4, 4, B5);
impl_factory_di!(0, B1, 1, B2, 2, B3, 3, B4, 4, B5, 5, B6);
impl_factory_di!(0, B1, 1, B2, 2, B3, 3, B4, 4, B5, 5, B6, 6, B7);
impl_factory_di!(0, B1, 1, B2, 2, B3, 3, B4, 4, B5, 5, B6, 6, B7, 7, B8);
impl_factory_di!(0, B1, 1, B2, 2, B3, 3, B4, 4, B5, 5, B6, 6, B7, 7, B8, 8, B9);
