pub use teloc_macros::{container, Teloc};

pub trait Getable<T> { }

impl Getable<()> for () { }

pub trait Get<T: Getable<U>, U> {
    fn get(&mut self) -> U;
}
pub trait GetRef<T: Getable<U>, U> {
    fn get_ref(&self) -> &U;
}
pub trait GetClone<T: Getable<U>, U> where U: Clone {
    fn get_clone(&self) -> U;
}

impl<C, T, U> GetClone<T, U> for ContainerWrapper<C>
where
    T: Getable<U>,
    U: Clone,
    ContainerWrapper<C>: GetRef<T, U>,
{
    fn get_clone(&self) -> U {
        self.get_ref().clone()
    }
}

pub struct ContainerWrapper<T>(pub T);
