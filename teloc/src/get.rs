use crate::container_elem::ContainerElem;

pub trait Get<'a, T: ContainerElem<TE>, TE, Index>
where
    TE: 'a,
{
    fn get(&'a self) -> TE;
}

mod impls {
    use crate::container::Container;
    use crate::container_elem::{
        ByRefInstanceContainerElem, ByRefSingletonContainerElem, InstanceContainerElem,
        SingletonContainerElem, TransientContainerElem,
    };
    use crate::dependency::Dependency;
    use crate::get::Get;
    use crate::GetDependencies;
    use frunk::hlist::Selector;

    impl<'a, H, T, Index, Deps, DepsElems, Indexes>
        Get<'a, TransientContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)> for Container<H>
    where
        H: Selector<TransientContainerElem<T>, Index>,
        T: Dependency<Deps> + 'a,
        Deps: 'a,
        Container<H>: GetDependencies<'a, Deps, DepsElems, Indexes>,
    {
        fn get(&'a self) -> T {
            let res = T::init(self.get_deps());
            res
        }
    }

    impl<'a, H, T, Index, Deps, DepsElems, Indexes>
        Get<'a, SingletonContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)> for Container<H>
    where
        H: Selector<SingletonContainerElem<T>, Index>,
        T: Dependency<Deps> + Clone + 'a,
        Deps: 'a,
        Container<H>: GetDependencies<'a, Deps, DepsElems, Indexes>,
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
    impl<'a, H, T, Index, Deps, DepsElems, Indexes>
        Get<'a, ByRefSingletonContainerElem<T>, &'a T, (Index, Deps, DepsElems, Indexes)>
        for Container<H>
    where
        H: Selector<SingletonContainerElem<T>, Index>,
        T: Dependency<Deps> + Clone + 'a,
        Deps: 'a,
        Container<H>: GetDependencies<'a, Deps, DepsElems, Indexes>,
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

    impl<'a, H, T, Index> Get<'a, ByRefInstanceContainerElem<T>, &'a T, Index> for Container<H>
    where
        H: Selector<InstanceContainerElem<T>, Index>,
    {
        fn get(&'a self) -> &'a T {
            let elem = self.dependencies().get();
            elem.get()
        }
    }

    impl<'a, H, T, Index> Get<'a, InstanceContainerElem<T>, T, Index> for Container<H>
    where
        H: Selector<InstanceContainerElem<T>, Index>,
        T: Clone + 'a,
    {
        fn get(&'a self) -> T {
            self.dependencies().get().get().clone()
        }
    }
}
