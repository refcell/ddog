use async_trait::async_trait;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use serde_with::TimestampMilliSeconds;
use serde_with::formats::Flexible;


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

/*

# Path parameters
export metric_name="dist.http.endpoint.request"
# Curl command
curl -X POST "https://api.datadoghq.com/api/v2/metrics/${metric_name}/tags" \
-H "Accept: application/json" \
-H "Content-Type: application/json" \
-H "DD-API-KEY: ${DD_API_KEY}" \
-H "DD-APPLICATION-KEY: ${DD_APP_KEY}" \
-d @- << EOF
{
    "data": {
        "type": "manage_tags",
        "id": "ExampleCreateatagconfigurationreturnsCreatedresponse",
        "attributes": {
        "tags": [
            "app",
            "datacenter"
        ],
        "metric_type": "gauge"
        }
    }
}
EOF

 */

/// A Metric Response
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct MetricResponse {
    /// Metric Response Data
    pub data: MetricResponseData,
}

/// The Metric Response Data
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct MetricResponseData {
    /// The metric type
    #[serde(rename="type")]
    pub type_: MetricTag,
    /// The metric id
    pub id: String,
    /// The metric attributes
    pub attributes: MetricResponseAttributes,
}

/// The Metric Response Attributes
#[serde_with::serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct MetricResponseAttributes {
    /// The time of creation
    #[serde(rename="created_at")]
    #[serde_as(as = "TimestampMilliSeconds<String, Flexible>")]
    pub createdAt: DateTime<Utc>,
    /// The included percentiles
    #[serde(rename="included_percentiles")]
    pub includedPercentiles: bool,
    /// The Metric's Type
    #[serde(rename="metric_type")]
    pub type_: MetricType,
    /// The time it was previously modified
    #[serde(rename="modified_at")]
    #[serde_as(as = "TimestampMilliSeconds<String, Flexible>")]
    pub modifiedAt: DateTime<Utc>,
    /// Tags
    pub tags: Vec<String>,
    /// Aggregations
    pub aggregations: Vec<Aggregation>,
}

/// A Metric Aggregation
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Aggregation {
    /// A space aggregation for use in query.
    pub space: SpaceEnum,
    /// A time aggregation for use in query.
    pub time: TimeEnum,
}

/// A Space Enum
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum MetricTag {
    /// Manage Tags Response
    #[serde(rename="manage_tags")]
    ManageTags,
}

impl Metric {
    /// Instantiates a new Metric
    pub fn new(name: String) -> Self {
        Self {
            route: None,
            headers: reqwest::header::HeaderMap::new(),
            body: reqwest::Body::empty(),
        }
    }

    /// Creates a target identifier for logging
    pub fn target() -> String {
        String::from("v2/metrics")
    }
}

#[async_trait]
impl Route for Metric {
    type ExecutionResult = ();

    /// The route path
    fn path(&self) -> String {
        format!("/v2/metrics/{}", self.name)
    }

    /// Sub Routes
    fn route(&mut self, route: String) -> Self {
        self.route = Some(route);
        self
    }

    /// Add a header to the request
    fn with_header(&mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Add a list of headers to the request
    fn headers(&mut self, headers: Vec<(String, String)>) -> Self {
        for (key, value) in headers {
            self.headers.insert(key, value);
        }
        self
    }

    /// Adds an api key to the request
    fn with_api_key(&mut self, key: String) -> Self {
        self.headers.push(("DD-API-KEY".to_string(), key));
        self
    }

    /// Adds an application key to the request
    fn with_application_key(&mut self, key: String) -> Self {
        self.headers.push(("DD-APPLICATION-KEY".to_string(), key));
        self
    }

    /// Adds a body to the request
    fn body<T: Into<reqwest::Body>>(&mut self, body: T) -> Self {
        self.body = body.into();
        self
    }

    /// Executes the api request
    async fn execute(&self) -> Result<Self::ExecutionResult, reqwest::StatusCode> {
        // Validate the route
        let route = if let Some(r) = self.route { r } else {
            return Err(reqwest::StatusCode::BAD_REQUEST);
        };
        let url = format!("{}/{}", types::base::BASE_API_URL, self.path());
        tracing::info!(target: "/v2/metrics", "Sending Request to {}", url);

        let mut req_builder: reqwest::RequestBuilder = reqwest::Client::new().post(url);
        req_builder = req_builder.headers(self.headers);
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

