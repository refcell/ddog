use std::str::FromStr;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{prelude::routes::tr::Route, types};

/// Series Metric Endpoint
///
/// ## Overvioew
///
/// The metrics end-point allows you to post time-series data that can be graphed on Datadog’s dashboards. The maximum payload size is 500 kilobytes (512000 bytes). Compressed payloads must have a decompressed size of less than 5 megabytes (5242880 bytes).
///
/// Host name is one of the resources in the Resources field.
///
/// Endpoint Format: `/v2/series` [POST]
///
/// ## Arguments
///
/// - Content-Encoding [header]
///    - type: `string`
///    - description: HTTP header used to compress the media-type.
///
/// ## Request
///
/// - Body Data (required)
///   - series [required] [type: object] - A list of time series to submit to Datadog.
///     - interval [type: int64] - If the type of the metric is rate or count, define the corresponding interval.
///     - metric [required] [type: string] - The name of the timeseries.
///     - source_type_name [type: string] - The source type name.
///     - tags [type: string[]] - A list of tags associated with the metric.
///     - type [type: enum] - The type of metric. The available types are 0 (unspecified), 1 (count), 2 (rate), and 3 (gauge). Allowed enum values: 0,1,2,3
///     - unit [type: string] - The unit of point value.
///     - metadata [type: object] - Metadata for the metric.
///       - origin [type: object] - Metric origin information.
///         - metric_type [type: int32] - The origin metric type code
///         - product [type: int32] - The origin product code
///         - service [type: int32] - The origin service code
///     - points [required] [type: object] - Points relating to a metric. All points must be objects with timestamp and a scalar value (cannot be a string). Timestamps should be in POSIX time in seconds, and cannot be more than ten minutes in the future or more than one hour in the past.
///       - timestamp [type: int64] - The timestamp should be in seconds and current. Current is defined as not more than 10 minutes in the future or more than 1 hour in the past.
///       - value [type: double] - The numeric value format should be a 64bit float gauge-type value.
///     - resources [type: object] - A list of resources to associate with this metric.
///       - name [type: string] - The name of the resource.
///       - type [type: string] - The type of the resource.
///
/// ## Response
///
/// One of: [202, 400, 403, 408, 413, 429]
///
/// #### Example
///
/// ```json
/// {
///     "errors": []
/// }
/// ```
#[derive(Debug)]
pub struct Series {
    /// Request Headers
    pub headers: reqwest::header::HeaderMap,
    /// Request Body
    pub body: reqwest::Body,
}

/// A Series Response
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct SeriesResponse {
    /// Series Response Data
    pub errors: Vec<serde_json::value::Value>,
}

impl Default for Series {
    fn default() -> Self {
        Self {
            headers: reqwest::header::HeaderMap::new(),
            body: reqwest::Body::from(""),
        }
    }
}

impl Series {
    /// Instantiates a new Metric
    pub fn new() -> Self {
        tracing::info!(target: "/v2/series", "Series Route Created");
        Self::default()
    }

    /// Creates a target identifier for logging
    pub fn target() -> String {
        String::from("v2/series")
    }
}

#[async_trait]
impl Route<SeriesResponse> for Series {
    /// The route path
    fn path(&self) -> String {
        String::from("v2/series")
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
        Result<SeriesResponse, Option<reqwest::Error>>,
    ) {
        // Create the request url
        let url = format!("{}/{}", types::base::BASE_API_URL, self.path());
        tracing::info!(target: "/v2/series", "Sending Request to {}", url);

        let mut req_builder: reqwest::RequestBuilder = reqwest::Client::new().post(url);
        req_builder = req_builder.headers(self.headers.clone());
        req_builder = req_builder.body(self.body);
        tracing::debug!(target: "/v2/series", "Request Builder: {:?}", req_builder);

        let response = req_builder.send().await;

        match response {
            Ok(r) => {
                let status_code = r.status();

                // Try to unwrap the response into json
                match r.json::<SeriesResponse>().await {
                    Ok(json_response) => {
                        tracing::info!(target: "/v2/series", "Deserialized Response into json: {:?}", json_response);
                        return (status_code, Ok(json_response));
                    }
                    Err(e) => {
                        tracing::error!(target: "/v2/series", "Failed to parse error response {:?}", e);
                        return (status_code, Err(Some(e)));
                    }
                }
            }
            Err(e) => {
                tracing::error!(target: "/v2/series", "Request failed with error {:?}", e);
                return (
                    e.status().unwrap_or(reqwest::StatusCode::BAD_REQUEST),
                    Err(Some(e)),
                );
            }
        }
    }
}
