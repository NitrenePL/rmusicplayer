use serde_json;

fn main() {
    println!("{}", serde_json::json!({
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "serde",
                "json"
            ],
            "homepage": null
        }
    }).to_string());
}
