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
///        .metrics()
///        .headers(vec![
///            ("Accept", "application/json"),
///            ("Content-Type", "application/json"),
///        ])
///        .execute().await {
///            Ok(_) => {
///                 println!("Post Request Sent Successfully!");
///            }
///            Err(e) => {
///                 panic!("Request Error: {:?}", e);
///            }
///    }
///    // assert_eq!(cards.unwrap().get(0).unwrap().name.chars().collect::<Vec<char>>()[0], 'A');
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

    /// Post a metric
    pub fn metrics(&self) -> impl Route {
        match self.version {
            // ApiVersion::V1 => crate::builder::v1::prelude::Metric::new(),
            ApiVersion::V2 => crate::builder::v2::prelude::Metric::new(),
            _ => panic!("Unimplemented API Version"),
        }
    }
}
