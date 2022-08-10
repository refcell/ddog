use std::fmt::Debug;

use async_trait::async_trait;

/// The Route Trait
#[async_trait]
pub trait Route<T>
where
    T: Debug,
{
    /// Response for execute a route query
    // type ExecutionResult: impl Debug;

    /// The route path
    fn path(&self) -> String;

    /// Specify Sub Routes
    ///
    /// For example, if the route is `/v2/metrics`, then the sub route is any string that is appended on like `metric_name` for `/v2/metrics/{metric_name}`.
    #[deprecated(
        since = "0.0.2",
        note = "please use an explicit sub route method instead."
    )]
    fn route(self, route: String) -> Self;

    /// Adds a body to the request
    fn body<B: Into<reqwest::Body>>(self, body: B) -> Self;

    /// Add a header to the request
    fn with_header(self, key: &str, value: &str) -> Self;

    /// Add a list of headers to the request
    fn headers(self, headers: Vec<(&str, &str)>) -> Self;

    /// Adds an api key to the request
    fn with_api_key(self, key: &str) -> Self;

    /// Adds an application key to the request
    fn with_application_key(self, key: &str) -> Self;

    /// Executes the api request
    async fn execute(self) -> (reqwest::StatusCode, Result<T, Option<reqwest::Error>>);
}
