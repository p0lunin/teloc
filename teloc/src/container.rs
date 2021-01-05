//! This is a section for advanced usage. For common usage you can not read this page.

use crate::dependency::DependencyClone;
use crate::get_dependencies::GetDependencies;
use crate::{Dependency, Resolver};
use frunk::hlist::Selector;
use frunk::HNil;
use once_cell::sync::OnceCell;
use std::marker::PhantomData;

/// Init is a trait used in [`ServiceProvider`] for create an empty version of `Container`. If you
/// create your own version of container and you want that it can work with other container like
/// [`ConvertContainer`], you must implement this trait.
///
/// [`ServiceProvider`]: teloc::ServiceProvider
/// [`ConvertContainer`]: teloc::container::ConvertContainer
pub trait Init {
    type Data;
    fn init(data: Self::Data) -> Self;
}
/// Container is a trait that used in `Get` trait to indicate a return value. Generic T shows that
/// which type will be returned by `Get` trait.
pub trait Container<T> {}
/// Trait needed primary to working with `ConvertContainer`. Implement it for your container if you
/// wish that your container can be placed inside of `ConvertContainer`
pub trait ResolveContainer<'a, Elem, ContGet, Deps> {
    fn resolve_container<F: Fn() -> Deps>(ct: &'a ContGet, get_deps: F) -> Elem;
}

pub struct TransientContainer<T>(PhantomData<T>);
impl<T> Init for TransientContainer<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}
impl<T> Container<T> for TransientContainer<T> {}
impl<'a, T, Deps> ResolveContainer<'a, T, Self, Deps> for TransientContainer<T>
where
    T: Dependency<Deps>,
{
    fn resolve_container<F: Fn() -> Deps>(_: &'a Self, get_deps: F) -> T {
        T::init(get_deps())
    }
}
impl<'a, T, SP, Index, Deps, Infer> Resolver<'a, TransientContainer<T>, T, (Index, Deps, Infer)>
    for SP
where
    SP: Selector<TransientContainer<T>, Index> + GetDependencies<'a, Deps, Infer>,
    T: Dependency<Deps> + 'a,
    TransientContainer<T>: ResolveContainer<'a, T, TransientContainer<T>, Deps>,
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
impl<T, Deps> ResolveContainer<'_, T, Self, Deps> for SingletonContainer<T>
where
    T: Dependency<Deps> + DependencyClone,
{
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

impl<'a, T, SP, Index, Deps, Infer> Resolver<'a, SingletonContainer<T>, T, (Index, Deps, Infer)>
    for SP
where
    SingletonContainer<T>: ResolveContainer<'a, T, SingletonContainer<T>, Deps>,
    T: Dependency<Deps> + 'a,
    Deps: 'a,
    SP: GetDependencies<'a, Deps, Infer> + Selector<SingletonContainer<T>, Index>,
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
impl<T> ResolveContainer<'_, T, Self, HNil> for InstanceContainer<T>
where
    T: DependencyClone,
{
    fn resolve_container<F: Fn() -> HNil>(ct: &Self, _: F) -> T {
        ct.0.clone()
    }
}
impl<'a, T, SP, Index> Resolver<'a, InstanceContainer<T>, T, Index> for SP
where
    T: 'a,
    SP: Selector<InstanceContainer<T>, Index>,
    InstanceContainer<T>: ResolveContainer<'a, T, InstanceContainer<T>, HNil>,
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
impl<'a, T, Deps> ResolveContainer<'a, &'a T, SingletonContainer<T>, Deps>
    for ByRefSingletonContainer<T>
where
    T: Dependency<Deps>,
{
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

impl<'a, T, SP, Index, Deps, Infer>
    Resolver<'a, ByRefSingletonContainer<T>, &'a T, (Index, Deps, Infer)> for SP
where
    T: 'a,
    SP: Selector<SingletonContainer<T>, Index> + GetDependencies<'a, Deps, Infer>,
    ByRefSingletonContainer<T>: ResolveContainer<'a, &'a T, SingletonContainer<T>, Deps>,
{
    fn resolve(&'a self) -> &'a T {
        ByRefSingletonContainer::resolve_container(self.get(), || self.get_deps())
    }
}

pub struct ByRefInstanceContainer<T>(PhantomData<T>);
impl<'a, T> Container<&'a T> for ByRefInstanceContainer<T> {}
impl<T> Init for ByRefInstanceContainer<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}
impl<'a, T> ResolveContainer<'a, &'a T, InstanceContainer<T>, HNil> for ByRefInstanceContainer<T> {
    fn resolve_container<F: Fn() -> HNil>(ct: &'a InstanceContainer<T>, _: F) -> &'a T {
        ct.get()
    }
}
impl<'a, T, SP, Index> Resolver<'a, ByRefInstanceContainer<T>, &'a T, Index> for SP
where
    SP: Selector<InstanceContainer<T>, Index>,
    ByRefInstanceContainer<T>: ResolveContainer<'a, &'a T, InstanceContainer<T>, HNil>,
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
impl<'a, C, CT, T, Deps> ResolveContainer<'a, T, Self, Deps> for ConvertContainer<C, CT, T>
where
    C: ResolveContainer<'a, CT, C, Deps>,
    CT: Into<T>,
{
    fn resolve_container<F: Fn() -> Deps>(ct: &'a Self, deps: F) -> T {
        C::resolve_container(&ct.0, deps).into()
    }
}
impl<'a, C, CT, T, SP, Index, Deps, Infer>
    Resolver<'a, ConvertContainer<C, CT, T>, T, (Index, Deps, Infer)> for SP
where
    T: 'a,
    Deps: 'a,
    C: Container<CT> + 'a,
    CT: Into<T> + 'a,
    ConvertContainer<C, CT, T>: ResolveContainer<'a, T, ConvertContainer<C, CT, T>, Deps>,
    SP: Selector<ConvertContainer<C, CT, T>, Index> + GetDependencies<'a, Deps, Infer>,
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
