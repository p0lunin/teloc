pub use teloc_macros::{container, Teloc};

/// This trait is used in the main to avoid Rust orphan rules. In default scenarios you don't
/// need to implement it yourself. It will be auto-import in macros for macros. It may be used when
/// you implement by yourself function `init()` and you wish to get some dependencies from
/// `Container`. For additional information see [Get], [GetRef] and [GetClone] traits.
///
/// Trait must be implement for type that contains `Container`, and `T` is a type that can be get
/// by [Get::get] method.
///
/// [Get::get]: teloc::Get::get
/// [Get]: teloc::Get
/// [GetRef]: teloc::GetRef
/// [GetClone]: teloc::GetClone
pub trait Getable<T> {}

impl Getable<()> for () {}

/// Trait that used to provide ability get some dependency by ownership. `get()` method must
/// remove initialized dependency from `Self` and return it. If there are no needed dependency,
/// `get()` method will panic.
///
/// You can use this trait in `init()` function when you need dependencies from container. It may be
/// looks like this:
/// ```
/// use teloc::{Getable, Get};
/// struct EmailOptions {
///     email: String
/// }
///
/// struct EmailingService {
///     email: String
/// }
/// impl EmailingService {
///     pub fn init<A, C>(container: &mut C) -> Self where
///         A: Getable<EmailOptions>,
///         C: Get<A, EmailOptions>
///     {
///         let options: EmailOptions = container.get();
///         Self { email: options.email }
///     }
/// }
/// ```
///
pub trait Get<T: Getable<U>, U> {
    fn get(&mut self) -> U;
}
pub trait GetRef<T: Getable<U>, U> {
    fn get_ref(&self) -> &U;
}
pub trait GetClone<T: Getable<U>, U>
where
    U: Clone,
{
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
