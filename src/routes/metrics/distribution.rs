use std::str::FromStr;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::types;

/// Distribution Points Metrics Endpoint
///
/// ## Overview
///
/// The distribution points end-point allows you to post distribution data that can be graphed on Datadog’s dashboards.
///
/// Endpoint Format: `/v1/distribution_points` [POST]
///
/// ## Arguments
///
/// - Content-Encoding [header]
///    - type: `string`
///    - description: HTTP header used to compress the media-type.
///
/// ## Request
///
/// **Body Data (required)**
///
/// - series [required] [type: object[]] - A list of distribution points series to submit to Datadog.
///   - host [type: string] - The name of the host that produced the distribution point metric.
///   - metric [required] [type: string] - The name of the distribution points metric.
///   - points [required] [type: array[]] - Points relating to the distribution point metric. All points must be tuples with timestamp and a list of values (cannot be a string). Timestamps should be in POSIX time in seconds.
///   - tags [type: string[]] - A list of tags associated with the distribution point metric.
///   - type [type: enum] - The type of the distribution point. Allowed enum values: distribution
///
/// ## Response
///
/// One of: [202, 400, 403, 408, 413, 429]
///
/// - status [type: string] - The status of the intake payload.
///
/// #### Example
///
/// ```json
/// {
///     "status": "ok"
/// }
/// ```
#[derive(Debug)]
pub struct Distribution {
    /// Version
    pub version: types::version::ApiVersion,
    /// Request Headers
    pub headers: reqwest::header::HeaderMap,
    /// Request Body
    pub body: reqwest::Body,
}

/// A Distribution Points Response
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct DistributionResponse {
    /// Status of payload acceptance
    pub status: String,
}

impl Default for Distribution {
    fn default() -> Self {
        Self {
            version: types::version::ApiVersion::V1,
            headers: reqwest::header::HeaderMap::new(),
            body: reqwest::Body::from(""),
        }
    }
}

impl Distribution {
    /// Instantiates a new Metric
    pub fn new() -> Self {
        tracing::info!(target: "/v1/distribution_points", "Route Created");
        Self::default()
    }

    /// Creates a target identifier for logging
    pub fn target() -> String {
        String::from("v1/distribution_points")
    }
}

impl TryFrom<types::version::ApiVersion> for Distribution {
    type Error = &'static str;

    fn try_from(v: types::version::ApiVersion) -> Result<Self, Self::Error> {
        match v {
            types::version::ApiVersion::V1 => Ok(Self::default()),
            _ => Err("Unsupported API Version"),
        }
    }
}

#[async_trait]
impl types::route::Route<DistributionResponse> for Distribution {
    /// The route path
    fn path(&self) -> String {
        String::from("v1/distribution_points")
    }

    /// Route does nothing here since the route path is fixed
    fn route(self, _: std::string::String) -> Self {
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
        Result<DistributionResponse, Option<reqwest::Error>>,
    ) {
        // Create the request url
        let url = format!("{}/{}", types::base::BASE_API_URL, self.path());
        tracing::info!(target: "/v1/distribution_points", "Sending Request to {}", url);

        let mut req_builder: reqwest::RequestBuilder = reqwest::Client::new().post(url);
        req_builder = req_builder.headers(self.headers.clone());
        req_builder = req_builder.body(self.body);

        let response = req_builder.send().await;

        match response {
            Ok(r) => {
                let status_code = r.status();

                // Try to unwrap the response into json
                match r.json::<DistributionResponse>().await {
                    Ok(json_response) => {
                        tracing::info!(target: "/v1/distribution_points", "Deserialized Response into json: {:?}", json_response);
                        return (status_code, Ok(json_response));
                    }
                    Err(e) => {
                        tracing::error!(target: "/v1/distribution_points", "Failed to parse error response {:?}", e);
                        return (status_code, Err(Some(e)));
                    }
                }
            }
            Err(e) => {
                tracing::error!(target: "/v1/distribution_points", "Request failed with error {:?}", e);
                return (
                    e.status().unwrap_or(reqwest::StatusCode::BAD_REQUEST),
                    Err(Some(e)),
                );
            }
        }
    }
}
