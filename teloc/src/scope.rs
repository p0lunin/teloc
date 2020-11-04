//! This module contains `Scope` data type and such containers related to `Scoped` lifetime.

use crate::container::{Container, Init};
use crate::dependency::DependencyClone;
use crate::get_dependencies::GetDependencies;
use crate::{Dependency, Resolver};
use frunk::hlist::{h_cons, HList, Selector};
use frunk::{HCons, HNil};
use once_cell::sync::OnceCell;
use std::marker::PhantomData;

/// Scope is a service provider, used for scoped dependencies. For instantiate `Scope` use
/// `ServiceProvider::scope` function. Scoped services can depend on `Transient` and `Singleton`
/// lifetimes.
pub struct Scope<'a, SP, Scoped, ScopedI> {
    pub(crate) container: &'a SP,
    scoped: Scoped,
    scoped_i: ScopedI,
}

impl<'a, SP, Scoped, SI> Scope<'a, SP, Scoped, SI>
where
    Scoped: InitScoped,
{
    pub(crate) fn new(container: &'a SP, scoped_i: SI) -> Self {
        Scope {
            container,
            scoped: Scoped::init(),
            scoped_i,
        }
    }
}

pub trait InitScoped {
    fn init() -> Self;
}

impl<T, H> InitScoped for HCons<ScopedContainerElem<T>, H>
where
    H: InitScoped + HList,
{
    fn init() -> Self {
        h_cons(ScopedContainerElem::init(()), H::init())
    }
}

impl InitScoped for HNil {
    fn init() -> Self {
        HNil
    }
}

impl<'a, SP, Uninit, Init, T, Index> Selector<T, Index> for Scope<'a, SP, Init, Uninit>
where
    SP: Selector<T, Index>,
{
    fn get(&self) -> &T {
        self.container.get()
    }

    fn get_mut(&mut self) -> &mut T {
        unreachable!("NEVER USE THIS FUNCTION")
    }
}

pub struct ScopedContainerElem<T>(OnceCell<T>);
impl<T> Container<T> for ScopedContainerElem<T> {}
impl<T> Init for ScopedContainerElem<T> {
    type Data = ();

    fn init(_: Self::Data) -> Self {
        Self(OnceCell::new())
    }
}
impl<'a, SP, S, SI, T, Index, Deps, DepsElems, Indexes>
    Resolver<'a, ScopedContainerElem<T>, T, Scope<'a, SP, S, SI>, (Index, Deps, DepsElems, Indexes)>
    for Scope<'a, SP, S, SI>
where
    Deps: 'a,
    S: Selector<ScopedContainerElem<T>, Index>,
    T: Dependency<Deps> + DependencyClone + 'a,
    Self: GetDependencies<'a, Deps, DepsElems, Indexes>,
{
    fn resolve(&'a self) -> T {
        let elem = self.scoped.get();
        let elem_ref = elem.0.get();
        match elem_ref {
            None => {
                let needed = self.get_deps();
                let dep = T::init(needed);
                match elem.0.set(dep.clone()) {
                    Ok(()) => {}
                    Err(_) => unreachable!("Should never been reached"),
                }
                dep
            }
            Some(dep) => dep.clone(),
        }
    }
}

pub struct ScopedInstanceContainer<T>(T);
impl<T> Container<T> for ScopedInstanceContainer<T> {}
impl<T> Init for ScopedInstanceContainer<T> {
    type Data = T;

    fn init(t: Self::Data) -> Self {
        Self(t)
    }
}
impl<'a, SP, S, SI, T, Index>
    Resolver<'a, ScopedInstanceContainer<T>, T, Scope<'a, SP, S, SI>, Index>
    for Scope<'a, SP, S, SI>
where
    SI: Selector<ScopedInstanceContainer<T>, Index>,
    T: DependencyClone + 'a,
{
    fn resolve(&'a self) -> T {
        self.scoped_i.get().0.clone()
    }
}

pub struct ByRefScopedContainer<T>(PhantomData<T>);
impl<T> Container<&T> for ByRefScopedContainer<T> {}
impl<'a, SP, S, SI, T, Index, Deps, DepsElems, Indexes>
    Resolver<
        'a,
        ByRefScopedContainer<T>,
        &'a T,
        Scope<'a, SP, S, SI>,
        (Index, Deps, DepsElems, Indexes),
    > for Scope<'a, SP, S, SI>
where
    Deps: 'a,
    S: Selector<ScopedContainerElem<T>, Index>,
    T: Dependency<Deps>,
    Self: GetDependencies<'a, Deps, DepsElems, Indexes>,
{
    fn resolve(&'a self) -> &'a T {
        let elem = self.scoped.get();
        let elem_ref = elem.0.get();
        match elem_ref {
            None => {
                let needed = self.get_deps();
                let dep = T::init(needed);
                match elem.0.set(dep) {
                    Ok(()) => {}
                    Err(_) => unreachable!("Should never been reached"),
                }
                elem.0.get().expect("Should never been failed")
            }
            Some(dep) => dep,
        }
    }
}

pub struct ByRefScopedInstanceContainer<T>(PhantomData<T>);
impl<T> Container<&T> for ByRefScopedInstanceContainer<T> {}
impl<'a, SP, S, SI, T, Index>
    Resolver<'a, ByRefScopedInstanceContainer<T>, &'a T, Scope<'a, SP, S, SI>, Index>
    for Scope<'a, SP, S, SI>
where
    SI: Selector<ScopedInstanceContainer<T>, Index>,
{
    fn resolve(&'a self) -> &'a T {
        &self.scoped_i.get().0
    }
}
