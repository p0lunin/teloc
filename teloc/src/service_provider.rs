use crate::container::{
    Container, ConvertContainer, InstanceContainer, SingletonContainer, TransientContainer,
};
use crate::index::{ParentIndex, SelfIndex};
use frunk::hlist::{HList, Selector};
use frunk::{HCons, HNil};
use std::rc::Rc;
use std::sync::Arc;

/// `ServiceProvider` struct is used as an IoC-container in which you declare your dependencies.
///
/// Algorithm for working in `ServiceProvider` is:
/// 1. Create an empty by `ServiceProvider::new` function.
/// 2. Declare your dependencies using `add_*` methods (more about theirs read below).
/// 3. Fork `ServiceProvider` when you need working with scoped sessions (like when you processing web request).
/// 4. Get needed dependencies from container using `Resolver::resolve` trait.
///
/// If you do not register all of needed dependencies, then compiler do not compile your code. If error
/// puts you into a stupor, read our [manual] about how read errors.
///
/// [manual]: https://github.com/p0lunin/teloc/blob/master/HOW-TO-READ-ERRORS.md
///
/// Example of usage `ServiceProvider`:
/// ```
/// use std::rc::Rc;
/// use teloc::*;
///
/// struct ConstService {
///     number: Rc<i32>,
/// }
///
/// #[inject]
/// impl ConstService {
///     pub fn new(number: Rc<i32>) -> Self {
///         ConstService { number }
///     }
/// }
///
/// #[derive(Dependency)]
/// struct Controller {
///     number_service: ConstService,
/// }
///
/// let container = ServiceProvider::new()
///     .add_transient::<ConstService>()
///     .add_transient::<Controller>();
/// let scope = container.fork().add_instance(Rc::new(10));
/// let controller: Controller = scope.resolve();
/// assert_eq!(*controller.number_service.number, 10);
/// ```
#[derive(Debug)]
pub struct ServiceProvider<Parent, Conts> {
    pub(crate) parent: Parent,
    pub(crate) containers: Conts,
}

#[derive(Debug)]
pub struct EmptyServiceProvider;

impl ServiceProvider<EmptyServiceProvider, HNil> {
    /// Create an empty instance of `ServiceProvider`
    pub fn new() -> Self {
        ServiceProvider {
            parent: EmptyServiceProvider,
            containers: HNil,
        }
    }
}

impl Default for ServiceProvider<EmptyServiceProvider, HNil> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Parent, Conts> ServiceProvider<Parent, Conts> {
    pub(crate) fn dependencies(&self) -> &Conts {
        &self.containers
    }

    /// Forking `ServiceProvider` creates a new `ServiceProvider` with reference to the parent.
    /// `resolve` method on forked `ServiceProvider` will find dependencies form self and parent.
    pub fn fork(&self) -> ServiceProvider<&Self, HNil> {
        ServiceProvider {
            parent: self,
            containers: HNil,
        }
    }

    /// Forking `ServiceProvider` creates a new `ServiceProvider` with reference to the parent.
    /// `resolve` method on forked `ServiceProvider` will find dependencies form self and parent.
    pub fn fork_rc(self: &Rc<ServiceProvider<Parent, Conts>>) -> ServiceProvider<Rc<Self>, HNil> {
        ServiceProvider {
            parent: self.clone(),
            containers: HNil,
        }
    }

    /// Forking `ServiceProvider` creates a new `ServiceProvider` with reference to the parent.
    /// `resolve` method on forked `ServiceProvider` will find dependencies form self and parent.
    pub fn fork_arc(
        self: &Arc<ServiceProvider<Parent, Conts>>,
    ) -> ServiceProvider<Arc<Self>, HNil> {
        ServiceProvider {
            parent: self.clone(),
            containers: HNil,
        }
    }
}

// Clippy requires to create type aliases
type ContainerTransientAddConvert<Parent, T, U, Conts> =
    ServiceProvider<Parent, HCons<ConvertContainer<TransientContainer<T>, T, U>, Conts>>;
type ContainerSingletonAddConvert<Parent, T, U, Conts> =
    ServiceProvider<Parent, HCons<ConvertContainer<SingletonContainer<T>, T, U>, Conts>>;
type ContainerInstanceAddConvert<Parent, T, U, Conts> =
    ServiceProvider<Parent, HCons<ConvertContainer<InstanceContainer<T>, T, U>, Conts>>;

impl<Parent, Conts: HList> ServiceProvider<Parent, Conts> {
    /// Method used primary for internal actions. In common usage you don't need to use it. It add dependencies to the store. You need
    /// to put in first generic parameter some `ContainerElem` type.
    /// Usage:
    ///
    /// ```
    /// use teloc::*;
    ///
    /// struct Service {
    ///     data: i32,
    /// }
    ///
    /// let sp = ServiceProvider::new()
    ///     ._add::<TransientContainer<Service>>(());
    /// ```
    pub fn _add<Cont: Container>(
        self,
        data: Cont::Data,
    ) -> ServiceProvider<Parent, HCons<Cont, Conts>> {
        let ServiceProvider { parent, containers } = self;
        ServiceProvider {
            parent,
            containers: containers.prepend(Container::init(data)),
        }
    }

