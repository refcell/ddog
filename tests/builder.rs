use ddog::{builder, types::routes::V2Routes};

use tokio_test;

#[test]
fn post_metrics_explicitly() {
    let builder = builder::Builder::new();
    tokio_test::block_on(async {
    match builder.v2()
        .metrics()
        .headers(vec![
            ("Accept".to_string(), "application/json".to_string()),
            ("Content-Type".to_string(), "application/json".to_string()),
        ])
        .post() {
            Ok() => {
                println!("Post Request Sent Successfully!");
            }
            Err(e) => {
                println!("Request Error: {:?}", e);
            }
    }});
}

// #[test]
// fn post_metrics_routes() {
//     let builder = builder::Builder::new();
//     match builder.v2()
//         .route(V2Routes::Metrics)
//         .with_header()
//         .with_header()
//         .with_api_key("api_key".to_string())
//         .with_application_key("application_key".to_string())
//         .post() {
//             Ok() => {
//                 println!("Post Request Sent Successfully!");
//             }
//             Err(e) => {
//                 println!("Request Error: {:?}", e);
//             }
//         }
// }

// #[test]
// fn post_metrics_default_route_config() {
//     let builder = builder::Builder::new();
//     match builder.v2()
//         .route(V2Routes::Metrics)
//         .with_api_key("api_key".to_string())
//         .with_application_key("application_key".to_string())
//         .post() {
//             Ok() => {
//                 println!("Post Request Sent Successfully!");
//             }
//             Err(e) => {
//                 println!("Request Error: {:?}", e);
//             }
//         }
// }