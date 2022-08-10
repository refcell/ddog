use std::str::FromStr;

use async_trait::async_trait;
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{prelude::routes::tr::Route, types};

/// Tags Metric Endpoint
///
/// ## Overvioew
///
/// The tags route on the metrics endpoint allows you to create a tag configuration for a given metric.
///
/// Endpoint Format: `/v2/metrics/{metric_name}/tags` [POST]
///
/// ```md
/// Create and define a list of queryable tag keys for an existing count/gauge/rate/distribution metric.
/// Optionally, include percentile aggregations on any distribution metric or configure custom aggregations on any count, rate, or gauge metric.
/// Can only be used with application keys of users with the Manage Tags for Metrics permission.
/// ```
///
/// ## Arguments
///
/// - metric_name [required]
///    - type: `string`
///    - description: The name of the metric.
///
/// ## Request
///
/// - Body Data (required)
///   - data [required] [type: object] - Object for a single metric to be configure tags on.
///     - attributes [type: object] - Object containing the definition of a metric tag configuration to be created.
///       - tags [type: string[]] - A list of tag keys that will be queryable for your metric.
///       - metric_type [type: enum] - The type of metric. Must be one of `count`, `gauge`, `rate`, or `distribution`.
///       - included_percentiles [type: boolean] - Toggle to include/exclude percentiles for a distribution metric. Defaults to false. Can only be applied to metrics that have a metric_type of distribution.
///       - aggregations [type: object[]] - A list of queryable aggregation combinations for a count, rate, or gauge metric.
///         - space [required] [type: enum] - A space aggregation for use in query. Allowed enum values: avg,max,min,sum.
///         - time [required] [type: enum] - A time aggregation for use in query. Allowed enum values: avg,count,max,min,sum
///     - id [required] [type: string] - The metric name for this resource.
///     - type [required] [type: enum] - The metric tag configuration resource type. Allowed enum values: manage_tags
///
/// ## Response
///
/// One of: [201, 400, 403, 409, 429]
///
/// #### Example
///
/// ```json
/// {
///     "data": {
///         "attributes": {
///             "aggregations": [
///                 { "space": "sum", "time": "sum" },
///             ],
///             "created_at": "2020-03-25T09:48:37.463835Z",
///             "include_percentiles": true,
///             "metric_type": "count",
///             "modified_at": "2020-03-25T09:48:37.463835Z",
///             "tags": [
///                 "app",
///                 "datacenter"
///             ]
///         },
///         "id": "test.metric.latency",
///         "type": "manage_tags"
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Tags {
    /// The metric name
    pub name: Option<String>,
    /// Request Headers
    pub headers: reqwest::header::HeaderMap,
    /// Request Body
    pub body: reqwest::Body,
}

/// A Tags Response
#[derive(Debug, Deserialize, Serialize)]
pub struct TagsResponse {
    /// Tags Response Data
    pub data: TagsResponseData,
}

/// The Tags Response Data
#[derive(Debug, Deserialize, Serialize)]
pub struct TagsResponseData {
    /// The metric type
    #[serde(rename = "type")]
    pub type_: MetricTag,
    /// The metric id
    pub id: String,
    /// The metric attributes
    pub attributes: TagsResponseAttributes,
}

/// The Tags Response Attributes
#[derive(Debug, Deserialize, Serialize)]
pub struct TagsResponseAttributes {
    /// The time of creation
    #[serde(with = "ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    /// The included percentiles
    pub included_percentiles: bool,
    /// The Metric's Type
    #[serde(rename = "type")]
    pub type_: MetricType,
    /// The time it was previously modified
    #[serde(with = "ts_seconds_option")]
    pub modified_at: Option<DateTime<Utc>>,
    /// Tags
    pub tags: Vec<String>,
    /// Aggregations
    pub aggregations: Vec<Aggregation>,
}

/// A Metric Aggregation
#[derive(Debug, Deserialize, Serialize)]
pub struct Aggregation {
    /// A space aggregation for use in query.
    pub space: SpaceEnum,
    /// A time aggregation for use in query.
    pub time: TimeEnum,
}

/// A Space Enum
#[derive(Debug, Deserialize, Serialize)]
pub enum SpaceEnum {
    /// Average Metric
    #[serde(rename = "avg")]
    Avg,
    /// Maximum Metric
    #[serde(rename = "max")]
    Max,
    /// Minimum Metric
    #[serde(rename = "min")]
    Min,
    /// Sum Metric
    #[serde(rename = "sum")]
    Sum,
}

