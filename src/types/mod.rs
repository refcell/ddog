//! Datadog API Types

/// The Base Datadog API URL
pub mod base;

/// API Version Types
pub mod version;

/// Prelude to re-export common types
pub mod prelude {
    pub use super::{
        base::{self, *},
        version::{self, *},
    };
}
