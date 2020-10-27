use crate::container_elem::{
    ContainerElem, ConvertContainerElem, InstanceContainerElem, SingletonContainerElem,
    TransientContainerElem,
};
use crate::Scope;
use frunk::hlist::HList;
use frunk::{HCons, HNil};
use std::marker::PhantomData;

pub struct ServiceProvider<Dependencies, Scoped> {
    dependencies: Dependencies,
    scoped: PhantomData<Scoped>,
}

impl ServiceProvider<HNil, HNil> {
    pub fn new() -> Self {
        ServiceProvider {
            dependencies: HNil,
            scoped: PhantomData,
        }
    }
}

impl Default for ServiceProvider<HNil, HNil> {
    fn default() -> Self {
        Self::new()
    }
}

type ContainerAddConvertElem<T, U, H, S> =
    ServiceProvider<HCons<ConvertContainerElem<TransientContainerElem<T>, T, U>, H>, S>;

impl<H: HList, S> ServiceProvider<H, S> {
    pub fn _add<TE, T: ContainerElem<TE>>(self, data: T::Data) -> ServiceProvider<HCons<T, H>, S> {
        let ServiceProvider { dependencies, .. } = self;
        ServiceProvider {
            dependencies: dependencies.prepend(T::init(data)),
            scoped: PhantomData,
        }
    }
    pub fn add_transient<T>(self) -> ServiceProvider<HCons<TransientContainerElem<T>, H>, S> {
        self._add::<T, TransientContainerElem<T>>(())
    }
    pub fn add_scoped<T>(self) -> ServiceProvider<H, HCons<T, S>> {
        let ServiceProvider { dependencies, .. } = self;
        ServiceProvider {
            dependencies,
            scoped: PhantomData,
        }
    }
    pub fn add_singleton<T>(self) -> ServiceProvider<HCons<SingletonContainerElem<T>, H>, S> {
        self._add::<T, SingletonContainerElem<T>>(())
    }
    pub fn add_instance<T>(
        self,
        data: T,
    ) -> ServiceProvider<HCons<InstanceContainerElem<T>, H>, S> {
        self._add::<T, InstanceContainerElem<T>>(data)
    }
    pub fn add_transient_<U, T>(self) -> ContainerAddConvertElem<T, U, H, S>
    where
        T: Into<U>,
    {
        self._add::<U, ConvertContainerElem<TransientContainerElem<T>, T, U>>(())
    }
}
impl<H, S> ServiceProvider<H, S> {
    pub fn scope(&self, data: S) -> Scope<H, S> {
        Scope::new(self, data)
    }
}

impl<H, S> ServiceProvider<H, S> {
    pub fn dependencies(&self) -> &H {
        &self.dependencies
    }
}
