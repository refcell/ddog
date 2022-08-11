//! A Minimal Datadog SDK written in Pure Rust
//!
//! ## Usage
//!
//! The simplest way to use the Datadog SDK is by using the [Builder](ddog::prelude::Builder).
//!
//! To create a new builder, you can instantiate one with the [new](ddog::prelude::Builder::new) method: `let mut builder = ddog::prelude::Builder::new();`.
//!
//! Then, to create a new query with a given endpoint, the Builder has explicit methods exposed for the specified endpoint.
//! For example, to post metrics series data to datadog, call the [post_series](ddog::prelude::Builder::post_series) method which returns a [Route](ddog::prelude::tr::Route) trait.
//!
//! ## Examples
//!
//! Below we show how to use [ddog](https://github.com/abigger87/ddog) to post metric series data to the Datadog API.
//!
//! Note: This request will not succeed since the `DD_API_KEY` environment variable is set to an invalid value in the request headers section.
//!
//! ```rust
//! use ddog::prelude::*;
//!
//! async {
//!     let mut builder = builder::Builder::new();
//!     let (status, res) = builder.v2()
//!         .post_series()
//!         .headers(vec![
//!             ("Accept", "application/json"),
//!             ("Content-Type", "application/json"),
//!             ("DD-API-KEY", "<api_key>"),
//!             ("DD-APPLICATION-KEY", "<application_key>"),
//!         ])
//!         .body(
//!             r#"{
//!                 "series": [{
//!                     "metric": "my.metric.name",
//!                     "type": 1,
//!                     "interval": 100000,
//!                     "unit": "count",
//!                     "tags": [ "my_tag:" ],
//!                     "source_type_name": "my_source_type",
//!                     "resources": [{
//!                         "name": "length",
//!                         "type": "time"
//!                     }],
//!                     "points": [
//!                         { "timestamp": 1660157680, "value": 10.0 },
//!                     ],
//!                     "metadata": {
//!                         "origin": {
//!                             "metric_type": 1,
//!                             "product": 1,
//!                             "service": 1
//!                         }
//!                     }
//!                 }]
//!             }"#
//!         )
//!         .execute().await;
//!
//!     // This should return a 403 status code now since the above API key is invalid.
//!     println!("Status Code: {:?}", status);
//!     println!("Response: {:?}", res);
//! };
//! ```

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
