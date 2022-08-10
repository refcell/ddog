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
///     let (status, res) = builder.v2()
///         .create_new_tag_config("my.metric.name")
///         .headers(vec![
///             ("Accept", "application/json"),
///             ("Content-Type", "application/json"),
///             ("DD-API-KEY", "<api_key>"),
///             ("DD-APPLICATION-KEY", "<application_key>"),
///         ])
///         .execute().await;
///
///     // This should return a 403 status code now since the above API key is invalid.
///     println!("Status Code: {:?}", status);
///     println!("Response: {:?}", res);
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

    /// Validate the json body
    ///
    /// Return some error if not valid.
    pub fn is_body_valid_json(body: &str) -> Option<serde_json::Error> {
        match serde_json::from_str::<serde_json::value::Value>(body) {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    /// Initialize a tracing subscriber
    pub fn with_subscriber(&mut self) -> &mut Self {
        let subscriber_builder = tracing_subscriber::fmt();
        let mut env_filter = tracing_subscriber::EnvFilter::from_default_env();
        env_filter = env_filter.add_directive(tracing::Level::DEBUG.into());
        if let Err(e) = subscriber_builder.with_env_filter(env_filter).try_init() {
            println!("Failed to initialize tracing!\nError: {:?}", e)
        }
        self
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
    // pub fn route<T>(&mut self, route: V2Routes) -> impl Route<T>
    // where
    //     crate::types::routes::v2::metrics::tags::Tags: crate::types::routes::tr::Route<T>,
    //     T: std::fmt::Debug,
    // {
    //     match self.version {
    //         ApiVersion::V2 => match route {
    //             // V2Routes::Metrics => self.metrics(),
    //             V2Routes::Metrics => panic!("Not implemented!"),
    //         },
    //         _ => panic!("Invalid Route Version \"V2Routes\" after calling builder.v1()"),
    //     }
    // }

    /// Create a new Tag Configuration
    pub fn create_new_tag_config<T>(&self, metric_name: &str) -> impl Route<T>
    where
        crate::types::routes::v2::metrics::tags::Tags: crate::types::routes::tr::Route<T>,
        T: std::fmt::Debug,
    {
        match self.version {
            ApiVersion::V2 => crate::builder::v2::metrics::tags::Tags::new(metric_name),
            _ => panic!("Unimplemented API Version"),
        }
    }

    /// Posts series data to the metrics endpoint
    pub fn post_series<T>(&self) -> impl Route<T>
    where
        crate::types::routes::v2::metrics::series::Series: crate::types::routes::tr::Route<T>,
        T: std::fmt::Debug,
    {
        match self.version {
            ApiVersion::V2 => crate::builder::v2::metrics::series::Series::new(),
            _ => panic!("Unimplemented API Version"),
        }
    }
}
