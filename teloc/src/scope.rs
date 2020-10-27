use crate::container_elem::{ByRefScopedContainerElem, ContainerElem, ScopedContainerElem};
use crate::{Get, GetDependencies, ServiceProvider};
use frunk::hlist::{HList, Selector};
use frunk::{HCons, HNil};

pub struct Scope<'a, Dependencies, Scoped> {
    container: &'a ServiceProvider<Dependencies, Scoped>,
    scoped: Scoped,
}

impl<'a, Dependencies, Scoped> Scope<'a, Dependencies, Scoped> {
    pub fn new(container: &'a ServiceProvider<Dependencies, Scoped>, scoped: Scoped) -> Self {
        Scope { container, scoped }
    }
}

impl<'a, T, TE, TER, TR, H, S, I, IR>
    GetDependencies<'a, HCons<TE, TER>, HCons<T, TR>, HCons<I, IR>> for Scope<'a, H, S>
where
    TER: HList,
    T: ContainerElem<TE>,
    TE: 'a,
    TER: 'a,
    Scope<'a, H, S>: Get<'a, T, TE, I> + GetDependencies<'a, TER, TR, IR>,
{
    fn get_deps(&'a self) -> HCons<TE, TER> {
        GetDependencies::<TER, TR, IR>::get_deps(self).prepend(self.get())
    }
}

impl<'a, H, S> GetDependencies<'a, HNil, HNil, HNil> for Scope<'a, H, S> {
    fn get_deps(&'a self) -> HNil {
        HNil
    }
}

impl<'a, H, S, T, Index> Get<'a, ScopedContainerElem<T>, T, Index> for Scope<'a, H, S>
where
    T: Clone + 'a,
    S: Selector<T, Index>,
{
    fn get(&'a self) -> T {
        self.scoped.get().clone()
    }
}

impl<'a, H, S, T, Index> Get<'a, ByRefScopedContainerElem<T>, &'a T, Index> for Scope<'a, H, S>
where
    S: Selector<T, Index>,
{
    fn get(&'a self) -> &'a T {
        self.scoped.get().clone()
    }
}

mod scope_container_impls {
    use crate::container_elem::{
        ByRefInstanceContainerElem, ByRefSingletonContainerElem, ConvertContainerElem,
        InstanceContainerElem, SingletonContainerElem, TransientContainerElem,
    };
    use crate::dependency::Dependency;
    use crate::get::Get;
    use crate::{GetDependencies, Scope};
    use frunk::hlist::Selector;

    impl<'a, H, S, T, Index, Deps, DepsElems, Indexes>
        Get<'a, TransientContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)> for Scope<'a, H, S>
    where
        H: Selector<TransientContainerElem<T>, Index>,
        T: Dependency<Deps> + 'a,
        Deps: 'a,
        Scope<'a, H, S>: GetDependencies<'a, Deps, DepsElems, Indexes>,
    {
        fn get(&'a self) -> T {
            T::init(self.get_deps())
        }
    }

    impl<'a, H, S, T, Index, Deps, DepsElems, Indexes>
        Get<'a, SingletonContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)> for Scope<'a, H, S>
    where
        H: Selector<SingletonContainerElem<T>, Index>,
        T: Dependency<Deps> + Clone + 'a,
        Deps: 'a,
        Scope<'a, H, S>: GetDependencies<'a, Deps, DepsElems, Indexes>,
    {
        fn get(&'a self) -> T {
            let dependencies = self.container.dependencies();

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
        for Scope<'a, H, S>
    where
        H: Selector<SingletonContainerElem<T>, Index>,
        T: Dependency<Deps> + Clone + 'a,
        Deps: 'a,
        Scope<'a, H, S>: GetDependencies<'a, Deps, DepsElems, Indexes>,
    {
        fn get(&'a self) -> &'a T {
            let dependencies = self.container.dependencies();

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

    impl<'a, H, S, T, Index> Get<'a, ByRefInstanceContainerElem<T>, &'a T, Index> for Scope<'a, H, S>
    where
        H: Selector<InstanceContainerElem<T>, Index>,
    {
        fn get(&'a self) -> &'a T {
            let elem = self.container.dependencies().get();
            elem.get()
        }
    }

    impl<'a, H, S, T, Index> Get<'a, InstanceContainerElem<T>, T, Index> for Scope<'a, H, S>
    where
        H: Selector<InstanceContainerElem<T>, Index>,
        T: Clone + 'a,
    {
        fn get(&'a self) -> T {
            self.container.dependencies().get().get().clone()
        }
    }

    impl<'a, H, S, T, U, Index, Deps, DepsElems, Indexes>
        Get<
            'a,
            ConvertContainerElem<TransientContainerElem<T>, T, U>,
            U,
            (Index, Deps, DepsElems, Indexes),
        > for Scope<'a, H, S>
    where
        H: Selector<ConvertContainerElem<TransientContainerElem<T>, T, U>, Index>,
        T: Into<U> + Dependency<Deps>,
        U: 'a,
        Deps: 'a,
        Scope<'a, H, S>: GetDependencies<'a, Deps, DepsElems, Indexes>,
    {
        fn get(&'a self) -> U {
            let res = T::init(self.get_deps());
            res.into()
        }
    }
}
