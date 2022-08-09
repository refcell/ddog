//! Datadog API Types

/// The Base Datadog API URL
pub mod base;

/// API Version Types
pub mod version;

/// The API Routes
pub mod routes;

/// Prelude to re-export common types
pub mod prelude {
    pub use super::{
        version::*,
        base::*,
        routes::*
    };
}
