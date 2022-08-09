use async_trait::async_trait;

/// The Route Trait
#[async_trait]
pub trait Route {
    type ExecutionResult;

    /// The route path
    fn path(&self) -> String;

    /// Specify Sub Routes
    ///
    /// For example, if the route is `/v2/metrics`, then the sub route is any string that is appended on like `metric_name` for `/v2/metrics/{metric_name}`.
    fn route(&mut self, route: String) -> Self;

    /// Adds a body to the request
    fn body<T: Into<reqwest::Body>>(&mut self, body: T) -> Self;

    /// Add a header to the request
    fn with_header(&mut self, key: &str, value: &str) -> Self;

    /// Add a list of headers to the request
    fn headers(&mut self, headers: Vec<(String, String)>) -> Self;

    /// Adds an api key to the request
    fn with_api_key(&mut self, key: String) -> Self;

    /// Adds an application key to the request
    fn with_application_key(&mut self, key: String) -> Self;

    /// Executes the api request
    fn execute(&self) -> Result<Self::ExecutionResult, reqwest::StatusCode>;
}
