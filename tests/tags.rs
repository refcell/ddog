use ddog::prelude::*;

extern crate dotenv;

use dotenv::dotenv;

#[test]
fn create_new_tag_explicitly() {
    dotenv().ok();

    // Read id and key from environment variables
    let api_key = dotenv::var("DD_API_KEY").unwrap();
    let application_key = dotenv::var("DD_APPLICATION_KEY").unwrap();

    // Build and send the metrics post request
    let mut builder = builder::Builder::new();
    let mut builder_ref = builder.v2();
    if dotenv::var("TRACING_SUBSCRIBER").unwrap() == "true" {
        builder_ref = builder_ref.with_subscriber();
    }
    tokio_test::block_on(async {
        let (status, res) = builder_ref
            .create_new_tag_config("rpc_latency")
            .headers(vec![
                ("Accept", "application/json"),
                ("Content-Type", "application/json"),
                ("DD-API-KEY", &api_key),
                ("DD-APPLICATION-KEY", &application_key),
            ])
            .body(
                r#"{
                    "data": {
                        "type": "manage_tags",
                        "id": "CreateGridfinTag2",
                        "attributes": {
                            "tags": [ "gridfin" ],
                            "metric_type": "count"
                        }
                    }
                }"#,
            )
            .execute()
            .await;

        tracing::info!(target: "v2/metrics/{}/tags", "Response: {:?}", res);
        tracing::info!(target: "v2/metrics/{}/tags", "Response: {:?}", status);
        // assert_eq!(status, 201);
        // TODO: make api key work
    });
}

#[test]
fn create_new_tag_forbidden_key() {
    // Build and send the metrics post request
    let mut builder = builder::Builder::new();
    let mut builder_ref = builder.v2();
    if dotenv::var("TRACING_SUBSCRIBER").unwrap() == "true" {
        builder_ref = builder_ref.with_subscriber();
    }
    tokio_test::block_on(async {
        let (status, res) = builder_ref
            .create_new_tag_config("rpc_latency")
            .headers(vec![
                ("Accept", "application/json"),
                ("Content-Type", "application/json"),
                ("DD-API-KEY", "RANDOM_API_KEY.JPEG"),
                ("DD-APPLICATION-KEY", "RANDOM_APPLICATION_KEY.JPEG"),
            ])
            .body(
                r#"{
                    "data": {
                        "type": "manage_tags",
                        "id": "CreateGridfinTag2",
                        "attributes": {
                            "tags": [ "gridfin" ],
                            "metric_type": "count"
                        }
                    }
                }"#,
            )
            .execute()
            .await;

        tracing::info!(target: "v2/series", "Response: {:?}", res);
        assert_eq!(status, 403);
    });
}

// #[test]
// fn post_metrics_routes() {
//     let mut builder = builder::Builder::new();
//     tokio_test::block_on(async {
//         match builder
//             .v2()
//             .route(V2Routes::Metrics)
//             .with_header("Accept", "application/json")
//             .with_header("Content-Type", "application/json")
//             .with_api_key("api_key")
//             .with_application_key("application_key")
//             .body(
//                 r#"{
//                     "data": {
//                         "type": "manage_tags",
//                         "id": "ExampleCreateatagconfigurationreturnsCreatedresponse",
//                         "attributes": {
//                         "tags": [
//                             "app",
//                             "datacenter"
//                         ],
//                         "metric_type": "gauge"
//                         }
//                     }
//                 }"#,
//             )
//             .execute()
//             .await
//         {
//             Ok(_) => {
//                 println!("Post Request Sent Successfully!");
//             }
//             Err(e) => {
//                 println!("Request Error: {:?}", e);
//             }
//         }
//     });
// }

// #[test]
// fn post_metrics_default_route_config() {
//     let mut builder = builder::Builder::new();
//     tokio_test::block_on(async {
//         match builder
//             .v2()
//             .route(V2Routes::Metrics)
//             .with_api_key("api_key")
//             .with_application_key("application_key")
//             .body(
//                 r#"{
//                     "data": {
//                         "type": "manage_tags",
//                         "id": "ExampleCreateatagconfigurationreturnsCreatedresponse",
//                         "attributes": {
//                         "tags": [
//                             "app",
//                             "datacenter"
//                         ],
//                         "metric_type": "gauge"
//                         }
//                     }
//                 }"#,
//             )
//             .execute()
//             .await
//         {
//             Ok(_) => {
//                 println!("Post Request Sent Successfully!");
//             }
//             Err(e) => {
//                 println!("Request Error: {:?}", e);
//             }
//         }
//     });
// }
