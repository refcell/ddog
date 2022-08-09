use std::fmt;

/// Api V2 routes available on the datadog api
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
