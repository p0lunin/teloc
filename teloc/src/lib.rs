pub use teloc_macros::{Teloc, container};

pub trait Get<T> {
    fn get(&mut self) -> T;
}
pub trait GetClone<T: Clone> {
    fn get_clone(&self) -> T;
}
pub trait Initable {
    type From;
    fn init<C: Get<Self::From>>(container: &mut C) -> Self;
    fn make_self(data: Self::From) -> Self;
}
