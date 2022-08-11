//! Metrics Endpoints
//!
//! ## Overvioew
//!
//! The metrics endpoints, as described in the [Datadog Documentation](https://docs.datadoghq.com/api/latest/metrics/?code-lang=curl):
//!
//! ```md
//!     - Post metrics data so it can be graphed on Datadog’s dashboards
//!     - Query metrics from any time period
//!     - Modify tag configurations for metrics
//!     - View tags and volumes for metrics
//! ```
//!
//! Further, note that a graph can only contain a set number of points and as the timeframe over which a metric is viewed increases, aggregation between points occurs to stay below that set number.

/// `v2/metrics/{metric_name}/tags` Endpoint [POST]
pub mod tags;

/// `v2/series` Endpoint [POST]
pub mod series;

/// `v2/distribution_points` Endpoint [POST]
pub mod distribution;

/// Re-exported prelude of all metrics-related endpoints
pub mod prelude {
    pub use super::{
        distribution::{self, *},
        series::{self, *},
        tags::{self, *},
    };
}
