use ddog::prelude::Builder;

#[test]
fn test_json_validation() {
    let json_string = r#"{
        "series": [{
            "metric": "rpc_latency",
            "type": 1,
            "source_type_name": "gridfin_worker",
            "resources": [{
                "name": "latency",
                "type": "time"
            }],
            "points": [
                { "timestamp": 1660157680, "value": 10.0 },
                { "timestamp": 1660157680, "value": 5.0 },
                { "timestamp": 1660157680, "value": 15.0 }
            ]
        }]
    }"#;

    match Builder::is_body_valid_json(json_string) {
        Some(e) => panic!("Failed to validate correct json with error {:?}", e),
        None => println!("Successfully validated json body string!"),
    }
}

#[test]
fn test_invalide_json() {
    let invalid_json_string = r#"{
        "series": [{
            "metric": "rpc_latency",
            "type": 1,
            "source_type_name": "gridfin_worker",
            "resources": [{
                "name": "latency",
                "type": "time"
            }],
            "points": [
                { "timestamp": 1660157680, "value": 10.0 },
                { "timestamp": 1660157680, "value": 5.0 },
                { "timestamp": 1660157680, "value": 15.0 },
            ]
        }]
    }"#;

    match Builder::is_body_valid_json(invalid_json_string) {
        Some(e) => println!("Correctly invalidated json with error {:?}", e),
        None => panic!("This json is invalid!"),
    }
}
