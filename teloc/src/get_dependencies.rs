pub trait GetDependencies<'a, Dependencies: 'a, DepsElems, Indexes> {
    fn get_deps(&'a self) -> Dependencies;
}

mod impls {
    use crate::container::Container;
    use crate::resolver::Resolver;
    use crate::get_dependencies::GetDependencies;
    use frunk::hlist::HList;
    use frunk::{HCons, HNil};

    impl<'a, T, TRest, CE, CERest, SP, I, IR>
        GetDependencies<'a, HCons<T, TRest>, HCons<CE, CERest>, HCons<I, IR>> for SP
    where
        TRest: HList,
        CE: Container<T>,
        TRest: 'a,
        T: 'a,
        SP: Resolver<'a, CE, T, SP, I> + GetDependencies<'a, TRest, CERest, IR>,
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
