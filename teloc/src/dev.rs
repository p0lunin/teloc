//! The module for the advanced usage.

pub use crate::{
    get_dependencies::GetDependencies,
    dependency::DependencyClone
};

pub mod container {
    //! Things needs to define your own containers.

    pub use crate::container::*;
}