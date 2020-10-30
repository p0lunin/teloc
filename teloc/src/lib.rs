pub mod container_elem;
mod dependency;
mod get;
mod get_dependencies;
pub mod scope;
mod service_provider;

pub use {
    dependency::Dependency,
    frunk,
    frunk::Hlist,
    get::Get,
    get_dependencies::GetDependencies,
    scope::Scope,
    service_provider::ServiceProvider,
    teloc_macros::{inject, Teloc},
};

#[macro_export]
macro_rules! scopei {
    [] => { teloc::frunk::HNil };
    [$x:expr, $($xs:expr),*] => {
        teloc::frunk::hlist::h_cons(
            teloc::container_elem::Init::init($x),
            teloc::scopei![$($xs,)*]
        )
    }
}
