use crate::models::Interest;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};
use lambda_http::tracing;
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
