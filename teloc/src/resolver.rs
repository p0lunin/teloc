/// This trait is used to resolve some object from service provider. Generic `T` used only to avoid
/// absence of specialization and for working of type inference. You must implement it yourself
/// only when you implement your own version of container.
///
/// For common usage you need only import it from teloc, and calling `resolve` method when you need
/// to get a service from `ServiceProvider`.
///
/// Example:
///
/// ```
/// use teloc::*;
///
/// struct Foo(u8);
///
/// #[inject]
/// fn new_foo() -> Foo {
///     Foo(5)
/// }
///
/// let sp = ServiceProvider::new()
///     .add_transient::<Foo>();
///
/// let foo: Foo = sp.resolve();
///
/// assert_eq!(foo.0, 5)
/// ```
// Container is a local-crate type to avoid orphan rules. It must be _concrete_, __unique__ type when impl.
pub trait Resolver<'a, Cont, T, Infer> {
    fn resolve(&'a self) -> T;
}
