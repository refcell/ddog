use ddog::prelude::*;

extern crate dotenv;
use dotenv::dotenv;

#[test]
fn get_metrics() {
    // Read id and key from environment variables
    dotenv().ok();
    let api_key = dotenv::var("DD_API_KEY").unwrap();
    let application_key = dotenv::var("DD_APPLICATION_KEY").unwrap();

    // Build and send the metrics post request
    let mut builder = builder::Builder::new();
    let mut builder_ref = builder.v1();
    if dotenv::var("TRACING_SUBSCRIBER")
        .map(|s| s == "true")
        .unwrap_or(false)
    {
        builder_ref = builder_ref.with_subscriber();
    }
    tokio_test::block_on(async {
        let (status, res) = builder_ref
            .get_metrics(0, None, Some("metric_name:rpc_latency".to_string()))
            .headers(vec![
                ("Accept", "application/json"),
                ("Content-Type", "application/json"),
                ("DD-API-KEY", &api_key),
                ("DD_APPLICATION_KEY", &application_key),
            ])
            .execute()
            .await;

        tracing::info!(target: "/v1/metrics", "Response: {:?}", res);
        assert_eq!(status, 200);
        let unwrapped_response = res.unwrap();
        assert_eq!(unwrapped_response.from, String::from("0"));
        assert_ne!(unwrapped_response.metrics.len(), 0);
    });
}
