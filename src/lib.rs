//! A Minimal Datadog SDK in Pure Rust

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

/// The Query Builder
pub mod builder;

/// Api Types
pub mod types;

/// Re-export prelude modules
pub mod prelude {
    pub use super::{
        builder::*,
        types::*
    };
}
