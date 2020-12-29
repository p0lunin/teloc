use crate::container::Init;
use crate::dependency::DependencyClone;
use once_cell::sync::OnceCell;
use std::marker::PhantomData;

pub trait Lifetime<'a, T, Out: 'a> {
    fn get_value_or_init(&'a self, init: impl FnOnce() -> T) -> Out;
}

pub struct Transient<T>(PhantomData<T>);
pub struct Singleton<T>(OnceCell<T>);

impl<T> Init for Transient<T> {
    type Data = ();

    fn init(_: Self::Data) -> Self {
        Transient(PhantomData)
    }
}
impl<T> Init for Singleton<T> {
    type Data = ();

    fn init(_: Self::Data) -> Self {
        Self(OnceCell::new())
    }
}

impl<'a, T: 'a> Lifetime<'a, T, T> for Transient<T> {
    fn get_value_or_init(&'a self, init: impl FnOnce() -> T) -> T {
        (init)()
    }
}
impl<'a, T> Lifetime<'a, T, &'a T> for Singleton<T> {
    fn get_value_or_init(&'a self, init: impl FnOnce() -> T) -> &'a T {
        self.0.get_or_init(init)
    }
}
impl<'a, T: 'a> Lifetime<'a, T, T> for Singleton<T>
where
    T: DependencyClone,
{
    fn get_value_or_init(&'a self, init: impl FnOnce() -> T) -> T {
        self.0.get_or_init(init).clone()
    }
}
