use crate::container::{
    ConvertContainer, Init, InstanceContainer, SingletonContainer, TransientContainer,
};
use crate::scope::{InitScope, InitScoped, ScopedContainerElem, ScopedInstanceContainer};
use crate::Scope;
use frunk::hlist::{HList, Selector};
use frunk::{HCons, HNil};
use std::marker::PhantomData;

pub struct ServiceProvider<Dependencies, Scoped, ScopedI> {
    dependencies: Dependencies,
    scoped_i: PhantomData<ScopedI>,
    scoped: PhantomData<Scoped>,
}

impl ServiceProvider<HNil, HNil, HNil> {
    pub fn new() -> Self {
        ServiceProvider {
            dependencies: HNil,
            scoped_i: PhantomData,
            scoped: PhantomData,
        }
    }
}

impl Default for ServiceProvider<HNil, HNil, HNil> {
    fn default() -> Self {
        Self::new()
    }
}

type ContainerTransientAddConvert<T, U, H, S, SI> =
    ServiceProvider<HCons<ConvertContainer<TransientContainer<T>, T, U>, H>, S, SI>;
type ContainerSingletonAddConvert<T, U, H, S, SI> =
    ServiceProvider<HCons<ConvertContainer<SingletonContainer<T>, T, U>, H>, S, SI>;

