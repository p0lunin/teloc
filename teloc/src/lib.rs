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
    teloc_macros::{inject, Teloc},
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
    [$x:expr, $($xs:expr),*] => {
        teloc::reexport::frunk::hlist::h_cons(
            teloc::container::Init::init($x),
            teloc::scopei![$($xs,)*]
        )
    }
}
