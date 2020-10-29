pub trait GetDependencies<'a, Dependencies: 'a, DepElems, Indexes> {
    fn get_deps(&'a self) -> Dependencies;
}

mod impls {
    use crate::container_elem::ContainerElem;
    use crate::get::Get;
    use crate::GetDependencies;
    use frunk::hlist::HList;
    use frunk::{HCons, HNil};

    impl<'a, T, TE, TER, TR, S, I, IR>
        GetDependencies<'a, HCons<TE, TER>, HCons<T, TR>, HCons<I, IR>> for S
    where
        TER: HList,
        T: ContainerElem<TE>,
        TE: 'a,
        TER: 'a,
        S: Get<'a, T, TE, I> + GetDependencies<'a, TER, TR, IR>,
    {
        fn get_deps(&'a self) -> HCons<TE, TER> {
            GetDependencies::<TER, TR, IR>::get_deps(self).prepend(self.get())
        }
    }
    impl<'a, S> GetDependencies<'a, HNil, HNil, HNil> for S {
        fn get_deps(&'a self) -> HNil {
            HNil
        }
    }
}
