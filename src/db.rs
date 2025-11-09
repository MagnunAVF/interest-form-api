use crate::models::{Interest, PaginatedResponse};
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};
use lambda_http::tracing;
use std::collections::HashMap;
use std::env;

async fn get_dynamodb_client() -> Result<Client, Error> {
    let config = crate::aws::get_aws_config();

    let client = if env::var("ENV").unwrap_or_default() == "dev" {
        let config = aws_sdk_dynamodb::config::Builder::from(config)
            .endpoint_url("http://localhost:8000")
            .build();
        Client::from_conf(config)
    } else {
        Client::new(config)
    };

    Ok(client)
}

pub async fn add_interest(interest: Interest) -> Result<(), Error> {
    tracing::info!("[DB] Adding interest: {:?}", interest);

    let table_name =
        env::var("INTERESTS_TABLE_NAME").expect("env var INTERESTS_TABLE_NAME must be set");
    let client = get_dynamodb_client().await?;

    let request = client
        .put_item()
        .table_name(table_name)
        .item("id", AttributeValue::S(interest.id))
        .item("name", AttributeValue::S(interest.name))
        .item("email", AttributeValue::S(interest.email))
        .item("created_at", AttributeValue::S(interest.created_at));

    request.send().await?;

    Ok(())
}

pub async fn get_interests(
    limit: Option<i32>,
    exclusive_start_key: Option<HashMap<String, AttributeValue>>,
) -> Result<PaginatedResponse, Error> {
    tracing::info!("[DB] Getting interests with limit: {:?}", limit);

    let table_name =
        env::var("INTERESTS_TABLE_NAME").expect("env var INTERESTS_TABLE_NAME must be set");
    let client = get_dynamodb_client().await?;

    let mut request = client
        .scan()
        .table_name(table_name)
        .limit(limit.unwrap_or(10));

    if let Some(start_key) = exclusive_start_key {
        request = request.set_exclusive_start_key(Some(start_key));
    }

    let response = request.send().await?;
    let items = response
        .items()
        .iter()
        .filter_map(|item| {
            let id = item.get("id")?.as_s().ok()?.to_string();
            let name = item.get("name")?.as_s().ok()?.to_string();
            let email = item.get("email")?.as_s().ok()?.to_string();
            let created_at = item.get("created_at")?.as_s().ok()?.to_string();

            Some(Interest {
                id,
                name,
                email,
                created_at,
            })
        })
        .collect::<Vec<Interest>>();

    let next_token = response.last_evaluated_key().and_then(|key| {
        let id = key.get("id")?.as_s().ok()?;
        let email = key.get("email")?.as_s().ok()?;
        let token = format!("{}|{}", id, email);
        Some(base64::encode(token))
    });

    let count = items.len();

    tracing::info!("[DB] Retrieved {} interests", count);

    Ok(PaginatedResponse {
        items,
        next_token,
        count,
    })
}