    /// Add dependency with the `Transient` lifetime. Transient services will be created each time
    /// when it called. Use this lifetime for lightweight stateless services.
    ///
    /// Can be resolved only by ownership.
    ///
    /// Usage:
    /// ```
    /// use teloc::*;
    /// use uuid::Uuid;
    ///
    /// struct Service { uuid: Uuid }
    /// #[inject]
    /// impl Service {
    ///     fn new() -> Self { Self { uuid: Uuid::new_v4() } }
    /// }
    ///
    /// let sp = ServiceProvider::new()
    ///     .add_transient::<Service>();
    ///
    /// let s1: Service = sp.resolve();
    /// let s2: Service = sp.resolve();
    ///
    /// assert_ne!(s1.uuid, s2.uuid);
    /// ```
    pub fn add_transient<T>(self) -> ServiceProvider<Parent, HCons<TransientContainer<T>, Conts>>
    where
        TransientContainer<T>: Container<Data = ()>,
    {
        self._add::<TransientContainer<T>>(())
    }

    /// Add dependency with the `Singleton` lifetime. Singleton services will be created only one
    /// time when it will be called first time. It will be same between different calls in parent
    /// and forked `ServiceProvider`
    ///
    /// Can be resolved by reference or by cloning. If you wish to clone this dependency then it
    /// must implement `DependencyClone` trait. For more information see `DependencyClone` trait.
    ///
    /// Usage:
    /// ```
    /// use teloc::*;
    /// use uuid::Uuid;
    ///
    /// struct Service { uuid: Uuid }
    /// #[inject]
    /// impl Service {
    ///     fn new() -> Self { Self { uuid: Uuid::new_v4() } }
    /// }
    ///
    /// let sp = ServiceProvider::new()
    ///     .add_singleton::<Service>();
    /// let scope = sp.fork();
    ///
    /// let s1: &Service = sp.resolve();
    /// let s2: &Service = scope.resolve();
    ///
    /// assert_eq!(s1.uuid, s2.uuid);
    /// ```
    ///
    /// Usage with cloning:
    ///
    /// ```
    /// use teloc::*;
    /// use uuid::Uuid;
    /// use std::rc::Rc;
    ///
    /// struct Service { uuid: Uuid }
    /// #[inject]
    /// impl Service {
    ///     fn new() -> Self { Self { uuid: Uuid::new_v4() } }
    /// }
    ///
    /// let sp = ServiceProvider::new()
    ///     .add_singleton::<Rc<Service>>();
    ///
    /// let s1: Rc<Service> = sp.resolve();
    /// let s2: Rc<Service> = sp.resolve();
    ///
    /// assert_eq!(s1.uuid, s2.uuid)
    /// ```
    pub fn add_singleton<T>(self) -> ServiceProvider<Parent, HCons<SingletonContainer<T>, Conts>>
    where
        SingletonContainer<T>: Container<Data = ()>,
    {
        self._add::<SingletonContainer<T>>(())
    }

    /// Add anything instance to provider. It likes singleton, but it cannot get dependencies from
    /// the provider. Use it for adding single objects like configs.
    ///
    /// Can be resolved by reference or by cloning. If you wish to clone this dependency then it
    /// must implement `DependencyClone` trait. For more information see `DependencyClone` trait.
    ///
    /// Usage:
    /// ```
    /// use teloc::*;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Config { token: String, ip: String }
    ///
    /// struct Service<'a> { token: &'a str, ip: &'a str }
    /// #[inject]
    /// impl<'a> Service<'a> {
    ///     fn new(config: &'a Config) -> Self { Self { token: &config.token, ip: &config.ip } }
    /// }
    ///
    /// let config = Config { token: "1234ABCDE".into(), ip: "192.168.0.1".into() };
    ///
    /// let sp = ServiceProvider::new()
    ///     .add_instance(&config)
    ///     .add_transient::<Service>();
    ///
    /// let config_ref: &Config = sp.resolve();
    /// let s: Service = sp.resolve();
    ///
    /// assert_eq!(&config, config_ref);
    /// assert_eq!(&config_ref.token, s.token);
    /// assert_eq!(&config_ref.ip, s.ip);
    /// ```
    pub fn add_instance<T>(
        self,
        data: T,
    ) -> ServiceProvider<Parent, HCons<InstanceContainer<T>, Conts>>
    where
        InstanceContainer<T>: Container<Data = T>,
    {
        self._add::<InstanceContainer<T>>(data)
    }

