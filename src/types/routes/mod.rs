use std::fmt;

pub mod v1;
pub mod v2;
pub mod tr;

/// Api V2 routes available on the datadog api
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum V2Routes {
    /// Metrics
    Metrics(v2::metric::Metric),
}

impl fmt::Display for V2Routes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            V2Routes::Metrics(_) => write!(f, "metrics"),
        }
    }
}

/// Api V1 routes available on the datadog api
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum V1Routes {
    /// Metrics
    Metrics(v1::metric::Metric),
}

impl fmt::Display for V1Routes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            V1Routes::Metrics(_) => write!(f, "metrics"),
        }
    }
}
