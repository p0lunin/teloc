use crate::container_elem::{ContainerElem, Init};
use crate::dependency::DependencyClone;
use crate::{Dependency, Get, GetDependencies};
use frunk::hlist::{h_cons, HList, Selector};
use frunk::{HCons, HNil};
use once_cell::sync::OnceCell;
use std::marker::PhantomData;

pub struct Scope<'a, SP, Scoped, ScopedI> {
    pub(crate) container: &'a SP,
    scoped: Scoped,
    scoped_i: ScopedI,
}

pub trait InitScope<'a, SP, SI>: Sized {
    fn new(container: &'a SP, scoped_i: SI) -> Self;
}

impl<'a, SP, Scoped, SI> InitScope<'a, SP, SI> for Scope<'a, SP, Scoped, SI>
where
    Scoped: InitScoped,
{
    fn new(container: &'a SP, scoped_i: SI) -> Self {
        let scope = Scope {
            container,
            scoped: Scoped::init(),
            scoped_i,
        };
        scope
    }
}

pub trait ScopeResolve<'a, Other> {
    fn resolve_scope(&'a self);
}

impl<'a, SP, Scoped, SI, Other> ScopeResolve<'a, Other> for Scope<'a, SP, Scoped, SI>
where
    Scoped: ResolveDependencies<'a, Self, Other>,
{
    fn resolve_scope(&'a self) {
        self.scoped.resolve_deps(self)
    }
}

pub trait ResolveDependencies<'a, Scope, Other> {
    fn resolve_deps(&self, scope: &'a Scope);
}

impl<'a, SP, Scoped, SI, T, Rest, Deps, DepsElems, Indexes, OtherRest>
    ResolveDependencies<'a, Scope<'a, SP, Scoped, SI>, (Deps, DepsElems, Indexes, OtherRest)>
    for HCons<ScopedContainerElem<T>, Rest>
where
    Rest: ResolveDependencies<'a, Scope<'a, SP, Scoped, SI>, OtherRest>,
    T: Dependency<Deps>,
    Deps: 'a,
    Scope<'a, SP, Scoped, SI>: GetDependencies<'a, Deps, DepsElems, Indexes>,
{
    fn resolve_deps(&self, scope: &'a Scope<'a, SP, Scoped, SI>) {
        let HCons { head, tail } = self;
        tail.resolve_deps(scope);
        match head.0.set(T::init(scope.get_deps())) {
            Ok(_) => {}
            Err(_) => unreachable!(),
        }
    }
}

impl<'a, SP, Scoped, SI> ResolveDependencies<'a, Scope<'a, SP, Scoped, SI>, ()> for HNil {
    fn resolve_deps(&self, _: &'a Scope<'a, SP, Scoped, SI>) {}
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

    // NEVER USE THIS FUNCTION
    fn get_mut(&mut self) -> &mut T {
        unreachable!()
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
impl<'a, SP, S, SI, T, Index> Get<'a, ScopedContainerElem<T>, T, Scope<'a, SP, S, SI>, Index>
    for Scope<'a, SP, S, SI>
where
    S: Selector<ScopedContainerElem<T>, Index>,
    T: DependencyClone + 'a,
{
    fn resolve(&'a self) -> T {
        self.scoped
            .get()
            .0
            .get()
            .expect("Should never been failed due to type check")
            .clone()
    }
}

pub struct ScopedInstanceContainerElem<T>(T);
impl<T> ContainerElem<T> for ScopedInstanceContainerElem<T> {}
impl<T> Init for ScopedInstanceContainerElem<T> {
    type Data = T;

    fn init(t: Self::Data) -> Self {
        Self(t)
    }
}
impl<'a, SP, S, SI, T, Index>
    Get<'a, ScopedInstanceContainerElem<T>, T, Scope<'a, SP, S, SI>, Index> for Scope<'a, SP, S, SI>
where
    SI: Selector<ScopedInstanceContainerElem<T>, Index>,
    T: DependencyClone + 'a,
{
    fn resolve(&'a self) -> T {
        self.scoped_i.get().0.clone()
    }
}

pub struct ByRefScopedContainerElem<T>(PhantomData<T>);
impl<T> ContainerElem<&T> for ByRefScopedContainerElem<T> {}
impl<'a, SP, S, SI, T, Index>
    Get<'a, ByRefScopedContainerElem<T>, &'a T, Scope<'a, SP, S, SI>, Index>
    for Scope<'a, SP, S, SI>
where
    S: Selector<ScopedContainerElem<T>, Index>,
{
    fn resolve(&'a self) -> &'a T {
        self.scoped
            .get()
            .0
            .get()
            .expect("Should never been failed due to type check")
    }
}

pub struct ByRefScopedInstanceContainerElem<T>(PhantomData<T>);
impl<T> ContainerElem<&T> for ByRefScopedInstanceContainerElem<T> {}
impl<'a, SP, S, SI, T, Index>
    Get<'a, ByRefScopedInstanceContainerElem<T>, &'a T, Scope<'a, SP, S, SI>, Index>
    for Scope<'a, SP, S, SI>
where
    SI: Selector<ScopedInstanceContainerElem<T>, Index>,
{
    fn resolve(&'a self) -> &'a T {
        &self.scoped_i.get().0
    }
}