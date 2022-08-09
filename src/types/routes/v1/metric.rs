
/// Metric Endpoint
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Metric {
    /// The metric name
    pub name: String,
}

impl Metric {
    /// Initializes the metric endpoint
    pub fn new() -> Self {
        Self {
            name: String::new(),
        }
    }
}