use std::str::FromStr;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds_option;

use crate::{prelude::routes::tr::Route, types};

/// Metric Endpoint
#[derive(Debug)]
pub struct Metric {
    /// The metric name as a subroute
    pub route: Option<String>,
    /// Request Headers
    pub headers: reqwest::header::HeaderMap,
    /// Request Body
    pub body: reqwest::Body,
}

/// A Metric Response
#[derive(Debug, Deserialize, Serialize)]
pub struct MetricResponse {
    /// Metric Response Data
    pub data: MetricResponseData,
}

/// The Metric Response Data
#[derive(Debug, Deserialize, Serialize)]
pub struct MetricResponseData {
    /// The metric type
    #[serde(rename="type")]
    pub type_: MetricTag,
    /// The metric id
    pub id: String,
    /// The metric attributes
    pub attributes: MetricResponseAttributes,
}

// #[serde_with::serde_as]
/// The Metric Response Attributes
#[derive(Debug, Deserialize, Serialize)]
pub struct MetricResponseAttributes {
    /// The time of creation
    #[serde(with = "ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    /// The included percentiles
    pub included_percentiles: bool,
    /// The Metric's Type
    #[serde(rename="type")]
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
    #[serde(rename="avg")]
    Avg,
    /// Maximum Metric
    #[serde(rename="max")]
    Max,
    /// Minimum Metric
    #[serde(rename="min")]
    Min,
    /// Sum Metric
    #[serde(rename="sum")]
    Sum
}

/// A Time Enum
#[derive(Debug, Deserialize, Serialize)]
pub enum TimeEnum {
    /// Average Metric
    #[serde(rename="avg")]
    Avg,
    /// Count Metric
    #[serde(rename="count")]
    Count,
    /// Maximum Metric
    #[serde(rename="max")]
    Max,
    /// Minimum Metric
    #[serde(rename="min")]
    Min,
    /// Sum Metric
    #[serde(rename="sum")]
    Sum
}

/// The Metric Types
#[derive(Debug, Deserialize, Serialize)]
pub enum MetricType {
    /// A guage metric
    #[serde(rename="gauge")]
    Gauge,
    /// A counter metric
    #[serde(rename="count")]
    Count,
    /// A rate metric
    #[serde(rename="rate")]
    Rate,
    /// A distribution metric
    #[serde(rename="distribution")]
    Distribution
}

/// A Metric Tag
#[derive(Debug, Deserialize, Serialize)]
pub enum MetricTag {
    /// Manage Tags Response
    #[serde(rename="manage_tags")]
    ManageTags,
}

impl Default for Metric {
    fn default() -> Self {
        Self {
            route: None,
            headers: reqwest::header::HeaderMap::new(),
            body: reqwest::Body::from(""),
        }
    }
}

impl Metric {
    /// Instantiates a new Metric
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a target identifier for logging
    pub fn target() -> String {
        String::from("v2/metrics")
    }
}

#[async_trait]
impl Route for Metric {
    /// Result returned by datadog for the metric endpoint query
    type ExecutionResult = MetricResponse;

    /// The route path
    fn path(&self) -> String {
        format!("/v2/metrics/{}", self.route.as_ref().unwrap_or(&"".to_string()))
    }

    /// Sub Routes
    fn route(mut self, route: String) -> Self {
        self.route = Some(route);
        self
    }

    /// Add a header to the request
    fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(reqwest::header::HeaderName::from_str(&key).unwrap(), reqwest::header::HeaderValue::from_str(&value).unwrap());
        self
    }

    /// Add a list of headers to the request
    fn headers(mut self, headers: Vec<(String, String)>) -> Self {
        for (key, value) in headers {
            self.headers.insert(reqwest::header::HeaderName::from_str(&key).unwrap(), reqwest::header::HeaderValue::from_str(&value).unwrap());
        }
        self
    }

    /// Adds an api key to the request
    fn with_api_key(mut self, key: String) -> Self {
        self.headers.insert(reqwest::header::HeaderName::from_str("DD-API-KEY").unwrap(), reqwest::header::HeaderValue::from_str(&key).unwrap());
        self
    }

    /// Adds an application key to the request
    fn with_application_key(mut self, key: String) -> Self {
        self.headers.insert(reqwest::header::HeaderName::from_str("DD-APPLICATION-KEY").unwrap(), reqwest::header::HeaderValue::from_str(&key).unwrap());
        self
    }

    /// Adds a body to the request
    fn body<T: Into<reqwest::Body>>(mut self, body: T) -> Self {
        self.body = body.into();
        self
    }

    /// Executes the api request
    async fn execute(self) -> Result<Self::ExecutionResult, reqwest::StatusCode> {
        // Validate the route
        if self.route.is_none() {
            return Err(reqwest::StatusCode::BAD_REQUEST);
        };
        let url = format!("{}/{}", types::base::BASE_API_URL, self.path());
        tracing::info!(target: "/v2/metrics", "Sending Request to {}", url);

        let mut req_builder: reqwest::RequestBuilder = reqwest::Client::new().post(url);
        req_builder = req_builder.headers(self.headers.clone());
        req_builder = req_builder.body(self.body);

        let response = req_builder.send().await;
        tracing::info!(target: "/v2/metrics", "Received response from target!");

        match &response {
            Ok(r) => {
                if r.status() != reqwest::StatusCode::OK {
                    return Err(r.status());
                }
            }
            Err(e) => {
                if e.is_status() {
                    return Err(e.status().unwrap());
                } else {
                    return Err(reqwest::StatusCode::BAD_REQUEST);
                }
            }
        }

        // Parse the response body as Json
        let content = response.unwrap().json::<MetricResponse>().await;

        match content {
            Ok(s) => Ok(s),
            Err(e) => {
                println!("{:?}", e);
                Err(reqwest::StatusCode::BAD_REQUEST)
            }
        }
    }
}

