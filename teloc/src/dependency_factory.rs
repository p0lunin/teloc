use frunk::{HCons, HNil};
use frunk::Hlist;
use pin_project::__private::PhantomData;
use crate::Dependency;

pub trait DependencyFactory<Deps, OutDep> {
    fn make(&self, dependencies: Deps) -> OutDep;
}

pub struct DepConstructorFactory<T>(PhantomData<T>);

impl<T> DepConstructorFactory<T> {
    pub fn new() -> Self {
        DepConstructorFactory(PhantomData)
    }
}

impl<Deps, OutDep> DependencyFactory<Deps, OutDep> for DepConstructorFactory<OutDep>
where
    OutDep: Dependency<Deps>
{
    fn make(&self, dependencies: Deps) -> OutDep {
        OutDep::init(dependencies)
    }
}


macro_rules! hlist_deconstruct {
    () => { HNil };
    ($dep:ident, $($rest:ident,)*) => {
        HCons { head: $dep, tail: hlist_deconstruct!($($rest,)*) }
    };
}

macro_rules! impl_for_fn {
    () => {
        impl<F, OutDep> DependencyFactory<HNil, OutDep> for F where F: Fn() -> OutDep {
            fn make(&self, _: HNil) -> OutDep {
                self()
            }
        }
    };
    ($dep:ident, $($rest:ident,)*) => {
        impl_for_fn!($($rest,)*);
        #[allow(non_snake_case)]
        impl<F, $dep, $($rest,)* OutDep> DependencyFactory<Hlist![$dep, $($rest,)*], OutDep> for F where F: Fn($dep, $($rest,)*) -> OutDep {
            fn make(&self, dependencies: Hlist![$dep, $($rest,)*]) -> OutDep {
                let hlist_deconstruct!($dep, $($rest,)*) = dependencies;
                self($dep, $($rest,)*)
            }
        }
    };
}

impl_for_fn!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16,);