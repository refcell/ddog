//! Environment Variable Config

/// Environment Variable Config for Datadog Api Requests
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct EnvConfig {
    /// `DD_API_KEY` environment variable
    pub api_key: Option<String>,
    /// `DD_APPLICATION_KEY` environment variable
    pub application_key: Option<String>,
}

impl EnvConfig {
    /// Instantiates a new EnvConfig
    pub fn new(keys: Option<Vec<(String, String)>>) -> Self {
        match keys {
            Some(key_vec) => Self::from(key_vec),
            None => Self::default(),
        }
    }
}

/// Attempts to transform a list of string tuples into an [EnvConfig](ddog::types::env::EnvConfig).
///
/// A successful conversion would be the following:
///
/// ```rust
/// use ddog::types::env::*;
///
/// let env_config = EnvConfig::from(Vec<("DD_API_KEY", "<api_key>"), ("DD_APPLICATION_KEY", "<application_key>")>);
/// assert_eq!(env_config.api_key, Some("<api_key>".to_string()));
/// assert_eq!(env_config.application_key, Some("<application_key>".to_string()));
impl From<Vec<(String, String)>> for EnvConfig {
    fn from(headers: Vec<(String, String)>) -> Self {
        let mut api_key = None;
        let mut application_key = None;
        for (key, value) in headers {
            match key.as_str() {
                "DD_API_KEY" => api_key = Some(value),
                "DD_APPLICATION_KEY" => application_key = Some(value),
                _ => (),
            }
        }
        Self {
            api_key,
            application_key,
        }
    }
}

/// Attempts to transform a list of strings into an [EnvConfig](ddog::types::env::EnvConfig).
///
/// Each string in the list is expected to be in key-value for with a '=' character as a separator. For example: `key=value`.
///
/// A successful conversion would be the following:
///
/// ```rust
/// use ddog::types::env::*;
///
/// let env_config = EnvConfig::from(Vec<"DD_API_KEY=<api_key>", "DD_APPLICATION_KEY=<application_key>">);
/// assert_eq!(env_config.api_key, Some("<api_key>".to_string()));
/// assert_eq!(env_config.application_key, Some("<application_key>".to_string()));
impl From<Vec<String>> for EnvConfig {
    fn from(headers: Vec<String>) -> Self {
        let mut api_key = None;
        let mut application_key = None;
        for potential_kv in headers {
            let split_kv = potential_kv.trim().split('=').collect::<Vec<&str>>();
            let key = *split_kv.get(0).unwrap_or(&"");
            match key {
                "DD_API_KEY" => api_key = Some(split_kv.get(1).map(|s| String::from(*s)).unwrap_or("".to_string())),
                "DD_APPLICATION_KEY" => application_key = Some(split_kv.get(1).map(|s| String::from(*s)).unwrap_or("".to_string())),
                _ => (),
            }
        }
        Self {
            api_key,
            application_key,
        }
    }
}

/// Attempts to transform a string into an [EnvConfig](ddog::types::env::EnvConfig).
///
/// The string is parsed as it is expected to be in key-value, comma-separated form ie: `key=value,key=value,key=value`.
///
/// A successful conversion would be the following:
///
/// ```rust
/// use ddog::types::env::*;
///
/// let env_config = EnvConfig::from("DD_API_KEY=<api_key>,DD_APPLICATION_KEY=<application_key>");
/// assert_eq!(env_config.api_key, Some("<api_key>".to_string()));
/// assert_eq!(env_config.application_key, Some("<application_key>".to_string()));
/// ```
impl From<String> for EnvConfig {
    fn from(potential_envs: String) -> Self {
        let mut api_key = None;
        let mut application_key = None;
        for potential_kv in potential_envs.split(',') {
            let split_kv = potential_kv.trim().split('=').collect::<Vec<&str>>();
            let key = *split_kv.get(0).unwrap_or(&"");
            match key {
                "DD_API_KEY" => api_key = Some(split_kv.get(1).map(|s| String::from(*s)).unwrap_or("".to_string())),
                "DD_APPLICATION_KEY" => application_key = Some(split_kv.get(1).map(|s| String::from(*s)).unwrap_or("".to_string())),
                _ => (),
            }
        }
        Self {
            api_key,
            application_key,
        }
    }
}
