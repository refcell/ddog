use std::str::FromStr;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::types;

/// Metrics Get Endpoint
///
/// ## Overview
///
/// Get the list of actively reporting metrics from a given time until now. This endpoint requires the `metrics_read` authorization [scope](https://docs.datadoghq.com/api/latest/scopes/#metrics).
///
/// Endpoint Format: `/v1/metrics` [GET]
///
///
/// ## Arguments
///
/// **Query Strings**
///
/// - from [required] [type: int] - Seconds since the Unix epoch.
/// - host [type: string] - Hostname for filtering the list of metrics returned. If set, metrics retrieved are those with the corresponding hostname tag.
/// - tag_filter [type: string] - Filter metrics that have been submitted with the given tags. Supports boolean and wildcard expressions. Cannot be combined with other filters.
///
///
/// ## Response
///
/// One of: [200, 400, 403, 429]
///
/// - from [required] [type: string] - Time when the metrics were active, seconds since the Unix epoch.
/// - metrics [type: string[]] - List of metric names.
///
///
/// #### Example
///
/// Below is an example of a response from the metrics get endpoint.
///
/// ```json
/// {
///     "from": "my.metric.name",
///     "metrics": [ "my.metric.name" ]
/// }
/// ```
#[derive(Debug)]
pub struct GetMetrics {
    /// Seconds since the Unix epoch.
    pub from: usize,
    /// Host name
    /// Used for filtering the list of metrics
    pub host: Option<String>,
    /// Tag filter
    /// Used for filtering the list of metrics
    pub tag_filter: Option<String>,
    /// The api version
    pub version: types::version::ApiVersion,
    /// Request Headers
    pub headers: reqwest::header::HeaderMap,
    /// Request Body
    pub body: reqwest::Body,
}

/// A GetMetrics Response
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct GetMetricsResponse {
    /// Time when the metrics were active, seconds since the Unix epoch.
    pub from: String,
    /// List of metric names.
    pub metrics: Vec<String>,
}

impl Default for GetMetrics {
    fn default() -> Self {
        Self {
            from: 0,
            host: None,
            tag_filter: None,
            version: types::version::ApiVersion::V2,
            headers: reqwest::header::HeaderMap::new(),
            body: reqwest::Body::from(""),
        }
    }
}

impl TryFrom<types::version::ApiVersion> for GetMetrics {
    type Error = &'static str;

    fn try_from(v: types::version::ApiVersion) -> Result<Self, Self::Error> {
        match v {
            types::version::ApiVersion::V1 => Ok(Self::default()),
            _ => Err("Unsupported API Version"),
        }
    }
}

impl GetMetrics {
    /// Instantiates a new Metric
    pub fn new(from: usize, host: Option<String>, tag_filter: Option<String>) -> Self {
        tracing::info!(target: "/v1/metrics", "Route Created");
        Self {
            from,
            host,
            tag_filter,
            version: types::version::ApiVersion::V1,
            headers: reqwest::header::HeaderMap::new(),
            body: reqwest::Body::from(""),
        }
    }

    /// Set the Metrics From
    pub fn set_from(mut self, from: usize) -> Self {
        tracing::info!(target: "/v1/metrics", "Metrics from set to {}", from);
        self.from = from;
        self
    }

    /// Set the Metrics Host
    pub fn set_host(mut self, host: String) -> Self {
        tracing::info!(target: "/v1/metrics", "Metrics host set to {}", host);
        self.host = Some(host);
        self
    }

    /// Set the Metrics Tag Filter
    pub fn set_tag_filter(mut self, tag_filter: String) -> Self {
        tracing::info!(target: "/v1/metrics", "Metrics tag filter set to {}", tag_filter);
        self.tag_filter = Some(tag_filter);
        self
    }

    /// Creates a target identifier for logging
    pub fn target() -> String {
        String::from("v1/metrics")
    }
}

#[async_trait]
impl types::route::Route<GetMetricsResponse> for GetMetrics {
    /// The route path
    fn path(&self) -> String {
        String::from("v1/metrics")
    }

    /// Manually sets the route path
    ///
    /// ## Usage
    ///
    /// It is not recommended to use this method directly, instead use the explicit sub routes functions.
    ///
    /// ## Warning
    ///
    /// Overrides any route path that may be already set.
    fn route(self, _: String) -> Self {
        self
    }

    /// Add a header to the request
    fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(
            reqwest::header::HeaderName::from_str(key).unwrap(),
            reqwest::header::HeaderValue::from_str(value).unwrap(),
        );
        self
    }

    /// Add a list of headers to the request
    fn headers(mut self, headers: Vec<(&str, &str)>) -> Self {
        for (key, value) in headers {
            self.headers.insert(
                reqwest::header::HeaderName::from_str(key).unwrap(),
                reqwest::header::HeaderValue::from_str(value).unwrap(),
            );
        }
        self
    }

    /// Adds an api key to the request
    fn with_api_key(mut self, key: &str) -> Self {
        self.headers.insert(
            reqwest::header::HeaderName::from_str("DD-API-KEY").unwrap(),
            reqwest::header::HeaderValue::from_str(key).unwrap(),
        );
        self
    }

    /// Adds an application key to the request
    fn with_application_key(mut self, key: &str) -> Self {
        self.headers.insert(
            reqwest::header::HeaderName::from_str("DD-APPLICATION-KEY").unwrap(),
            reqwest::header::HeaderValue::from_str(key).unwrap(),
        );
        self
    }

    /// Adds a body to the request
    fn body<T: Into<reqwest::Body>>(mut self, body: T) -> Self {
        self.body = body.into();
        self
    }

    /// Executes the api request
    async fn execute(
        self,
    ) -> (
        reqwest::StatusCode,
        Result<GetMetricsResponse, Option<reqwest::Error>>,
    ) {
        let url = format!(
            "{}/{}?from={}&host={}&tag_filter={}",
            types::base::BASE_API_URL,
            self.path(),
            self.from,
            self.host.unwrap_or_else(|| "".to_string()),
            self.tag_filter.unwrap_or_else(|| "".to_string())
        );
        tracing::info!(target: "/v1/metrics", "Sending Request to {}", url);

        let mut req_builder: reqwest::RequestBuilder = reqwest::Client::new().get(url);
        req_builder = req_builder.headers(self.headers.clone());

        let response = req_builder.send().await;

        match response {
            Ok(r) => {
                let status_code = r.status();
                if status_code != reqwest::StatusCode::from_u16(200).unwrap() {
                    tracing::error!(target: "/v1/metrics", "Received non-ok status code {:?}", status_code);
                    return (status_code, Err(None));
                }

                // Parse the response body as Json
                match r.json::<GetMetricsResponse>().await {
                    Ok(s) => {
                        tracing::info!(target: "/v1/metrics", "Succesfully parsed response from tags route {:?}", s);
                        return (status_code, Ok(s));
                    }
                    Err(e) => {
                        tracing::error!(target: "/v1/metrics", "Failed to parse response as a GetMetricsResponse with error {:?}", e);
                        return (reqwest::StatusCode::BAD_REQUEST, Err(Some(e)));
                    }
                }
            }
            Err(e) => {
                tracing::error!(target: "/v1/metrics", "Request failed with error {:?}", e);
                return (
                    e.status().unwrap_or(reqwest::StatusCode::BAD_REQUEST),
                    Err(Some(e)),
                );
            }
        }
    }
}
