//! Datadog API Types

/// The Base Datadog API URL
pub mod base;

/// API Version Types
pub mod version;

/// Route Type
pub mod route;

/// Prelude to re-export common types
pub mod prelude {
    pub use super::{
        base::{self, *},
        route::{self, *},
        version::{self, *},
    };
}
