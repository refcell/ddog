use std::fmt;

/// API Versions
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApiVersion {
    /// API Version 1
    V1,
    /// API Version 2
    V2,
}

impl Default for ApiVersion {
    fn default() -> Self {
        Self::V2
    }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiVersion::V1 => write!(f, "v1"),
            ApiVersion::V2 => write!(f, "v2"),
        }
    }
}
