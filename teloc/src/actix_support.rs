use crate::container::Container;
use crate::Resolver;
use actix_web::dev::*;
use actix_web::Responder;
use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct DIActixHandler<SP, F, Args, Conts, Infers> {
    sp: Arc<SP>,
    f: F,
    phantom: PhantomData<(Args, Conts, Infers)>,
}

impl<SP, F, Args, Conts, Infers> DIActixHandler<SP, F, Args, Conts, Infers> {
    pub fn new(sp: Arc<SP>, f: F) -> Self {
        DIActixHandler {
            sp,
            f,
            phantom: PhantomData,
        }
    }
}

impl<SP, F, Args, Conts, Infers> Clone for DIActixHandler<SP, F, Args, Conts, Infers>
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

macro_rules! impl_factory_di_args {
    (($($num:tt, $param:ident),*), $($arg:ident, $cont:ident, $other:ident),*) => {
        impl<$($param,)* SP, F, Res, O, $($arg, $cont, $other),*> Factory<($($param,)*), Res, O>
            for DIActixHandler<SP, F, ($($param,)*), ($(($arg, $cont),)*), ($($other,)*)>
        where
            $($param: 'static,)*
            SP: 'static,
            F: 'static,
            F: Clone + Fn($($arg,)* $($param),*) -> Res,
            Res: Future<Output = O>,
            O: Responder,
            SP: $(for<'a> Resolver<'a, $cont, $arg, $other> +)*,
            $($arg: 'static,)*
            $($cont: Container<$arg> + 'static,)*
            $($other: 'static,)*
        {
            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            fn call(&self, data: ($($param,)*)) -> Res {
                $(let $arg = self.sp.resolve();)*
                $(let $param = data.$num;)*
                (self.f)($($arg,)* $($param),*)
            }
        }
    }
}

macro_rules! impl_factory_di {
    ($($num:tt, $param:ident),*) => {
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
