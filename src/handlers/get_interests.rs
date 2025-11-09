use crate::db::get_interests;
use aws_sdk_dynamodb::types::AttributeValue;
use lambda_http::RequestExt;
use lambda_http::{tracing, Body, Error, Request, Response};
use std::collections::HashMap;

pub async fn handler(event: Request) -> Result<Response<Body>, Error> {
    let query_params = event.query_string_parameters();

    let limit = query_params
        .first("limit")
        .and_then(|l| l.parse::<i32>().ok())
        .filter(|&l| l > 0 && l <= 100);

    let exclusive_start_key = query_params.first("next_token").and_then(|token| {
        let decoded = base64::decode(token).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let parts: Vec<&str> = decoded_str.split('|').collect();

        if parts.len() != 2 {
            return None;
        }

        let mut key = HashMap::new();
        key.insert("id".to_string(), AttributeValue::S(parts[0].to_string()));
        key.insert("email".to_string(), AttributeValue::S(parts[1].to_string()));
        Some(key)
    });

    tracing::info!(
        "Getting interests with limit: {:?} and exclusive_start_key: {:?}",
        limit,
        exclusive_start_key
    );

    match get_interests(limit, exclusive_start_key).await {
        Ok(response) => {
            let body = serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string());

            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(body.into())
                .expect("Failed to render response"))
        }
        Err(e) => {
            tracing::error!("Failed to get interests. Error: {}", e);
            Ok(Response::builder()
                .status(500)
                .body("Internal Server Error".into())
                .expect("Failed to render response"))
        }
    }
}
