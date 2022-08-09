//! Internal query builder

use crate::{prelude::routes::tr::Route, types::prelude::*};

/// Builder for creating datadog API requests
///
/// ## Usage
///
/// Below we showcase using the ddog::Builder to post metrics to the datadog API.
///
/// ```rust
/// use ddog::prelude::*;
///
/// async {
///     let mut builder = builder::Builder::new();
///     match builder.v2()
///         .metrics()
///         .route("my.metric.name".to_string())
///         .headers(vec![
///             ("Accept", "application/json"),
///             ("Content-Type", "application/json"),
///         ])
///         .execute().await {
///             Ok(_) => {
///                 println!("Post Request Sent Successfully!");
///             }
///             Err(e) => {
///                 panic!("Request Error: {:?}", e);
///             }
///     }
/// };
/// ```
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Builder {
    /// API Version
    pub version: ApiVersion,
    /// Request headers
    pub headers: Vec<(String, String)>,
}

impl Builder {
    /// Initializes the query builder
    pub fn new() -> Self {
        Self::default()
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

    /// Creates the respective route for the given V2 Route types
    pub fn route(&mut self, route: V2Routes) -> impl Route {
        match self.version {
            ApiVersion::V2 => match route {
                V2Routes::Metrics => self.metrics(),
            },
            _ => panic!("Invalid Route Version \"V2Routes\" after calling builder.v1()"),
        }
    }

    /// Post a metric
    pub fn metrics(&self) -> impl Route {
        match self.version {
            // ApiVersion::V1 => crate::builder::v1::prelude::Metric::new(),
            ApiVersion::V2 => crate::builder::v2::prelude::Metric::new(),
            _ => panic!("Unimplemented API Version"),
        }
    }
}
