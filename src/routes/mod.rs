//! API Routes

/// Traits
pub mod tr;

/// API Version V1
pub mod v1;

/// API Version V2
pub mod v2;

/// Prelude to re-export common types
pub mod prelude {
    pub use super::{
        tr::{self, *},
        v1::{self, *},
        v2::{self, *},
    };
}
