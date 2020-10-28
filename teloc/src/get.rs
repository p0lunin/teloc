use crate::container_elem::ContainerElem;

pub trait Get<'a, T: ContainerElem<TE>, TE, Index>
where
    TE: 'a,
{
    fn get(&'a self) -> TE;
}

mod impls {
    use crate::container_elem::{ByRefInstanceContainerElem, ByRefSingletonContainerElem, ConvertContainerElem, InstanceContainerElem, SingletonContainerElem, TransientContainerElem, ScopedContainerElem, ByRefScopedContainerElem};
    use crate::dependency::{Dependency, DependencyClone};
    use crate::get::Get;
    use crate::service_provider::ServiceProvider;
    use crate::{GetDependencies, Scope};
    use frunk::hlist::Selector;
    use frunk::HNil;

    impl<'a, H, S, T, Index, Deps, DepsElems, Indexes>
        Get<'a, TransientContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)>
        for ServiceProvider<H, S>
    where
        H: Selector<TransientContainerElem<T>, Index>,
        T: Dependency<Deps> + 'a,
        Deps: 'a,
        ServiceProvider<H, S>: GetDependencies<'a, Deps, DepsElems, Indexes>,
    {
        fn get(&'a self) -> T {
            T::init(self.get_deps())
        }
    }

    impl<'a, H, S, T, Index, Deps, DepsElems, Indexes>
        Get<'a, SingletonContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)>
        for ServiceProvider<H, S>
    where
        H: Selector<SingletonContainerElem<T>, Index>,
        T: Dependency<Deps> + DependencyClone + 'a,
        Deps: 'a,
        ServiceProvider<H, S>: GetDependencies<'a, Deps, DepsElems, Indexes>,
    {
        fn get(&'a self) -> T {
            let dependencies = self.dependencies();

            let elem = dependencies.get();
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
    impl<'a, H, S, T, Index, Deps, DepsElems, Indexes>
        Get<'a, ByRefSingletonContainerElem<T>, &'a T, (Index, Deps, DepsElems, Indexes)>
        for ServiceProvider<H, S>
    where
        H: Selector<SingletonContainerElem<T>, Index>,
        T: Dependency<Deps> + 'a,
        Deps: 'a,
        ServiceProvider<H, S>: GetDependencies<'a, Deps, DepsElems, Indexes>,
    {
        fn get(&'a self) -> &'a T {
            let dependencies = self.dependencies();

            let elem = dependencies.get();
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

    impl<'a, H, S, T, Index> Get<'a, ByRefInstanceContainerElem<T>, &'a T, Index>
        for ServiceProvider<H, S>
    where
        H: Selector<InstanceContainerElem<T>, Index>,
    {
        fn get(&'a self) -> &'a T {
            let elem = self.dependencies().get();
            elem.get()
        }
    }

    impl<'a, H, S, T, Index> Get<'a, InstanceContainerElem<T>, T, Index> for ServiceProvider<H, S>
    where
        H: Selector<InstanceContainerElem<T>, Index>,
        T: Clone + 'a,
    {
        fn get(&'a self) -> T {
            self.dependencies().get().get().clone()
        }
    }

    impl<'a, H, S, T, U, Index, Deps, DepsElems, Indexes>
        Get<
            'a,
            ConvertContainerElem<TransientContainerElem<T>, T, U>,
            U,
            (Index, Deps, DepsElems, Indexes),
        > for ServiceProvider<H, S>
    where
        H: Selector<ConvertContainerElem<TransientContainerElem<T>, T, U>, Index>,
        T: Into<U> + Dependency<Deps>,
        U: 'a,
        Deps: 'a,
        ServiceProvider<H, S>: GetDependencies<'a, Deps, DepsElems, Indexes>,
    {
        fn get(&'a self) -> U {
            let res = T::init(self.get_deps());
            res.into()
        }
    }

    impl<'a, H, T, Index> Get<'a, ScopedContainerElem<T>, T, Index> for ServiceProvider<H>
    where
        T: DependencyClone + 'a,
        H: Selector<T, Index>,
    {
        fn get(&'a self) -> T {
            self.dependencies().get().clone()
        }
    }

    impl<'a, H, T, Index> Get<'a, ByRefScopedContainerElem<T>, &'a T, Index> for ServiceProvider<H>
    where
        H: Selector<T, Index>,
    {
        fn get(&'a self) -> &'a T {
            self.dependencies().get()
        }
    }
}
