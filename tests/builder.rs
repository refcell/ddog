use ddog::prelude::*;


/*

# Path parameters
export metric_name="dist.http.endpoint.request"
# Curl command
curl -X POST "https://api.datadoghq.com/api/v2/metrics/${metric_name}/tags" \
-H "Accept: application/json" \
-H "Content-Type: application/json" \
-H "DD-API-KEY: ${DD_API_KEY}" \
-H "DD-APPLICATION-KEY: ${DD_APP_KEY}" \
-d @- << EOF
{
    "data": {
        "type": "manage_tags",
        "id": "ExampleCreateatagconfigurationreturnsCreatedresponse",
        "attributes": {
        "tags": [
            "app",
            "datacenter"
        ],
        "metric_type": "gauge"
        }
    }
}
EOF

 */


#[test]
fn post_metrics_explicitly() {
    let mut builder = builder::Builder::new();
    tokio_test::block_on(async {
        match builder.v2()
            .metrics()
            .headers(vec![
                ("Accept".to_string(), "application/json".to_string()),
                ("Content-Type".to_string(), "application/json".to_string()),
            ])
            .execute().await {
                Ok(_) => {
                    println!("Post Request Sent Successfully!");
                }
                Err(e) => {
                    println!("Request Error: {:?}", e);
                }
        }
    });
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