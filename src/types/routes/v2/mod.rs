/// Metrics endpoints
pub mod metrics;

/// Re-export common modules
pub mod prelude {
    pub use super::metrics::*;
}
