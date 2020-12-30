use actix_web::dev::{Factory, Payload, PayloadStream};
use actix_web::{Error, FromRequest, HttpRequest, Responder};
use futures_util::future::Ready;
use std::convert::Infallible;
use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;
use teloc::container::Container;
use teloc::Resolver;

pub struct DIActixHandler<SP, F, Args> {
    sp: Arc<SP>,
    f: F,
    phantom: PhantomData<Args>,
}

impl<SP, F, Args> DIActixHandler<SP, F, Args> {
    pub fn new(sp: Arc<SP>, f: F) -> Self {
        DIActixHandler {
            sp,
            f,
            phantom: PhantomData,
        }
    }
}

impl<SP, F, Args> Clone for DIActixHandler<SP, F, Args>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            sp: self.sp.clone(),
            f: self.f.clone(),
            phantom: PhantomData,
        }
    }
}

pub struct PhantomArgs<T>(PhantomData<T>);

impl<T> FromRequest for PhantomArgs<T> {
    type Error = Infallible;
    type Future = Ready<Result<PhantomArgs<T>, Self::Error>>;
    type Config = ();

    fn from_request(_: &HttpRequest, _: &mut Payload<PayloadStream>) -> Self::Future {
        futures_util::future::ok(PhantomArgs(PhantomData))
    }
}

impl<A, SP, F, Res, O, C1, OtherC1> Factory<PhantomArgs<(C1, OtherC1)>, Res, O>
    for DIActixHandler<SP, F, (A,)>
where
    A: 'static,
    SP: 'static,
    F: 'static,
    F: Clone + Fn(A) -> Res,
    Res: Future<Output = O>,
    O: Responder,
    SP: for<'a> Resolver<'a, C1, A, OtherC1>,
    C1: Container<A>,
{
    fn call(&self, _: PhantomArgs<(C1, OtherC1)>) -> Res {
        (self.f)(self.sp.resolve())
    }
}

impl<A, B, SP, F, Res, O, C1, OtherC1> Factory<(B, PhantomArgs<(C1, OtherC1)>), Res, O>
    for DIActixHandler<SP, F, (A,)>
where
    A: 'static,
    B: 'static,
    SP: 'static,
    F: 'static,
    F: Clone + Fn(A, B) -> Res,
    Res: Future<Output = O>,
    O: Responder,
    SP: for<'a> Resolver<'a, C1, A, OtherC1>,
    C1: Container<A>,
{
    fn call(&self, (arg, _): (B, PhantomArgs<(C1, OtherC1)>)) -> Res {
        (self.f)(self.sp.resolve(), arg)
    }
}
