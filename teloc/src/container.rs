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
pub trait ResolveContainer<'a, T, Deps> {
    fn resolve_container<F: Fn() -> Deps>(&'a self, deps: F) -> T;
}

impl<'a, Cont, T, SP, Index, Deps, Infer> Resolver<'a, Cont, T, (Index, Deps, Infer)> for SP
where
    SP: GetDependencies<'a, Deps, Infer> + Selector<Cont, Index>,
    Cont: ResolveContainer<'a, T, Deps> + 'a,
    T: 'a,
    Deps: 'a,
{
    fn resolve(&'a self) -> T {
        Cont::resolve_container(self.get(), || self.get_deps())
    }
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
    fn resolve_container<F: Fn() -> Deps>(&'a self, get_deps: F) -> T {
        T::init(get_deps())
    }
}

#[derive(Debug)]
pub struct SingletonContainer<T>(OnceCell<T>);
impl<T> SingletonContainer<T> {
    #[inline]
    pub fn get(&self) -> &OnceCell<T> {
        &self.0
    }
}

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
    fn resolve_container<F: Fn() -> Deps>(&'a self, get_deps: F) -> &'a T {
        self.get().get_or_init(|| T::init(get_deps()))
    }
}

impl<'a, T, Deps> ResolveContainer<'a, T, Deps> for SingletonContainer<T>
where
    T: Dependency<Deps> + DependencyClone + 'a,
{
    fn resolve_container<F: Fn() -> Deps>(&'a self, get_deps: F) -> T {
        self.get().get_or_init(|| T::init(get_deps())).clone()
    }
}

#[derive(Debug)]
pub struct InstanceContainer<T>(T);
impl<T> InstanceContainer<T> {
    #[inline]
    pub fn get(&self) -> &T {
        &self.0
    }
}

impl<T> Container for InstanceContainer<T> {
    type Data = T;

    fn init(instance: T) -> Self {
        Self(instance)
    }
}
impl<'a, T> ResolveContainer<'a, &'a T, HNil> for InstanceContainer<T> {
    fn resolve_container<F: Fn() -> HNil>(&'a self, _: F) -> &'a T {
        &self.0
    }
}

impl<'a, T> ResolveContainer<'a, T, HNil> for InstanceContainer<T>
where
    T: DependencyClone,
{
    fn resolve_container<F: Fn() -> HNil>(&'a self, _: F) -> T {
        self.0.clone()
    }
}

pub struct ConvertContainer<Cont, T, U>(Cont, PhantomData<(T, U)>);
impl<Cont, ContT, T> ConvertContainer<Cont, ContT, T> {
    #[inline]
    pub fn get(&self) -> &Cont {
        &self.0
    }
}

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
    fn resolve_container<F: Fn() -> Deps>(&'a self, deps: F) -> U {
        Cont::resolve_container(&self.0, deps).into()
    }
}
