use crate::dependency::DependencyClone;
use crate::{Dependency, GetDependencies, Resolver};
use frunk::hlist::Selector;
use frunk::HNil;
use once_cell::sync::OnceCell;
use std::marker::PhantomData;

pub trait Init {
    type Data;
    fn init(data: Self::Data) -> Self;
}
pub trait Container<T> {}
pub trait ResolveContainer<'a, ContGet, Deps> {
    type Output;
    fn resolve_container<F: Fn() -> Deps>(ct: &'a ContGet, get_deps: F) -> Self::Output;
}

pub struct TransientContainer<T>(PhantomData<T>);
impl<T> Init for TransientContainer<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}
impl<T> Container<T> for TransientContainer<T> {}
impl<T, Deps> ResolveContainer<'_, Self, Deps> for TransientContainer<T>
where
    T: Dependency<Deps>,
{
    type Output = T;

    fn resolve_container<F: Fn() -> Deps>(_: &Self, get_deps: F) -> Self::Output {
        T::init(get_deps())
    }
}
impl<'a, T, SP, Index, Deps, DepsElems, Indexes>
    Resolver<'a, TransientContainer<T>, T, SP, (Index, Deps, DepsElems, Indexes)> for SP
where
    T: 'a,
    Deps: 'a,
    SP: Selector<TransientContainer<T>, Index> + GetDependencies<'a, Deps, DepsElems, Indexes> + 'a,
    T: Dependency<Deps> + 'a,
    TransientContainer<T>: ResolveContainer<'a, TransientContainer<T>, Deps, Output = T>,
{
    fn resolve(&'a self) -> T {
        TransientContainer::resolve_container(self.get(), || self.get_deps())
    }
}

pub struct SingletonContainer<T>(OnceCell<T>);
impl<T> Init for SingletonContainer<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(OnceCell::new())
    }
}
impl<T> Container<T> for SingletonContainer<T> {}
impl<T, Deps> ResolveContainer<'_, Self, Deps> for SingletonContainer<T>
where
    T: Dependency<Deps> + DependencyClone,
{
    type Output = T;

    fn resolve_container<F: Fn() -> Deps>(ct: &Self, get_deps: F) -> T {
        let elem_ref = ct.get().get();
        match elem_ref {
            None => {
                let needed = get_deps();
                let dep = T::init(needed);
                match ct.get().set(dep.clone()) {
                    Ok(()) => {}
                    Err(_) => unreachable!("Should never been reached"),
                }
                dep
            }
            Some(dep) => dep.clone(),
        }
    }
}

impl<'a, T, SP, Index, Deps, DepsElems, Indexes>
    Resolver<'a, SingletonContainer<T>, T, SP, (Index, Deps, DepsElems, Indexes)> for SP
where
    SingletonContainer<T>: ResolveContainer<'a, SingletonContainer<T>, Deps, Output = T>,
    T: Dependency<Deps> + 'a,
    Deps: 'a,
    SP: GetDependencies<'a, Deps, DepsElems, Indexes> + Selector<SingletonContainer<T>, Index>,
{
    fn resolve(&'a self) -> T {
        SingletonContainer::resolve_container(self.get(), || self.get_deps())
    }
}
impl<T> SingletonContainer<T> {
    #[inline]
    pub fn get(&self) -> &OnceCell<T> {
        &self.0
    }
}

pub struct InstanceContainer<T>(T);
impl<T> Container<T> for InstanceContainer<T> {}
impl<T> Init for InstanceContainer<T> {
    type Data = T;

    fn init(instance: T) -> Self {
        Self(instance)
    }
}
impl<T> ResolveContainer<'_, Self, HNil> for InstanceContainer<T>
where
    T: DependencyClone,
{
    type Output = T;

    fn resolve_container<F: Fn() -> HNil>(ct: &Self, _: F) -> T {
        ct.0.clone()
    }
}
impl<'a, T, SP, Index> Resolver<'a, InstanceContainer<T>, T, SP, Index> for SP
where
    T: 'a,
    SP: Selector<InstanceContainer<T>, Index>,
    InstanceContainer<T>: ResolveContainer<'a, InstanceContainer<T>, HNil, Output = T>,
{
    fn resolve(&'a self) -> T {
        InstanceContainer::resolve_container(self.get(), || HNil)
    }
}
impl<T> InstanceContainer<T> {
    #[inline]
    pub fn get(&self) -> &T {
        &self.0
    }
}

