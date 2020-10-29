use crate::container_elem::ContainerElem;

pub trait Get<'a, T: ContainerElem<TE>, TE, Index>
where
    TE: 'a,
{
    fn get(&'a self) -> TE;
}

mod impls {
    use crate::container_elem::{ByRefInstanceContainerElem, ByRefSingletonContainerElem, ConvertContainerElem, InstanceContainerElem, SingletonContainerElem, TransientContainerElem};
    use crate::dependency::{Dependency, DependencyClone};
    use crate::get::Get;
    use crate::{GetDependencies};
    use frunk::hlist::Selector;

    impl<'a, S, T, Index, Deps, DepsElems, Indexes>
        Get<'a, TransientContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)>
        for S
    where
        S: GetDependencies<'a, Deps, DepsElems, Indexes>,
        T: Dependency<Deps> + 'a,
        Deps: 'a,
    {
        fn get(&'a self) -> T {
            T::init(self.get_deps())
        }
    }

    impl<'a, S, T, Index, Deps, DepsElems, Indexes>
        Get<'a, SingletonContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)>
        for S
    where
        S: Selector<SingletonContainerElem<T>, Index> + GetDependencies<'a, Deps, DepsElems, Indexes>,
        T: Dependency<Deps> + DependencyClone + 'a,
        Deps: 'a,
    {
        fn get(&'a self) -> T {
            let elem = self.get();
            let elem_ref = elem.get().get();
            match elem_ref {
                None => {
                    let needed = self.get_deps();
                    let dep = T::init(needed);
                    match elem.get().set(dep.clone()) {
                        Ok(()) => {}
                        Err(_) => unreachable!("Should never been reached"),
                    }
                    dep
                }
                Some(dep) => dep.clone(),
            }
        }
    }
    impl<'a, S, T, Index, Deps, DepsElems, Indexes>
        Get<'a, ByRefSingletonContainerElem<T>, &'a T, (Index, Deps, DepsElems, Indexes)>
        for S
    where
        S: Selector<SingletonContainerElem<T>, Index> + GetDependencies<'a, Deps, DepsElems, Indexes>,
        T: Dependency<Deps> + 'a,
        Deps: 'a,
    {
        fn get(&'a self) -> &'a T {
            let elem = self.get();
            let elem_ref = elem.get().get();
            match elem_ref {
                None => {
                    let needed = self.get_deps();
                    let dep = T::init(needed);
                    match elem.get().set(dep) {
                        Ok(()) => {}
                        Err(_) => unreachable!("Should never been reached"),
                    }
                    elem.get().get().expect("Should never been failed")
                }
                Some(dep) => dep,
            }
        }
    }

    impl<'a, S, T, Index> Get<'a, ByRefInstanceContainerElem<T>, &'a T, Index>
        for S
    where
        S: Selector<InstanceContainerElem<T>, Index>,
    {
        fn get(&'a self) -> &'a T {
            let elem = self.get();
            elem.get()
        }
    }

    impl<'a, S, T, Index> Get<'a, InstanceContainerElem<T>, T, Index> for S
    where
        S: Selector<InstanceContainerElem<T>, Index>,
        T: Clone + 'a,
    {
        fn get(&'a self) -> T {
            self.get().get().clone()
        }
    }

    impl<'a, S, T, U, Index, Deps, DepsElems, Indexes>
        Get<
            'a,
            ConvertContainerElem<TransientContainerElem<T>, T, U>,
            U,
            (Index, Deps, DepsElems, Indexes),
        > for S
    where
        S: Selector<ConvertContainerElem<TransientContainerElem<T>, T, U>, Index>
            + GetDependencies<'a, Deps, DepsElems, Indexes>,
        T: Into<U> + Dependency<Deps>,
        U: 'a,
        Deps: 'a,
    {
        fn get(&'a self) -> U {
            let res = T::init(self.get_deps());
            res.into()
        }
    }
}
