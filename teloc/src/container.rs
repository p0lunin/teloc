use crate::container_elem::{
    ContainerElem, InstanceContainerElem, SingletonContainerElem, TransientContainerElem,
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

impl<H: HList> Container<H> {
    pub fn add<TE, T: ContainerElem<TE>>(self, data: T::Data) -> Container<HCons<T, H>> {
        let Container { dependencies } = self;
        Container {
            dependencies: dependencies.prepend(T::init(data)),
        }
    }
    pub fn add_transient<T>(self) -> Container<HCons<TransientContainerElem<T>, H>> {
        self.add::<T, TransientContainerElem<T>>(())
    }
    pub fn add_singleton<T>(self) -> Container<HCons<SingletonContainerElem<T>, H>> {
        self.add::<T, SingletonContainerElem<T>>(())
    }
    pub fn add_instance<T>(self, data: T) -> Container<HCons<InstanceContainerElem<T>, H>> {
        self.add::<T, InstanceContainerElem<T>>(data)
    }
}
impl<H> Container<H> {
    pub fn dependencies(&self) -> &H {
        &self.dependencies
    }
}
