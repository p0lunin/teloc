pub use teloc_macros::{container, Teloc};

pub trait Get<T> {
    fn get(&self) -> T;
}
pub trait GetClone<T: Clone> {
    fn get_clone(&self) -> T;
}

pub struct ContainerWrapper<T>(pub T);
