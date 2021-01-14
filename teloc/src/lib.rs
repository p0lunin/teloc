//! There are one type can be provider of services: `ServiceProvider`. It used as store for dependencies with
//! `Instance` and `Singleton` lifetimes, and for declaring all dependencies using `.add_*()` methods. It can be forked to
//! create a local scope with local instances.
//!
//! There are four lifetimes for dependencies:
//! 1. `Transient`. Service will be created when resolves. Can depend on dependencies with anything lifetime.
//! 2. `Singleton`. Service will be created once at `ServiceProvider` when it resolved (lazy). Can depend on dependencies
//! with anything lifetime. Cannot depend on services from forked `ServiceProvider` instances.
//! 3. `Instance`. Dependency was created outside of `ServiceProvider` and can be used by any other dependency.
//!
//! How to work:
//! 1. Declare your structs.
//! 2. Create constructors and add `#[inject]` macro on its.
//! 3. Create a `ServiceProvider` object.
//! 4. Add your services and dependencies using `ServiceProvider::add_*` methods.
//! 5. Fork `ServiceProvider` if you need to create local scope.
//! 6. Get service from provider using `.resolve()` method.
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
//! // derive macro can be used when all fields implement `Dependency` trait,
//! // but we do not recommend use it in production code
//! #[derive(Dependency)]
//! struct Controller {
//!     number_service: ConstService,
//! }
//!
//! // Create `ServiceProvider` struct that store itself all dependencies
//! let container = ServiceProvider::new()
//!     // Add dependency with `Singleton` lifetime. More about lifetimes see above.
//!     .add_transient::<ConstService>()
//!     // Add dependency with `Transient` lifetime. More about lifetimes see above.
//!     .add_transient::<Controller>();
//! // Fork `ServiceProvider`. It creates a new `ServiceProvider` which will have
//! // access to the dependencies from parent `ServiceProvider`.
//! let scope = container
//!     // .fork() method creates a local mutable scope with self parent immutable `ServiceProvider`.
//!     .fork()
//!     // Add an instance of `i32` that will be used when `ConstService` will be initialized.
//!     .add_instance(10);
//! let controller: Controller = scope.resolve();
//! assert_eq!(controller.number_service.number, 10);
//! ```

#![deny(unsafe_code)]

#[cfg(feature = "actix-support")]
mod actix_support;
mod container;
mod dependency;
mod get_dependencies;
mod index;
mod lifetime;
mod resolver;
mod service_provider;
pub mod dev;
mod dependency_factory;

#[cfg(feature = "actix-support")]
pub use actix_support::DIActixHandler;

pub use {
    dependency::Dependency,
    resolver::Resolver,
    service_provider::ServiceProvider,
    teloc_macros::{inject, Dependency},
};

#[doc(hidden)]
pub mod reexport {
    pub use {frunk, frunk::Hlist};
}
