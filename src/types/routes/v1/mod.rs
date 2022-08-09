
/// Metrics endpoint
pub mod metric;

/// Re-export common modules
pub mod prelude {
    pub use super::{
        metric::*,
    };
}
