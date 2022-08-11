//! Api Client

use crate::{routes, types};

/// Client for Lazy API Usage
///
/// ## Getting Started
///
/// The Client is a wrapper for creating requests to the Datadog API.
///
/// It features a configurable HTTP Client and uses the exposed [Builder](ddog::prelude::Builder) as a middleware for request construction.
///
/// Below we demonstrate using the ddog [Client](ddog::prelude::Client) to post metrics to the Datadog API.
///
/// ```rust
/// use ddog::prelude::Client as DdogClient;
///
/// // Execute queries in an async context
/// async {
///     // Instantiate and configure the client
///     let mut client = DdogClient {
///         name: String::from("Example Datadog Client"),
///         host: String::from("imahost.com"),
///         port: 8196,
///         logger: 
///         tracing: true,
///         env:    
    /// Datadog agent port, defaults to `8196`.
    pub port: String,
    /// Optional Logging Config to also set this tracer as the main logger
    pub logging_config: Option<LoggingConfig>,
    /// APM Config to set up APM Analytics (default is to disable)
    pub apm_config: ApmConfig,
    /// Turn on tracing
    pub enable_tracing: bool,
    /// Number of threads to send the HTTP messages to the Datadog agent
    pub num_client_send_threads: u32,
///         Some(vec![
///             ("DD-API-KEY", "<api_key>"),
///             ("DD-APPLICATION-KEY", "<application_key>"),
///         ])
///     };
///     client.set_client(reqwest::Client::new());
///     client.set_
/// 
///     // Create a new metric
///     
/// 
/// 
///    let (status, res) = client.v2()
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
    pub version: types::version::ApiVersion,
    /// Request headers
    pub headers: Vec<(String, String)>,
}

