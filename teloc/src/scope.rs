use crate::ServiceProvider;
use once_cell::sync::OnceCell;
use crate::container_elem::{Init, ContainerElem};
use frunk::hlist::Selector;
use frunk::HNil;
use std::marker::PhantomData;

pub struct Scope<'a, Dependencies> {
    pub(crate) container: &'a ServiceProvider<Dependencies>,
}

impl<'a, Dependencies> Scope<'a, Dependencies> {
    pub fn new(container: &'a ServiceProvider<Dependencies>) -> Self {
        Scope { container }
    }
}

pub struct ScopedContainerElem<T>(OnceCell<T>);
impl<T> ContainerElem<T> for ScopedContainerElem<T> {}
impl<T> Init for ScopedContainerElem<T> {
    type Data = ();

    fn init(_: Self::Data) -> Self {
        Self(OnceCell::new())
    }
}
/*impl<T> ContainerElem<T> for ScopedContainerElem<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }

    fn resolve(service_provider: _) -> T {
        unimplemented!()
    }
}

pub struct ByRefScopedContainerElem<T>(PhantomData<T>);
impl<T> ContainerElem<&T> for ByRefScopedContainerElem<T> {
    type Data = ();

    fn init(_: Self::Data) -> Self {
        Self(PhantomData)
    }
}*/

/*
impl<'a, H, T, Index> Get<'a, ScopedContainerElem<T>, T, Index> for Scope<'a, H>
where
    T: DependencyClone + 'a,
    H: Selector<T, Index>,
{
    fn get(&'a self) -> T {
        self.get().clone()
    }
}

impl<'a, H, T, Index> Get<'a, ByRefScopedContainerElem<T>, &'a T, Index> for ServiceProvider<H>
where
    H: Selector<T, Index>,
{
    fn get(&'a self) -> &'a T {
        self.get()
    }
}*/
/*
impl<'a, T, TE, TER, TR, H, I, IR>
    GetDependencies<'a, HCons<TE, TER>, HCons<T, TR>, HCons<I, IR>> for Scope<'a, H>
where
    TER: HList,
    T: ContainerElem<TE>,
    TE: 'a,
    TER: 'a,
    Scope<'a, H>: Get<'a, T, TE, I> + GetDependencies<'a, TER, TR, IR>,
{
    fn get_deps(&'a self) -> HCons<TE, TER> {
        GetDependencies::<TER, TR, IR>::get_deps(self).prepend(self.get())
    }
}

impl<'a, H> GetDependencies<'a, HNil, HNil, HNil> for Scope<'a, H> {
    fn get_deps(&'a self) -> HNil {
        HNil
    }
}*/
/*
mod scope_container_impls {
    use crate::container_elem::{
        ByRefInstanceContainerElem, ByRefSingletonContainerElem, ConvertContainerElem,
        InstanceContainerElem, SingletonContainerElem, TransientContainerElem,
    };
    use crate::dependency::Dependency;
    use crate::get::Get;
    use crate::{GetDependencies, Scope};
    use frunk::hlist::Selector;

    impl<'a, H, SU, S, T, Index, Deps, DepsElems, Indexes>
        Get<'a, TransientContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)> for Scope<'a, H, SU, S>
    where
        H: Selector<TransientContainerElem<T>, Index>,
        T: Dependency<Deps> + 'a,
        Deps: 'a,
        Scope<'a, H, SU, S>: GetDependencies<'a, Deps, DepsElems, Indexes>,
    {
        fn get(&'a self) -> T {
            T::init(self.get_deps())
        }
    }

    impl<'a, H, SU, S, T, Index, Deps, DepsElems, Indexes>
        Get<'a, SingletonContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)> for Scope<'a, H, SU, S>
    where
        H: Selector<SingletonContainerElem<T>, Index>,
        T: Dependency<Deps> + Clone + 'a,
        Deps: 'a,
        Scope<'a, H, SU, S>: GetDependencies<'a, Deps, DepsElems, Indexes>,
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
    impl<'a, H, SU, S, T, Index, Deps, DepsElems, Indexes>
        Get<'a, ByRefSingletonContainerElem<T>, &'a T, (Index, Deps, DepsElems, Indexes)>
        for Scope<'a, H, SU, S>
    where
        H: Selector<SingletonContainerElem<T>, Index>,
        T: Dependency<Deps> + Clone + 'a,
        Deps: 'a,
        Scope<'a, H, SU, S>: GetDependencies<'a, Deps, DepsElems, Indexes>,
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

    impl<'a, H, SU, S, T, Index> Get<'a, ByRefInstanceContainerElem<T>, &'a T, Index> for Scope<'a, H, SU, S>
    where
        H: Selector<InstanceContainerElem<T>, Index>,
    {
        fn get(&'a self) -> &'a T {
            let elem = self.container.dependencies().get();
            elem.get()
        }
    }

    impl<'a, H, SU, S, T, Index> Get<'a, InstanceContainerElem<T>, T, Index> for Scope<'a, H, SU, S>
    where
        H: Selector<InstanceContainerElem<T>, Index>,
        T: Clone + 'a,
    {
        fn get(&'a self) -> T {
            self.container.dependencies().get().get().clone()
        }
    }

    impl<'a, H, SU, S, T, U, Index, Deps, DepsElems, Indexes>
        Get<
            'a,
            ConvertContainerElem<TransientContainerElem<T>, T, U>,
            U,
            (Index, Deps, DepsElems, Indexes),
        > for Scope<'a, H, SU, S>
    where
        H: Selector<ConvertContainerElem<TransientContainerElem<T>, T, U>, Index>,
        T: Into<U> + Dependency<Deps>,
        U: 'a,
        Deps: 'a,
        Scope<'a, H, SU, S>: GetDependencies<'a, Deps, DepsElems, Indexes>,
    {
        fn get(&'a self) -> U {
            let res = T::init(self.get_deps());
            res.into()
        }
    }
}
*/
