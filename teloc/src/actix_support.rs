//! Support for `actix-web` crate.
#![allow(unsafe_code)]

use crate::dependency::DependencyClone;
use crate::{container::InstanceContainer, Resolver, ServiceProvider};
use actix_web::dev::*;
use actix_web::web::Data;
use actix_web::Responder;
use actix_web::{http, FromRequest, HttpRequest};
use frunk::hlist::Selector;
use frunk::{HCons, HNil};
use std::cell::Ref;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::Arc;

/// Struct for inject dependencies from `ServiceProvider` to an actix-web handler function.
///
/// **IMPORTANT:** dependencies from the `ServiceProvider` must be first in the list of arguments.
///
/// For example you can see [example in git repo](https://github.com/p0lunin/teloc/tree/master/examples/actix_example).
pub struct DIActixHandler<SP, CreateScope, F, ScopeResult, Args, Infers> {
    sp: Arc<SP>,
    create_scope: CreateScope,
    f: F,
    phantom: PhantomData<(ScopeResult, Args, Infers)>,
}

impl<ParSP, DepsSP, CreateScope, F, ScopeResult, Args, Infers>
    DIActixHandler<ServiceProvider<ParSP, DepsSP>, CreateScope, F, ScopeResult, Args, Infers>
where
    CreateScope: Fn(
            ServiceProvider<
                Arc<ServiceProvider<ParSP, DepsSP>>,
                HCons<InstanceContainer<HttpRequest>, HNil>,
            >,
        ) -> ScopeResult
        + Clone
        + 'static,
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

impl<SP, CreateScope, F, ScopeResult, Args, Infers> Clone
    for DIActixHandler<SP, CreateScope, F, ScopeResult, Args, Infers>
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

// Safety was checked in https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=118c918dcf33f7fd15faec185e3bcc4b
// by miri
#[pin_project::pin_project(PinnedDrop)]
pub struct SPFuture<SP, Fut> {
    sp: *mut SP,
    #[pin]
    fut: NonNull<Fut>,
}

impl<SP, Fut> SPFuture<SP, Fut> {
    pub fn new(sp: *mut SP, f: impl FnOnce(*const SP) -> Fut) -> Pin<Box<Self>> {
        let mut this = Box::pin(SPFuture {
            sp,
            fut: NonNull::dangling(),
        });
        let fut = Box::leak(Box::new(f(this.sp)));
        unsafe {
            let mut_ref: Pin<&mut Self> = this.as_mut();
            Pin::get_unchecked_mut(mut_ref).fut = NonNull::from(fut);
        }
        this
    }
}

#[pin_project::pinned_drop]
impl<SP, Fut> PinnedDrop for SPFuture<SP, Fut> {
    fn drop(self: Pin<&mut Self>) {
        use std::alloc::{dealloc, Layout};
        unsafe {
            std::ptr::drop_in_place(self.fut.as_ptr());
            dealloc(self.fut.as_ptr() as *mut u8, Layout::new::<Fut>());
            std::ptr::drop_in_place(self.sp);
            dealloc(self.sp as *mut u8, Layout::new::<SP>());
        }
    }
}

impl<SP, Fut> Future for SPFuture<SP, Fut>
where
    Fut: Future,
{
    type Output = Fut::Output;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        // SAFETY: we just cast NonNull<T> to &'static mut T and use it only to the end of the function.
        let fut = unsafe { this.fut.map_unchecked_mut(|x| x.as_mut()) };
        fut.poll(cx)
    }
}

