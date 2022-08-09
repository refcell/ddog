//! Datadog API Types

/// API Version Types
pub mod version;

/// Prelude to re-export common types
pub mod prelude {
    pub use super::{
        version::*,
    };
}
