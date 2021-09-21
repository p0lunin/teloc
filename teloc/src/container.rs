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
pub trait Container {
    type Data;
    fn init(data: Self::Data) -> Self;
}

/// Trait needed primary to working with `ConvertContainer`. Implement it for your container if you
/// wish that your container can be placed inside of `ConvertContainer`
pub trait ResolveContainer<'a, T, Cont, Deps> {
    fn resolve_container<F: Fn() -> Deps>(ct: &'a Cont, deps: F) -> T;
}

#[derive(Debug)]
pub struct TransientContainer<T>(PhantomData<T>);
impl<T> Container for TransientContainer<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}
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

#[derive(Debug)]
pub struct SingletonContainer<T>(OnceCell<T>);
impl<T> Container for SingletonContainer<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(OnceCell::new())
    }
}
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

#[derive(Debug)]
pub struct InstanceContainer<T>(T);
impl<T> Container for InstanceContainer<T> {
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

#[derive(Debug)]
pub struct ByRefSingletonContainer<T>(PhantomData<T>);
impl<T> Container for ByRefSingletonContainer<T> {
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

#[derive(Debug)]
pub struct ByRefInstanceContainer<T>(PhantomData<T>);
impl<T> Container for ByRefInstanceContainer<T> {
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

pub struct ConvertContainer<Cont, T, U>(Cont, PhantomData<(T, U)>);
impl<Cont, T, U> Container for ConvertContainer<Cont, T, U>
where
    Cont: Container,
{
    type Data = Cont::Data;

    fn init(data: Self::Data) -> Self {
        Self(Cont::init(data), PhantomData)
    }
}
impl<'a, Cont, T, U, Deps> ResolveContainer<'a, U, Self, Deps> for ConvertContainer<Cont, T, U>
where
    Cont: ResolveContainer<'a, T, Cont, Deps>,
    T: Into<U>,
{
    fn resolve_container<F: Fn() -> Deps>(ct: &'a Self, deps: F) -> U {
        Cont::resolve_container(&ct.0, deps).into()
    }
}
impl<'a, Cont, T, U, SP, Index, Deps, Infer>
    Resolver<'a, ConvertContainer<Cont, T, U>, U, (Index, Deps, Infer)> for SP
where
    U: 'a,
    Deps: 'a,
    Cont: 'a,
    T: Into<U> + 'a,
    ConvertContainer<Cont, T, U>: ResolveContainer<'a, U, ConvertContainer<Cont, T, U>, Deps>,
    SP: Selector<ConvertContainer<Cont, T, U>, Index> + GetDependencies<'a, Deps, Infer>,
{
    fn resolve(&'a self) -> U {
        ConvertContainer::resolve_container(self.get(), || self.get_deps())
    }
}
impl<Cont, ContT, T> ConvertContainer<Cont, ContT, T> {
    #[inline]
    pub fn get(&self) -> &Cont {
        &self.0
    }
}
