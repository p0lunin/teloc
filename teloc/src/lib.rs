//! There are 2 types can be provider of services: `ServiceProvider` and `Scope`. First used as store for dependencies with
//! `Instance` and `Singleton` lifetimes, and for declaring all dependencies using `.add_*()` methods. `Scope` can be
//! created from `ServiceProvider` object by calling method `ServiceProvider::scope`.
//!
//! There are four lifetimes for dependencies:
//! 1. `Transient`. Service will be created when resolves. Can depend on dependencies with anything lifetime. If depend on
//! dependency with `Scoped` lifetime can be resolves only from `Scope` object.
//! 2. `Scoped`. Service will be created once at `Scope` when it resolved (lazy). Can depend on dependencies with anything
//! lifetime.
//! 3. `Singleton`. Service will be created once at `ServiceProvider` when it resolved (lazy). Can depend on dependencies
//! with anything lifetime exclude `Scoped`.
//! 4. `Instance`. Dependency was created outside of `ServiceProvider`.
//!
//! Process of working with library:
//! 1. Define your structs.
//! 2. Create constructors and add `#[inject]` macro on its.
//! 3. Create a `ServiceProvider` object.
//! 4. Add your services and dependencies using `ServiceProvider::add_*` methods.
//! 5. Create `Scope` if need.
//! 6. Get service from container using `.resolve()` method.
//! 7. Work with service.
//!
//! Example:
//! ```rust
//! use teloc::*;
//!
//! struct ConstService {
//!     number: i32,
//! }
//! #[inject]
//! impl ConstService {
//!     pub fn new(number: i32) -> Self {
//!         ConstService { number }
//!     }
//! }
//!
//! // derive macro can be used when all fields implement `Dependency` trait, but we do not recommend use it in production
//! // code
//! #[derive(Dependency)]
//! struct Controller {
//!     number_service: ConstService,
//! }
//!
//! let container = ServiceProvider::new()
//!     .add_scoped_i::<i32>()
//!     .add_transient::<ConstService>()
//!     .add_transient::<Controller>();
//! let scope = container.scope(teloc::scopei![10]);
//! let controller: Controller = scope.resolve();
//! assert_eq!(controller.number_service.number, 10);
//! ```

#![forbid(unsafe_code)]

pub mod container;
mod dependency;
mod get_dependencies;
mod resolver;
pub mod scope;
mod service_provider;

pub use {
    dependency::Dependency,
    resolver::Resolver,
    scope::Scope,
    service_provider::ServiceProvider,
    teloc_macros::{inject, Dependency},
};

pub mod reexport {
    //! This module is used to reexport some libraries to `teloc-macros`
    pub use {frunk, frunk::Hlist};
}

/// This macro creates an `HList` with data needed to send to the `Scope` when it init.
/// Usage:
/// ```
/// use teloc::*;
///
/// let sp = ServiceProvider::new()
///     .add_scoped_i::<i32>()
///     .add_scoped_i::<bool>();
/// let scope = sp.scope(scopei![false, 10]);
/// ```
#[macro_export]
macro_rules! scopei {
    [] => { teloc::reexport::frunk::HNil };
    [$x:expr] => {
        teloc::reexport::frunk::hlist::h_cons(
            teloc::container::Init::init($x),
            teloc::reexport::frunk::HNil
        )
    };
    [$x:expr, $($xs:expr),*] => {
        teloc::reexport::frunk::hlist::h_cons(
            teloc::container::Init::init($x),
            teloc::scopei![$($xs,)*]
        )
    }
}
