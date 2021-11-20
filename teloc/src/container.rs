use crate::dependency::DependencyClone;
use crate::get_dependencies::GetDependencies;
use crate::service_provider::SelectContainer;
use crate::{Dependency, Resolver};
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
pub trait ResolveContainer<'a, T, Deps> {
    fn resolve_container<F: Fn() -> Deps>(ct: &'a Self, deps: F) -> T;
}

#[derive(Debug)]
pub struct TransientContainer<T>(PhantomData<T>);
impl<T> Container for TransientContainer<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}
impl<'a, T, Deps> ResolveContainer<'a, T, Deps> for TransientContainer<T>
where
    T: Dependency<Deps>,
{
    fn resolve_container<F: Fn() -> Deps>(_: &'a Self, get_deps: F) -> T {
        T::init(get_deps())
    }
}
impl<'this, 'cont, T, SP, Index, Deps, Infer>
    Resolver<'this, &'cont TransientContainer<T>, T, (Index, Deps, Infer)> for SP
where
    SP: SelectContainer<'this, &'cont TransientContainer<T>, Index>
        + GetDependencies<'this, Deps, Infer>,
    TransientContainer<T>: ResolveContainer<'cont, T, Deps>,
    T: Dependency<Deps> + 'cont,
{
    fn resolve(&'this self) -> T {
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
impl<'a, T, Deps> ResolveContainer<'a, &'a T, Deps> for SingletonContainer<T>
where
    T: Dependency<Deps> + 'a,
{
    fn resolve_container<F: Fn() -> Deps>(ct: &'a Self, get_deps: F) -> &'a T {
        ct.get().get_or_init(|| T::init(get_deps()))
    }
}

impl<'this, 'cont, T, SP, Index, Deps, Infer>
    Resolver<'this, &'cont SingletonContainer<T>, T, (Index, Deps, Infer)> for SP
where
    SP: GetDependencies<'this, Deps, Infer>
        + SelectContainer<'this, &'cont SingletonContainer<T>, Index>,
    SingletonContainer<T>: ResolveContainer<'cont, &'cont T, Deps>,
    T: Dependency<Deps> + DependencyClone + 'cont,
    Deps: 'cont,
{
    fn resolve(&'this self) -> T {
        SingletonContainer::resolve_container(self.get(), || self.get_deps()).clone()
    }
}
impl<'this, 'cont, T, SP, Index, Deps, Infer>
    Resolver<'this, &'cont SingletonContainer<T>, &'cont T, (Index, Deps, Infer)> for SP
where
    SP: GetDependencies<'this, Deps, Infer>
        + SelectContainer<'this, &'cont SingletonContainer<T>, Index>,
    SingletonContainer<T>: ResolveContainer<'cont, &'cont T, Deps>,
    T: Dependency<Deps> + 'cont,
    Deps: 'cont,
{
    fn resolve(&'this self) -> &'cont T {
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
impl<'a, T> ResolveContainer<'a, &'a T, HNil> for InstanceContainer<T> {
    fn resolve_container<F: Fn() -> HNil>(ct: &'a InstanceContainer<T>, _: F) -> &'a T {
        &ct.0
    }
}
impl<'this, 'cont, T, SP, Index> Resolver<'this, &'cont InstanceContainer<T>, T, Index> for SP
where
    SP: SelectContainer<'this, &'cont InstanceContainer<T>, Index>,
    InstanceContainer<T>: ResolveContainer<'cont, &'cont T, HNil>,
    T: DependencyClone + 'cont,
{
    fn resolve(&'this self) -> T {
        InstanceContainer::resolve_container(self.get(), || HNil).clone()
    }
}
impl<'this, 'cont, T, SP, Index> Resolver<'this, &'cont InstanceContainer<T>, &'cont T, Index>
    for SP
where
    SP: SelectContainer<'this, &'cont InstanceContainer<T>, Index>,
    InstanceContainer<T>: ResolveContainer<'cont, &'cont T, HNil>,
{
    fn resolve(&'this self) -> &'cont T {
        InstanceContainer::resolve_container(self.get(), || HNil)
    }
}
impl<T> InstanceContainer<T> {
    #[inline]
    pub fn get(&self) -> &T {
        &self.0
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
impl<'a, Cont, T, U, Deps> ResolveContainer<'a, U, Deps> for ConvertContainer<Cont, T, U>
where
    Cont: ResolveContainer<'a, T, Deps>,
    T: Into<U>,
{
    fn resolve_container<F: Fn() -> Deps>(ct: &'a Self, deps: F) -> U {
        Cont::resolve_container(&ct.0, deps).into()
    }
}
impl<'this, 'cont, Cont, T, U, SP, Index, Deps, Infer>
    Resolver<'this, &'cont ConvertContainer<Cont, T, U>, U, (Index, Deps, Infer)> for SP
where
    SP: SelectContainer<'this, &'cont ConvertContainer<Cont, T, U>, Index>
        + GetDependencies<'this, Deps, Infer>,
    ConvertContainer<Cont, T, U>: ResolveContainer<'cont, U, Deps>,
    Cont: 'cont,
    T: Into<U> + 'cont,
    U: 'cont,
    Deps: 'cont,
{
    fn resolve(&'this self) -> U {
        ConvertContainer::resolve_container(self.get(), || self.get_deps())
    }
}
impl<Cont, ContT, T> ConvertContainer<Cont, ContT, T> {
    #[inline]
    pub fn get(&self) -> &Cont {
        &self.0
    }
}
