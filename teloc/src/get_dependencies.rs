//! This is a section for advanced usage. For common usage you can not read this page.

/// The trait, used for getting list of dependencies from provider of services. Do not use it by
/// yourself.
pub trait GetDependencies<'a, Dependencies: 'a, DepsElems, Indexes> {
    fn get_deps(&'a self) -> Dependencies;
}

mod impls {
    use crate::container::Container;
    use crate::get_dependencies::GetDependencies;
    use crate::resolver::Resolver;
    use crate::ServiceProvider;
    use frunk::hlist::HList;
    use frunk::{HCons, HNil};

    impl<'a, T, TRest, CE, CERest, I, IR, Parent, Deps>
        GetDependencies<'a, HCons<T, TRest>, HCons<CE, CERest>, HCons<I, IR>>
        for ServiceProvider<Parent, Deps>
    where
        CE: Container<T>,
        TRest: HList,
        TRest: 'a,
        T: 'a,
        ServiceProvider<Parent, Deps>:
            Resolver<'a, CE, T, I> + GetDependencies<'a, TRest, CERest, IR>,
    {
        fn get_deps(&'a self) -> HCons<T, TRest> {
            GetDependencies::<TRest, CERest, IR>::get_deps(self).prepend(self.resolve())
        }
    }
    impl<'a, S> GetDependencies<'a, HNil, HNil, HNil> for S {
        fn get_deps(&'a self) -> HNil {
            HNil
        }
    }
}
