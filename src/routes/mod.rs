//! API Routes

/// Metrics endpoints
pub mod metrics;

/// Prelude to re-export common types
pub mod prelude {
    pub use super::metrics::{self, *};
}
