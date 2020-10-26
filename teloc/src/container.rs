use crate::container_elem::{
    ContainerElem, ConvertContainerElem, InstanceContainerElem, SingletonContainerElem,
    TransientContainerElem,
};
use frunk::hlist::HList;
use frunk::{HCons, HNil};

pub struct Container<Dependencies = HNil> {
    dependencies: Dependencies,
}

impl Container {
    pub fn new() -> Self {
        Container { dependencies: HNil }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

type ContainerAddConvertElem<T, U, H> =
    Container<HCons<ConvertContainerElem<TransientContainerElem<T>, T, U>, H>>;

impl<H: HList> Container<H> {
    pub fn _add<TE, T: ContainerElem<TE>>(self, data: T::Data) -> Container<HCons<T, H>> {
        let Container { dependencies } = self;
        Container {
            dependencies: dependencies.prepend(T::init(data)),
        }
    }
    pub fn add_transient<T>(self) -> Container<HCons<TransientContainerElem<T>, H>> {
        self._add::<T, TransientContainerElem<T>>(())
    }
    pub fn add_singleton<T>(self) -> Container<HCons<SingletonContainerElem<T>, H>> {
        self._add::<T, SingletonContainerElem<T>>(())
    }
    pub fn add_instance<T>(self, data: T) -> Container<HCons<InstanceContainerElem<T>, H>> {
        self._add::<T, InstanceContainerElem<T>>(data)
    }
    pub fn add_transient_<U, T>(self) -> ContainerAddConvertElem<T, U, H>
    where
        T: Into<U>,
    {
        self._add::<U, ConvertContainerElem<TransientContainerElem<T>, T, U>>(())
    }
}
impl<H> Container<H> {
    pub fn dependencies(&self) -> &H {
        &self.dependencies
    }
}
