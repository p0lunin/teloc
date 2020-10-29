use crate::container_elem::{ContainerElem, ConvertContainerElem, InstanceContainerElem, SingletonContainerElem, TransientContainerElem, ScopedContainerElem};
use frunk::hlist::{HList, Selector};
use frunk::{HCons, HNil};

pub struct ServiceProvider<Dependencies> {
    dependencies: Dependencies,
}

impl ServiceProvider<HNil> {
    pub fn new() -> Self {
        ServiceProvider {
            dependencies: HNil,
        }
    }
}

impl Default for ServiceProvider<HNil> {
    fn default() -> Self {
        Self::new()
    }
}

type ContainerAddConvertElem<T, U, H> =
    ServiceProvider<HCons<ConvertContainerElem<TransientContainerElem<T>, T, U>, H>>;

impl<H: HList> ServiceProvider<H> {
    pub fn _add<TE, T: ContainerElem<TE>>(self, data: T::Data) -> ServiceProvider<HCons<T, H>> {
        let ServiceProvider { dependencies, .. } = self;
        ServiceProvider {
            dependencies: dependencies.prepend(T::init(data)),
        }
    }
    pub fn add_transient<T>(self) -> ServiceProvider<HCons<TransientContainerElem<T>, H>> {
        self._add::<T, TransientContainerElem<T>>(())
    }
    pub fn add_scoped<T>(self) -> ServiceProvider<HCons<ScopedContainerElem<T>, H>> {
        self._add::<T, ScopedContainerElem<T>>(())
    }
    pub fn add_singleton<T>(self) -> ServiceProvider<HCons<SingletonContainerElem<T>, H>> {
        self._add::<T, SingletonContainerElem<T>>(())
    }
    pub fn add_instance<T>(
        self,
        data: T,
    ) -> ServiceProvider<HCons<InstanceContainerElem<T>, H>> {
        self._add::<T, InstanceContainerElem<T>>(data)
    }
    pub fn add_transient_<U, T>(self) -> ContainerAddConvertElem<T, U, H>
    where
        T: Into<U>,
    {
        self._add::<U, ConvertContainerElem<TransientContainerElem<T>, T, U>>(())
    }
}/*
impl<H> ServiceProvider<H> {
    pub fn scope(&self) -> Scop
}*/

impl<H> ServiceProvider<H> {
    pub fn dependencies(&self) -> &H {
        &self.dependencies
    }
}

impl<H, T, Index> Selector<T, Index> for ServiceProvider<H>
where
    H: Selector<T, Index>
{
    fn get(&self) -> &T {
        self.dependencies().get()
    }

    fn get_mut(&mut self) -> &mut T {
        self.dependencies.get_mut()
    }
}