/// A Time Enum
#[derive(Debug, Deserialize, Serialize)]
pub enum TimeEnum {
    /// Average Metric
    #[serde(rename = "avg")]
    Avg,
    /// Count Metric
    #[serde(rename = "count")]
    Count,
    /// Maximum Metric
    #[serde(rename = "max")]
    Max,
    /// Minimum Metric
    #[serde(rename = "min")]
    Min,
    /// Sum Metric
    #[serde(rename = "sum")]
    Sum,
}

/// The Metric Types
#[derive(Debug, Deserialize, Serialize)]
pub enum MetricType {
    /// A guage metric
    #[serde(rename = "gauge")]
    Gauge,
    /// A counter metric
    #[serde(rename = "count")]
    Count,
    /// A rate metric
    #[serde(rename = "rate")]
    Rate,
    /// A distribution metric
    #[serde(rename = "distribution")]
    Distribution,
}

/// A Metric Tag
#[derive(Debug, Deserialize, Serialize)]
pub enum MetricTag {
    /// Manage Tags Response
    #[serde(rename = "manage_tags")]
    ManageTags,
}

impl Default for Tags {
    fn default() -> Self {
        Self {
            name: None,
            headers: reqwest::header::HeaderMap::new(),
            body: reqwest::Body::from(""),
        }
    }
}

impl Tags {
    /// Instantiates a new Metric
    pub fn new(metric_name: &str) -> Self {
        tracing::info!(target: "/v2/metrics/{metric_name}/tags", "Tags Route Created");
        Self {
            name: Some(metric_name.to_string()),
            headers: reqwest::header::HeaderMap::new(),
            body: reqwest::Body::from(""),
        }
    }

    /// Set the Metric Name
    pub fn set_metric_name(mut self, metric_name: String) -> Self {
        tracing::info!(target: "/v2/metrics/{metric_name}/tags", "Tag Metric Name Set to {}", metric_name);
        self.name = Some(metric_name);
        self
    }

    /// Creates a target identifier for logging
    pub fn target() -> String {
        String::from("v2/metrics/{metric_name}/tags")
    }
}

#[async_trait]
impl Route<TagsResponse> for Tags {
    /// The route path
    fn path(&self) -> String {
        format!(
            "v2/metrics/{}/tags",
            self.name.as_ref().unwrap_or(&"".to_string())
        )
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
    fn route(mut self, route: String) -> Self {
        self.name = Some(route);
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
        Result<TagsResponse, Option<reqwest::Error>>,
    ) {
        // Validate the metric name
        if self.name.is_none() {
            return (reqwest::StatusCode::BAD_REQUEST, Err(None));
        };
        let url = format!("{}/{}", types::base::BASE_API_URL, self.path());
        tracing::info!(target: "/v2/metrics/{metric_name}/tags", "Sending Request to {}", url);

        let mut req_builder: reqwest::RequestBuilder = reqwest::Client::new().post(url);
        req_builder = req_builder.headers(self.headers.clone());
        req_builder = req_builder.body(self.body);

        let response = req_builder.send().await;

        match response {
            Ok(r) => {
                let status_code = r.status();
                if status_code != reqwest::StatusCode::from_u16(201).unwrap() {
                    tracing::error!(target: "/v2/metrics/{metric_name}/tags", "Received non-ok status code {:?}", status_code);
                    return (status_code, Err(None));
                }

                // Parse the response body as Json
                match r.json::<TagsResponse>().await {
                    Ok(s) => {
                        tracing::info!(target: "/v2/metrics/{metric_name}/tags", "Succesfully parsed response from tags route {:?}", s);
                        return (status_code, Ok(s));
                    }
                    Err(e) => {
                        tracing::error!(target: "/v2/metrics/{metric_name}/tags", "Failed to parse response as a TagsResponse with error {:?}", e);
                        return (reqwest::StatusCode::BAD_REQUEST, Err(Some(e)));
                    }
                }
            }
            Err(e) => {
                tracing::error!(target: "/v2/metrics/{metric_name}/tags", "Request failed with error {:?}", e);
                return (
                    e.status().unwrap_or(reqwest::StatusCode::BAD_REQUEST),
                    Err(Some(e)),
                );
            }
        }
    }
}