macro_rules! impl_factory_di_args {
    (($($num:tt, $param:ident),*), $($arg:ident, $cont:ident, $other:ident),*) => {
        impl<$($param,)* ParSP, DepsSP, CreateScope, ScopeResult, F, Res, $($arg, $cont, $other),*>
            Factory<
                (HttpRequest, $($param,)*),
                Pin<Box<SPFuture<ScopeResult, Pin<Box<dyn Future<Output=Res::Output>>>>>>,
                Res::Output
            >
            for DIActixHandler<ServiceProvider<ParSP, DepsSP>, CreateScope, F, ScopeResult, ($(($arg,$cont),)*), ($($other,)*)>
        where
            (HttpRequest, $($param,)*): FromRequest + 'static,
            F: 'static,
            ParSP: 'static,
            DepsSP: 'static,
            F: Clone + Fn($($arg,)* $($param),*) -> Res,
            Res: Future,
            Res::Output: Responder,
            CreateScope: Fn(ServiceProvider<Arc<ServiceProvider<ParSP, DepsSP>>, HCons<InstanceContainer<HttpRequest>, HNil>>) -> ScopeResult + Clone + 'static,
            ScopeResult: $(Resolver<'static, $cont, $arg, $other> +)* 'static,
            Self: 'static,
        {
            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            fn call(&self, data: (HttpRequest, $($param,)*))
                -> Pin<Box<SPFuture<ScopeResult, Pin<Box<dyn Future<Output=Res::Output>>>>>>
            {
                let (req, $($param,)*) = data;
                let forked = self.sp.fork_arc().add_instance(req);
                let scope = Box::new((self.create_scope)(forked));
                let ptr = Box::into_raw(scope);
                SPFuture::new(ptr, move |sp| {
                    let f = self.f.clone();
                    $(let $param = $param;)*
                    Box::pin(async move {
                        // SAFETY: cast *mut T to &'static mut T is valid because we drop reference early than drop T
                        // (see impl PinnedDrop for SPFuture)
                        let sp_ref = unsafe { sp.as_ref() }.unwrap();
                        $(let $arg = sp_ref.resolve();)*
                        (f)($($arg,)* $($param),*).await
                    })
                } as Pin<Box<dyn Future<Output=Res::Output>>>)
            }
        }
    }
}

macro_rules! impl_factory_di {
    ($($num:tt, $param:ident),*) => {
        impl_factory_di_args!(($($num, $param),*),);
        impl_factory_di_args!(($($num, $param),*), A1, C1, O1);
        impl_factory_di_args!(($($num, $param),*), A1, C1, O1, A2, C2, O2);
        impl_factory_di_args!(($($num, $param),*), A1, C1, O1, A2, C2, O2, A3, C3, O3);
        impl_factory_di_args!(($($num, $param),*), A1, C1, O1, A2, C2, O2, A3, C3, O3, A4, C4, O4);
        impl_factory_di_args!(($($num, $param),*), A1, C1, O1, A2, C2, O2, A3, C3, O3, A4, C4, O4, A5, C5, O5);
        impl_factory_di_args!(($($num, $param),*), A1, C1, O1, A2, C2, O2, A3, C3, O3, A4, C4, O4, A5, C5, O5, A6, C6, O6);
        impl_factory_di_args!(($($num, $param),*), A1, C1, O1, A2, C2, O2, A3, C3, O3, A4, C4, O4, A5, C5, O5, A6, C6, O6, A7, C7, O7);
        impl_factory_di_args!(($($num, $param),*), A1, C1, O1, A2, C2, O2, A3, C3, O3, A4, C4, O4, A5, C5, O5, A6, C6, O6, A7, C7, O7, A8, C8, O8);
        impl_factory_di_args!(($($num, $param),*), A1, C1, O1, A2, C2, O2, A3, C3, O3, A4, C4, O4, A5, C5, O5, A6, C6, O6, A7, C7, O7, A8, C8, O8, A9, C9, O9);
    };
}

impl_factory_di!();
impl_factory_di!(0, B1);
impl_factory_di!(0, B1, 1, B2);
impl_factory_di!(0, B1, 1, B2, 2, B3);
impl_factory_di!(0, B1, 1, B2, 2, B3, 3, B4);
impl_factory_di!(0, B1, 1, B2, 2, B3, 3, B4, 4, B5);
impl_factory_di!(0, B1, 1, B2, 2, B3, 3, B4, 4, B5, 5, B6);
impl_factory_di!(0, B1, 1, B2, 2, B3, 3, B4, 4, B5, 5, B6, 6, B7);
impl_factory_di!(0, B1, 1, B2, 2, B3, 3, B4, 4, B5, 5, B6, 6, B7, 7, B8);
impl_factory_di!(0, B1, 1, B2, 2, B3, 3, B4, 4, B5, 5, B6, 6, B7, 7, B8, 8, B9);

impl DependencyClone for HttpRequest {}

pub struct GetRequestData<T>(T);

macro_rules! impl_resolver_for_request {
    ($(($ty:ty, $get:expr)),*) => {
        $(
        impl<'a, SP, Index> Resolver<'a, GetRequestData<$ty>, $ty, Index> for SP
        where
            Self: Selector<InstanceContainer<HttpRequest>, Index>,
        {
            fn resolve(&'a self) -> $ty {
                $get(self.get().get())
            }
        }
        )*
    };
}

impl_resolver_for_request! (
    (&'a actix_http::RequestHead, |req: &'a HttpRequest| req.head()),
    (&'a http::Uri, |req: &'a HttpRequest| req.uri()),
    (&'a http::Method, |req: &'a HttpRequest| req.method()),
    (http::Version, |req: &'a HttpRequest| req.version()),
    (&'a http::HeaderMap, |req: &'a HttpRequest| req.headers()),
    (&'a actix_router::Path<actix_router::Url>, |req: &'a HttpRequest| req.match_info()),
    (Ref<'a, actix_http::Extensions>, |req: &'a HttpRequest| req.extensions()),
    (&'a actix_web::dev::ResourceMap, |req: &'a HttpRequest| req.resource_map()),
    (Option<std::net::SocketAddr>, |req: &'a HttpRequest| req.head().peer_addr),
    (Ref<'a, ConnectionInfo>, |req: &'a HttpRequest| req.connection_info()),
    (&'a AppConfig, |req: &'a HttpRequest| req.app_config())
);

impl<'a, T, SP, Index>
    Resolver<'a, GetRequestData<Option<&'a Data<T>>>, Option<&'a Data<T>>, Index> for SP
where
    T: 'static,
    Self: Selector<InstanceContainer<HttpRequest>, Index>,
{
    fn resolve(&'a self) -> Option<&'a Data<T>> {
        self.get().get().app_data::<Data<T>>()
    }
}