impl<H: HList, S, SI> ServiceProvider<H, S, SI> {
    /// Method used primary for internal actions. In common usage you don't need to use it. It add dependencies to the store. You need
    /// to put in first generic parameter some `ContainerElem` type.
    /// Usage:
    ///
    /// ```
    /// use teloc::*;
    /// use teloc::container::TransientContainer;
    ///
    /// struct Service {
    ///     data: i32,
    /// }
    ///
    /// let sp = ServiceProvider::new()
    ///     ._add::<TransientContainer<Service>>(());
    /// ```
    pub fn _add<T: Init>(self, data: T::Data) -> ServiceProvider<HCons<T, H>, S, SI> {
        let ServiceProvider { dependencies, .. } = self;
        ServiceProvider {
            dependencies: dependencies.prepend(T::init(data)),
            scoped_i: PhantomData,
            scoped: PhantomData,
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
    pub fn add_transient<T>(self) -> ServiceProvider<HCons<TransientContainer<T>, H>, S, SI>
    where
        TransientContainer<T>: Init<Data = ()>,
    {
        self._add::<TransientContainer<T>>(())
    }

    /// Add dependency with the `Scoped` lifetime. Scoped services will be created only one time for
    /// one scope, which can be created using `ServiceProvider::scope` method. Scoped dependencies
    /// is not available in `ServiceProvider`, only in `Scope`.
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
    ///     .add_scoped::<Service>();
    ///
    /// // .scope_() is a wrapper for .scope(HNil)
    /// let scope1 = sp.scope_();
    ///
    /// let s1: &Service = scope1.resolve();
    /// let s2: &Service = scope1.resolve();
    ///
    /// let scope2 = sp.scope_();
    /// let s3: &Service = scope2.resolve();
    ///
    /// assert_eq!(s1.uuid, s2.uuid);
    /// assert_ne!(s1.uuid, s3.uuid);
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
    ///     .add_scoped::<Rc<Service>>();
    ///
    /// let scope = sp.scope_();
    ///
    /// let s1: Rc<Service> = scope.resolve();
    /// let s2: Rc<Service> = scope.resolve();
    ///
    /// assert_eq!(s1.uuid, s2.uuid)
    /// ```
    #[inline]
    pub fn add_scoped<T>(self) -> ServiceProvider<H, HCons<ScopedContainerElem<T>, S>, SI> {
        let ServiceProvider { dependencies, .. } = self;
        ServiceProvider {
            dependencies,
            scoped_i: PhantomData,
            scoped: PhantomData,
        }
    }

    /// Add information about instance that should be added to `Scope` before it's initialization.
    /// It can be `Request`, `DbConnection`, etc. It must be passed to `ServiceProvider::scope`
    /// method in future. Scoped dependencies is not available in `ServiceProvider`, only in `Scope`.
    ///
    /// Usage:
    /// ```
    /// use teloc::*;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct City(String);
    ///
    /// // Note that we does not implement `DependencyClone` for City so only way to give `City`
    /// // value is by reference
    /// struct WeatherService<'a> { city: &'a City }
    /// #[inject]
    /// impl<'a> WeatherService<'a> {
    ///     fn new(city: &'a City) -> Self { Self { city } }
    /// }
    ///
    /// let sp = ServiceProvider::new()
    ///     .add_scoped_i::<City>()
    ///     .add_scoped::<WeatherService>();
    ///
    /// let scope = sp.scope(scopei![City("Odessa".into()),]);
    ///
    /// let s1: &WeatherService = scope.resolve();
    /// let s2: &WeatherService = scope.resolve();
    ///
    /// assert_eq!(s1.city.0, "Odessa".to_string());
    /// assert_eq!(s1.city, s2.city);
    /// ```
    #[inline]
    pub fn add_scoped_i<T>(self) -> ServiceProvider<H, S, HCons<ScopedInstanceContainer<T>, SI>> {
        let ServiceProvider { dependencies, .. } = self;
        ServiceProvider {
            dependencies,
            scoped_i: PhantomData,
            scoped: PhantomData,
        }
    }

    /// Add dependency with the `Singleton` lifetime. Singleton services will be created only one
    /// time when it will be called first time. It will be same between different calls in
    /// `Scope::resolve` and `ServiceProvider::resolve`.
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
    /// let scope = sp.scope_();
    ///
    /// let s1: &Service = sp.resolve();
    /// let s2: &Service = scope.resolve();
    ///
    /// assert_eq!(s1.uuid, s2.uuid);
    /// ```
    ///
    /// Usage by cloning is the same as in `ServiceProvider::add_scoped` method.
    pub fn add_singleton<T>(self) -> ServiceProvider<HCons<SingletonContainer<T>, H>, S, SI>
    where
        SingletonContainer<T>: Init<Data = ()>,
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
    pub fn add_instance<T>(self, data: T) -> ServiceProvider<HCons<InstanceContainer<T>, H>, S, SI>
    where
        InstanceContainer<T>: Init<Data = T>,
    {
        self._add::<InstanceContainer<T>>(data)
    }

    /// Same as `ServiceProvider::add_transient`, but can be used for convert one type to another
    /// when resolving. Can be used for creating `Box<dyn Trait>` instances, for example.
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
    /// #[derive(Teloc)]
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
    pub fn add_transient_c<U, T>(self) -> ContainerTransientAddConvert<T, U, H, S, SI>
    where
        T: Into<U>,
        ConvertContainer<TransientContainer<T>, T, U>: Init<Data = ()>,
        TransientContainer<T>: Init<Data = ()>,
    {
        self._add::<ConvertContainer<TransientContainer<T>, T, U>>(())
    }

    /// Same as `Provider::add_transient_c` but for `Singleton` lifetime.
    pub fn add_singleton_c<U, T>(self) -> ContainerSingletonAddConvert<T, U, H, S, SI>
    where
        T: Into<U>,
        ConvertContainer<SingletonContainer<T>, T, U>: Init<Data = ()>,
        TransientContainer<T>: Init<Data = ()>,
    {
        self._add::<ConvertContainer<SingletonContainer<T>, T, U>>(())
    }
}

impl<'a, H, S, SI> ServiceProvider<H, S, SI>
where
    S: InitScoped,
{
    /// Create `Scope` for working with dependencies with `Scoped` lifetime. You must pass to the
    /// scope instances that you was added by `ServiceProvider::add_scoped_i` before by `scopei![]`
    /// macro.
    pub fn scope(&self, si: SI) -> Scope<Self, S, SI> {
        Scope::new(self, si)
    }
}

impl<'a, H, S> ServiceProvider<H, S, HNil>
where
    S: InitScoped,
{
    /// Wrapper for `ServiceProvider::scope(self, HNil)`, when you have not scoped instances.
    pub fn scope_(&self) -> Scope<Self, S, HNil> {
        self.scope(HNil)
    }
}

impl<H, S, SI> ServiceProvider<H, S, SI> {
    pub(crate) fn dependencies(&self) -> &H {
        &self.dependencies
    }
}

impl<H, S, SI, T, Index> Selector<T, Index> for ServiceProvider<H, S, SI>
where
    H: Selector<T, Index>,
{
    fn get(&self) -> &T {
        self.dependencies().get()
    }

    fn get_mut(&mut self) -> &mut T {
        self.dependencies.get_mut()
    }
}
