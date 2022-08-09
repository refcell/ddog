use std::fmt;

/// API Version V1
pub mod v1;
/// API Version V2
pub mod v2;
/// Traits
pub mod tr;

/// Api V2 routes available on the datadog api
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum V2Routes {
    /// Metrics
    Metrics,
}

impl fmt::Display for V2Routes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            V2Routes::Metrics => write!(f, "metrics"),
        }
    }
}

/// Api V1 routes available on the datadog api
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum V1Routes {
    /// Metrics
    Metrics,
}

impl fmt::Display for V1Routes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            V1Routes::Metrics => write!(f, "metrics"),
        }
    }
}
