use ddog::prelude::{v2::prelude::distribution::DistributionResponse, *};

extern crate dotenv;
use dotenv::dotenv;

#[test]
fn post_distribution_points() {
    // Read id and key from environment variables
    dotenv().ok();
    let api_key = dotenv::var("DD_API_KEY").unwrap();

    // Build and send the metrics post request
    let mut builder = builder::Builder::new();
    let mut builder_ref = builder.v2();
    if dotenv::var("TRACING_SUBSCRIBER")
        .map(|s| s == "true")
        .unwrap_or(false)
    {
        builder_ref = builder_ref.with_subscriber();
    }
    tokio_test::block_on(async {
        let (status, res) = builder_ref
            .post_distribution()
            .headers(vec![
                ("Accept", "application/json"),
                ("Content-Type", "application/json"),
                ("DD-API-KEY", &api_key),
            ])
            .body(
                r#"{
                    "series": [{
                        "host": "cloudflare_worker_1.jpeg",
                        "metric": "rpc_latency",
                        "points": [ [ 1636629071, [ 1.0, 2.0 ] ] ],
                        "tags": [ "gridfin:" ],
                        "type": "distribution"
                    }]
                }"#,
            )
            .execute()
            .await;

        tracing::info!(target: "/v2/distribution_points", "Response: {:?}", res);
        assert_eq!(status, 202);
        assert_eq!(
            res.unwrap(),
            DistributionResponse {
                status: String::from("ok")
            }
        );
    });
}

#[test]
fn post_distribution_invalid_json() {
    // Read id and key from environment variables
    dotenv().ok();
    let api_key = dotenv::var("DD_API_KEY").unwrap();

    // Build and send the metrics post request
    let mut builder = builder::Builder::new();
    let mut builder_ref = builder.v2();
    if dotenv::var("TRACING_SUBSCRIBER")
        .map(|s| s == "true")
        .unwrap_or(false)
    {
        builder_ref = builder_ref.with_subscriber();
    }
    tokio_test::block_on(async {
        let (status, res) = builder_ref
            .post_distribution()
            .headers(vec![
                ("Accept", "application/json"),
                ("Content-Type", "application/json"),
                ("DD-API-KEY", &api_key),
            ])
            .body(
                // NOTE THE EXTRA COMMA AT THE END OF THE FIRST SERIES OBJECT
                r#"{
                    "series": [{
                        "host": "cloudflare_worker_1.jpeg",
                        "metric": "rpc_latency",
                        "points": [ [ 1636629071, [ 1.0, 2.0 ] ] ],
                        "tags": [ "gridfin:" ],
                        "type": "distribution",
                    }]
                }"#,
            )
            .execute()
            .await;

        tracing::info!(target: "/v2/distribution_points", "Request Error: {:?}", res);
        assert_eq!(status, reqwest::StatusCode::from_u16(400).unwrap());
    });
}

#[test]
fn distribution_post_fails_invalid_api_key() {
    // Build and send the metrics post request
    let mut builder = builder::Builder::new();
    let mut builder_ref = builder.v2();
    if dotenv::var("TRACING_SUBSCRIBER")
        .map(|s| s == "true")
        .unwrap_or(false)
    {
        builder_ref = builder_ref.with_subscriber();
    }
    tokio_test::block_on(async {
        let (status, res) = builder_ref
            .post_distribution()
            .headers(vec![
                ("Accept", "application/json"),
                ("Content-Type", "application/json"),
                ("DD-API-KEY", "THIS_IS_NOT_A_VALID_API_KEY.JPEG"),
            ])
            .body(
                r#"{
                    "series": [{
                        "host": "cloudflare_worker_1.jpeg",
                        "metric": "rpc_latency",
                        "points": [ [ 1636629071, [ 1.0, 2.0 ] ] ],
                        "tags": [ "gridfin:" ],
                        "type": "distribution",
                    }]
                }"#,
            )
            .execute()
            .await;

        tracing::info!(target: "/v2/distribution_points", "Unauthorized Error: {:?}", res);
        assert_eq!(status, reqwest::StatusCode::from_u16(403).unwrap());
    });
}
