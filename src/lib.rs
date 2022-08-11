#![doc=include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

/// The Query Builder
pub mod builder;

/// Api Routes
pub mod routes;

/// Api Types
pub mod types;

/// Re-export prelude modules
pub mod prelude {
    pub use super::{
        builder::{self, *},
        routes::{self, prelude::*},
        types::{self, prelude::*},
    };
}