pub struct ByRefSingletonContainer<T>(PhantomData<T>);
impl<T> Container<&T> for ByRefSingletonContainer<T> {}
impl<T> Init for ByRefSingletonContainer<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}
impl<'a, T, Deps> ResolveContainer<'a, SingletonContainer<T>, Deps> for ByRefSingletonContainer<T>
where
    T: 'a,
    T: Dependency<Deps>,
{
    type Output = &'a T;

    fn resolve_container<F: Fn() -> Deps>(ct: &'a SingletonContainer<T>, get_deps: F) -> &'a T {
        let elem_ref = ct.get().get();
        match elem_ref {
            None => {
                let needed = get_deps();
                let dep = T::init(needed);
                match ct.get().set(dep) {
                    Ok(()) => {}
                    Err(_) => unreachable!("Should never been reached"),
                }
                ct.get().get().expect("Should never been failed")
            }
            Some(dep) => dep,
        }
    }
}

impl<'a, T, SP, Index, Deps, DepsElems, Indexes>
    Resolver<'a, ByRefSingletonContainer<T>, &'a T, SP, (Index, Deps, DepsElems, Indexes)> for SP
where
    T: 'a,
    Deps: 'a,
    SP: Selector<SingletonContainer<T>, Index> + GetDependencies<'a, Deps, DepsElems, Indexes> + 'a,
    ByRefSingletonContainer<T>: ResolveContainer<'a, SingletonContainer<T>, Deps, Output = &'a T>,
{
    fn resolve(&'a self) -> &'a T {
        ByRefSingletonContainer::resolve_container(self.get(), || self.get_deps())
    }
}

pub struct ByRefInstanceContainer<'a, T>(PhantomData<&'a T>);
impl<T> Container<&T> for ByRefInstanceContainer<'_, T> {}
impl<T> Init for ByRefInstanceContainer<'_, T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}
impl<'a, T> ResolveContainer<'a, InstanceContainer<T>, HNil> for ByRefInstanceContainer<'a, T> {
    type Output = &'a T;

    fn resolve_container<F: Fn() -> HNil>(ct: &'a InstanceContainer<T>, _: F) -> &'a T {
        ct.get()
    }
}
impl<'a, T, SP, Index> Resolver<'a, ByRefInstanceContainer<'a, T>, &'a T, SP, Index> for SP
where
    SP: Selector<InstanceContainer<T>, Index>,
    ByRefInstanceContainer<'a, T>: ResolveContainer<'a, InstanceContainer<T>, HNil, Output = &'a T>,
{
    fn resolve(&'a self) -> &'a T {
        ByRefInstanceContainer::resolve_container(self.get(), || HNil)
    }
}

pub struct ConvertContainer<C, CT, T>(C, PhantomData<(CT, T)>);
impl<C, CT, T> Container<T> for ConvertContainer<C, CT, T>
where
    C: Container<CT>,
    CT: Into<T>,
{
}
impl<C, CT, T> Init for ConvertContainer<C, CT, T>
where
    C: Init,
{
    type Data = C::Data;

    fn init(data: Self::Data) -> Self {
        Self(C::init(data), PhantomData)
    }
}
impl<'a, C, CT, T, Deps> ResolveContainer<'a, Self, Deps> for ConvertContainer<C, CT, T>
where
    C: ResolveContainer<'a, C, Deps, Output = CT>,
    CT: Into<T>,
{
    type Output = T;

    fn resolve_container<F: Fn() -> Deps>(ct: &'a Self, deps: F) -> T {
        C::resolve_container(&ct.0, deps).into()
    }
}
impl<'a, C, CT, T, SP, Index, Deps, DepsElems, Indexes>
    Resolver<'a, ConvertContainer<C, CT, T>, T, SP, (Index, Deps, DepsElems, Indexes)> for SP
where
    T: 'a,
    Deps: 'a,
    C: Container<CT> + 'a,
    CT: Into<T> + 'a,
    ConvertContainer<C, CT, T>: ResolveContainer<'a, ConvertContainer<C, CT, T>, Deps, Output = T>,
    SP: Selector<ConvertContainer<C, CT, T>, Index> + GetDependencies<'a, Deps, DepsElems, Indexes>,
{
    fn resolve(&'a self) -> T {
        ConvertContainer::resolve_container(self.get(), || self.get_deps())
    }
}
impl<Cont, ContT, T> ConvertContainer<Cont, ContT, T> {
    #[inline]
    pub fn get(&self) -> &Cont {
        &self.0
    }
}
