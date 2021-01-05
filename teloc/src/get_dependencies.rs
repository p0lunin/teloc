//! This is a section for advanced usage. For common usage you can not read this page.

/// The trait, used for getting list of dependencies from provider of services. Do not use it by
/// yourself.
pub trait GetDependencies<'a, Dependencies, Indexes> {
    fn get_deps(&'a self) -> Dependencies;
}

mod impls {
    use crate::get_dependencies::GetDependencies;
    use crate::resolver::Resolver;
    use frunk::hlist::HList;
    use frunk::{HCons, HNil};

    impl<'a, T, TRest, Cont, Infer, InferRest, SP>
        GetDependencies<'a, HCons<T, TRest>, HCons<(Cont, Infer), InferRest>> for SP
    where
        TRest: HList,
        SP: Resolver<'a, Cont, T, Infer> + GetDependencies<'a, TRest, InferRest>,
    {
        fn get_deps(&'a self) -> HCons<T, TRest> {
            GetDependencies::<TRest, InferRest>::get_deps(self).prepend(self.resolve())
        }
    }
    impl<'a, S> GetDependencies<'a, HNil, HNil> for S {
        fn get_deps(&'a self) -> HNil {
            HNil
        }
    }
}
