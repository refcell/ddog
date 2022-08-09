
//! Internal query builder
//!
//! This module is public and therefore available for two reasons:
//! - So you can use it to code possible (although improbable) new API calls before this wrapper gets updated
//! - To test wrong calls, e.g.:
//! ```rust
//! #[tokio::test]
//! async fn error_404_not_found(){
//!     let not: Result<formats::RootAll, StatusCode> = query_builder::all("forcenotfound").await;
//!     assert_eq!(not.unwrap_err(),StatusCode::NOT_FOUND);
//! }
//! ```

use crate::{types::prelude::*, prelude::routes::tr::Route};

/// Builder for creating datadog API requests
///
/// ## Usage
///
/// Below we showcase using the ddog::Builder to post metrics to the datadog API.
///
/// ```rust
/// use ddog::prelude::Builder;
///
/// async {
///     let builder = builder::Builder::new();
///     match builder.v2()
///        .metrics()
///        .headers(vec![
///            ("Accept".to_string(), "application/json".to_string()),
///            ("Content-Type".to_string(), "application/json".to_string()),
///        ])
///        .post().await {
///            Ok() => {
///                println!("Post Request Sent Successfully!");
///            }
///            Err(e) => {
///                println!("Request Error: {:?}", e);
///            }
///    }
///    assert_eq!(cards.unwrap().get(0).unwrap().name.chars().collect::<Vec<char>>()[0], 'A');
/// };
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Builder {
    /// API Version
    pub version: ApiVersion,
    /// Request headers
    pub headers: Vec<(String, String)>
}

impl Builder {
    /// Initializes the query builder
    pub fn new() -> Self {
        Self {
            version: ApiVersion::Default(),
            headers: Vec::new(),
        }
    }

    /// Sets the api version to v1
    pub fn v1(&mut self) -> &mut Self {
        self.version = ApiVersion::V1;
        self
    }

    /// Sets the api version to v2
    pub fn v2(&mut self) -> &mut Self {
        self.version = ApiVersion::V2;
        self
    }

    /// Post a metric
    pub fn metrics(&self) -> impl Route {
        match self.version {
            ApiVersion::V1 => types::routes::v1::metric::Metric::new(),
            ApiVersion::V2 => types::routes::v2::metric::Metric::new(),
        }
    }
}