    /// Same as `ServiceProvider::add_transient`, but can be used for convert one type to another
    /// when resolving. Can be used for creating `Box<dyn Trait>` instances, for example.
    ///
    /// Suffix `_c` means 'convert'.
    ///
    /// Usage:
    /// ```
    /// use teloc::*;
    ///
    /// trait NumberService {
    ///     fn get_num(&self) -> i32;
    /// }
    ///
    /// struct TenService {
    ///     number: i32,
    /// }
    /// impl NumberService for TenService {
    ///     fn get_num(&self) -> i32 {
    ///         self.number
    ///     }
    /// }
    /// #[inject]
    /// impl TenService {
    ///     fn new() -> Self {
    ///         Self { number: 10 }
    ///     }
    /// }
    ///impl From<Box<TenService>> for Box<dyn NumberService> {
    ///     fn from(x: Box<TenService>) -> Self {
    ///         x
    ///     }
    /// }
    ///
    /// #[derive(Dependency)]
    /// struct Controller {
    ///     number_service: Box<dyn NumberService>,
    /// }
    ///
    /// let container = ServiceProvider::new()
    ///     .add_transient_c::<Box<dyn NumberService>, Box<TenService>>()
    ///     .add_transient::<Controller>();
    /// let controller: Controller = container.resolve();
    ///
    /// assert_eq!(controller.number_service.get_num(), 10);
    /// ```
    pub fn add_transient_c<U, T>(self) -> ContainerTransientAddConvert<Parent, T, U, Conts>
    where
        T: Into<U>,
        ConvertContainer<TransientContainer<T>, T, U>: Container<Data = ()>,
        TransientContainer<T>: Container<Data = ()>,
    {
        self._add::<ConvertContainer<TransientContainer<T>, T, U>>(())
    }

    /// Same as `Provider::add_transient_c` but for `Singleton` lifetime.
    pub fn add_singleton_c<U, T>(self) -> ContainerSingletonAddConvert<Parent, T, U, Conts>
    where
        T: Into<U>,
        ConvertContainer<SingletonContainer<T>, T, U>: Container<Data = ()>,
        SingletonContainer<T>: Container<Data = ()>,
    {
        self._add::<ConvertContainer<SingletonContainer<T>, T, U>>(())
    }

    /// Same as `Provider::add_transient_c` but for `Instance` lifetime.
    pub fn add_instance_c<U, T>(
        self,
        instance: T,
    ) -> ContainerInstanceAddConvert<Parent, T, U, Conts>
    where
        T: Into<U>,
        ConvertContainer<InstanceContainer<T>, T, U>: Container<Data = T>,
        InstanceContainer<T>: Container<Data = T>,
    {
        self._add::<ConvertContainer<InstanceContainer<T>, T, U>>(instance)
    }
}

/// Borrow containers from a ServiceProvider.
///
/// ```
/// # use teloc::*;
/// let cont: &InstanceContainer<i32> =
///     ServiceProvider::new().add_instance(10i32).get();
/// ```
///
/// Can borrow containers from a forked ServiceProvider that would outlive the fork
///
/// ```
/// # use teloc::*;
/// let provider = ServiceProvider::new().add_instance(10i32);
/// let invalid_singleton: &InstanceContainer<i32> = {
///     let provider = provider.fork();
///     provider.get()
/// };
///```
///
/// Cannot borrow containers from a ServiceProvider that would outlive the ServiceProvider
///
/// ```compile_fail
/// # use teloc::*;
/// # use teloc::dev::*;
/// let invalid_singleton: &InstanceContainer<i32> = {
///     let provider = ServiceProvider::new().add_instance(10i32);
///     provider.get()
/// };
///```
pub trait SelectContainer<'a, Cont, Index> {
    fn get(&'a self) -> Cont;
}

impl<'this, Parent, Conts, Cont, Index> SelectContainer<'this, &'this Cont, SelfIndex<Index>>
    for ServiceProvider<Parent, Conts>
where
    Conts: Selector<Cont, Index>,
{
    fn get(&'this self) -> &'this Cont {
        self.dependencies().get()
    }
}

impl<'this, 'parent, 'cont, Parent, Conts, Cont, Index>
    SelectContainer<'this, &'cont Cont, ParentIndex<Index>>
    for ServiceProvider<&'parent Parent, Conts>
where
    Parent: SelectContainer<'parent, &'cont Cont, Index>,
{
    fn get(&'this self) -> &'cont Cont {
        self.parent.get()
    }
}

impl<'this, 'cont, Parent, Conts, Cont, Index>
    SelectContainer<'this, &'cont Cont, ParentIndex<Index>> for ServiceProvider<Rc<Parent>, Conts>
where
    Parent: SelectContainer<'this, &'cont Cont, Index>,
{
    fn get(&'this self) -> &'cont Cont {
        self.parent.get()
    }
}

impl<'this, 'cont, Parent, Conts, Cont, Index>
    SelectContainer<'this, &'cont Cont, ParentIndex<Index>> for ServiceProvider<Arc<Parent>, Conts>
where
    Parent: SelectContainer<'this, &'cont Cont, Index>,
{
    fn get(&'this self) -> &'cont Cont {
        self.parent.get()
    }
}
