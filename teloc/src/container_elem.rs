use once_cell::sync::OnceCell;
use std::marker::PhantomData;

pub trait ContainerElem<Elem> {
    type Data;
    fn init(data: Self::Data) -> Self;
}

pub struct TransientContainerElem<T>(PhantomData<T>);
impl<T> ContainerElem<T> for TransientContainerElem<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}

pub struct SingletonContainerElem<T>(OnceCell<T>);
impl<T> ContainerElem<T> for SingletonContainerElem<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(OnceCell::new())
    }
}
impl<T> SingletonContainerElem<T> {
    #[inline]
    pub fn get(&self) -> &OnceCell<T> {
        &self.0
    }
}

pub struct InstanceContainerElem<T>(T);
impl<T> ContainerElem<T> for InstanceContainerElem<T> {
    type Data = T;

    fn init(instance: T) -> Self {
        Self(instance)
    }
}
impl<T> InstanceContainerElem<T> {
    #[inline]
    pub fn get(&self) -> &T {
        &self.0
    }
}

pub struct ByRefSingletonContainerElem<T>(PhantomData<T>);
impl<T> ContainerElem<&T> for ByRefSingletonContainerElem<T> {
    type Data = ();

    fn init(_: Self::Data) -> Self {
        Self(PhantomData)
    }
}

pub struct ByRefInstanceContainerElem<T>(PhantomData<T>);
impl<T> ContainerElem<&T> for ByRefInstanceContainerElem<T> {
    type Data = ();

    fn init(_: Self::Data) -> Self {
        Self(PhantomData)
    }
}
