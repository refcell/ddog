
//! Internal query builder
//!
//! This module is public and therefore available for two reasons:
//! - So you can use it to code possible (although improbable) new API calls before this wrapper gets updated
//! - To test wrong calls, e.g.:
//! ```rust
//! #[tokio::test]
//! async fn error_404_not_found(){
//!     let not: Result<formats::RootAll, StatusCode> = query_builder::all("forcenotfound").await;
//!     assert_eq!(not.unwrap_err(),StatusCode::NOT_FOUND);
//! }
//! ```

use crate::types::prelude::*;

use reqwest::StatusCode;
use serde::de::DeserializeOwned;

/// The base datadog api url
const BASE_API_URL: &str = "https://api.datadoghq.com/api/";


// Build the URL for all calls
async fn build<T>(url: String) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await;

    match &response {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status());
            }
        }
        Err(e) => {
            if e.is_status() {
                return Err(e.status().unwrap());
            } else {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    // Parse the response body as Json
    let content = response.unwrap().json::<T>().await;

    match content {
        Ok(s) => Ok(s),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}